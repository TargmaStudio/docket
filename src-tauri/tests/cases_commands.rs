// Integration tests for the case Tauri commands.
//
// Unlike src-tauri/src/cases.rs's unit tests (which call the private *_impl
// functions directly against an i64 id), these exercise the exact public
// commands the frontend invokes: State<Db> extraction, string id parsing,
// and the Result<T, String> error mapping.

use docket_lib::cases::{self, Db, NewCaseInput};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

fn mock_app_with_db(conn: Connection) -> tauri::App<tauri::test::MockRuntime> {
    cases::init_schema(&conn).unwrap();

    let app = tauri::test::mock_app();
    app.manage(Db(Mutex::new(conn)));
    app
}

fn in_memory_app() -> tauri::App<tauri::test::MockRuntime> {
    mock_app_with_db(Connection::open_in_memory().unwrap())
}

fn sample_input(patient_name: &str) -> NewCaseInput {
    NewCaseInput {
        patient_name: patient_name.to_string(),
        date_of_birth: "1990-01-01".to_string(),
        payer: "Blue Cross".to_string(),
        member_id: "M123".to_string(),
        procedure_code: "12345".to_string(),
        procedure_description: "MRI lumbar spine".to_string(),
        status: "needs_submission".to_string(),
        priority: "normal".to_string(),
        due_date: "2026-08-01".to_string(),
        summary: "Initial notes".to_string(),
    }
}

#[test]
fn create_then_get_returns_the_same_case() {
    let app = in_memory_app();

    let created = cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();
    let fetched = cases::get_case(app.state::<Db>(), created.id.clone()).unwrap();

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.patient_name, "Jane Doe");
    assert_eq!(fetched.case_number, "PA-00001");
}

#[test]
fn create_then_list_includes_the_case_newest_first() {
    let app = in_memory_app();

    cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();
    cases::create_case(app.state::<Db>(), sample_input("John Smith")).unwrap();

    let all = cases::list_cases(app.state::<Db>()).unwrap();

    assert_eq!(all.len(), 2);
    assert_eq!(all[0].patient_name, "John Smith");
    assert_eq!(all[1].patient_name, "Jane Doe");
}

#[test]
fn update_then_get_reflects_the_change() {
    let app = in_memory_app();
    let created = cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();

    let mut changes = sample_input("Jane Doe");
    changes.status = "approved".to_string();
    changes.priority = "urgent".to_string();
    cases::update_case(app.state::<Db>(), created.id.clone(), changes).unwrap();

    let fetched = cases::get_case(app.state::<Db>(), created.id).unwrap();

    assert_eq!(fetched.status, "approved");
    assert_eq!(fetched.priority, "urgent");
}

#[test]
fn delete_then_get_errors() {
    let app = in_memory_app();
    let created = cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();

    cases::delete_case(app.state::<Db>(), created.id.clone()).unwrap();

    assert!(cases::get_case(app.state::<Db>(), created.id).is_err());
}

#[test]
fn delete_then_list_no_longer_includes_the_case() {
    let app = in_memory_app();
    let created = cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();
    cases::create_case(app.state::<Db>(), sample_input("John Smith")).unwrap();

    cases::delete_case(app.state::<Db>(), created.id).unwrap();

    let all = cases::list_cases(app.state::<Db>()).unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].patient_name, "John Smith");
}

#[test]
fn get_case_with_non_numeric_id_returns_invalid_id_error() {
    let app = in_memory_app();

    let result = cases::get_case(app.state::<Db>(), "not-a-number".to_string());

    assert_eq!(result.unwrap_err(), "Invalid case id");
}

#[test]
fn update_case_with_non_numeric_id_returns_invalid_id_error() {
    let app = in_memory_app();

    let result = cases::update_case(
        app.state::<Db>(),
        "not-a-number".to_string(),
        sample_input("Ghost"),
    );

    assert_eq!(result.unwrap_err(), "Invalid case id");
}

#[test]
fn delete_case_with_non_numeric_id_returns_invalid_id_error() {
    let app = in_memory_app();

    let result = cases::delete_case(app.state::<Db>(), "not-a-number".to_string());

    assert_eq!(result.unwrap_err(), "Invalid case id");
}

/// The real advantage of SQLite: data written in one "session" is visible
/// to a completely separate connection opened against the same file later,
/// just like it will be across real app restarts.
#[test]
fn data_persists_to_a_real_sqlite_file_across_separate_connections() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("workspace.db");

    {
        let conn = Connection::open(&db_path).unwrap();
        let app = mock_app_with_db(conn);
        cases::create_case(app.state::<Db>(), sample_input("Jane Doe")).unwrap();
    }

    let conn = Connection::open(&db_path).unwrap();
    let app = mock_app_with_db(conn);
    let all = cases::list_cases(app.state::<Db>()).unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].patient_name, "Jane Doe");
    assert_eq!(all[0].case_number, "PA-00001");
}
