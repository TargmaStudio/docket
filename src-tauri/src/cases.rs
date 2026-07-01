use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

pub struct Db(pub Mutex<Connection>);

const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS cases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    patient_name TEXT NOT NULL,
    date_of_birth TEXT NOT NULL,
    payer TEXT NOT NULL,
    member_id TEXT NOT NULL,
    procedure_code TEXT NOT NULL,
    procedure_description TEXT NOT NULL,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    due_date TEXT NOT NULL,
    summary TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
";

pub fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(SCHEMA_SQL)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseRow {
    pub id: String,
    pub case_number: String,
    pub patient_name: String,
    pub date_of_birth: String,
    pub payer: String,
    pub member_id: String,
    pub procedure_code: String,
    pub procedure_description: String,
    pub status: String,
    pub priority: String,
    pub due_date: String,
    pub summary: String,
    pub last_activity: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCaseInput {
    pub patient_name: String,
    pub date_of_birth: String,
    pub payer: String,
    pub member_id: String,
    pub procedure_code: String,
    pub procedure_description: String,
    pub status: String,
    pub priority: String,
    pub due_date: String,
    pub summary: String,
}

fn case_number_for(id: i64) -> String {
    format!("PA-{:05}", id)
}

fn row_to_case(row: &Row) -> rusqlite::Result<CaseRow> {
    let id: i64 = row.get("id")?;

    Ok(CaseRow {
        id: id.to_string(),
        case_number: case_number_for(id),
        patient_name: row.get("patient_name")?,
        date_of_birth: row.get("date_of_birth")?,
        payer: row.get("payer")?,
        member_id: row.get("member_id")?,
        procedure_code: row.get("procedure_code")?,
        procedure_description: row.get("procedure_description")?,
        status: row.get("status")?,
        priority: row.get("priority")?,
        due_date: row.get("due_date")?,
        summary: row.get("summary")?,
        last_activity: row.get("updated_at")?,
    })
}

fn get_case_impl(conn: &Connection, id: i64) -> rusqlite::Result<CaseRow> {
    conn.query_row("SELECT * FROM cases WHERE id = ?1", params![id], row_to_case)
}

fn list_cases_impl(conn: &Connection) -> rusqlite::Result<Vec<CaseRow>> {
    let mut statement = conn.prepare("SELECT * FROM cases ORDER BY id DESC")?;
    let rows = statement
        .query_map([], row_to_case)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

fn create_case_impl(conn: &Connection, input: NewCaseInput) -> rusqlite::Result<CaseRow> {
    conn.execute(
        "INSERT INTO cases (
            patient_name, date_of_birth, payer, member_id,
            procedure_code, procedure_description, status, priority,
            due_date, summary
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            input.patient_name,
            input.date_of_birth,
            input.payer,
            input.member_id,
            input.procedure_code,
            input.procedure_description,
            input.status,
            input.priority,
            input.due_date,
            input.summary,
        ],
    )?;

    let id = conn.last_insert_rowid();
    conn.query_row("SELECT * FROM cases WHERE id = ?1", params![id], row_to_case)
}

fn update_case_impl(conn: &Connection, id: i64, input: NewCaseInput) -> rusqlite::Result<CaseRow> {
    conn.execute(
        "UPDATE cases SET
            patient_name = ?1, date_of_birth = ?2, payer = ?3, member_id = ?4,
            procedure_code = ?5, procedure_description = ?6, status = ?7, priority = ?8,
            due_date = ?9, summary = ?10, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        WHERE id = ?11",
        params![
            input.patient_name,
            input.date_of_birth,
            input.payer,
            input.member_id,
            input.procedure_code,
            input.procedure_description,
            input.status,
            input.priority,
            input.due_date,
            input.summary,
            id,
        ],
    )?;

    conn.query_row("SELECT * FROM cases WHERE id = ?1", params![id], row_to_case)
}

fn delete_case_impl(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM cases WHERE id = ?1", params![id])?;
    Ok(())
}

#[tauri::command]
pub fn get_case(db: State<Db>, id: String) -> Result<CaseRow, String> {
    let case_id: i64 = id.parse().map_err(|_| "Invalid case id".to_string())?;
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    get_case_impl(&conn, case_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_cases(db: State<Db>) -> Result<Vec<CaseRow>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    list_cases_impl(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_case(db: State<Db>, input: NewCaseInput) -> Result<CaseRow, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    create_case_impl(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_case(db: State<Db>, id: String, input: NewCaseInput) -> Result<CaseRow, String> {
    let case_id: i64 = id.parse().map_err(|_| "Invalid case id".to_string())?;
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    update_case_impl(&conn, case_id, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_case(db: State<Db>, id: String) -> Result<(), String> {
    let case_id: i64 = id.parse().map_err(|_| "Invalid case id".to_string())?;
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    delete_case_impl(&conn, case_id).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();
        conn
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
    fn create_case_assigns_sequential_case_numbers() {
        let conn = setup();

        let first = create_case_impl(&conn, sample_input("Jane Doe")).unwrap();
        let second = create_case_impl(&conn, sample_input("John Smith")).unwrap();

        assert_eq!(first.case_number, "PA-00001");
        assert_eq!(second.case_number, "PA-00002");
        assert_eq!(second.patient_name, "John Smith");
    }

    #[test]
    fn list_cases_orders_newest_first() {
        let conn = setup();
        create_case_impl(&conn, sample_input("Jane Doe")).unwrap();
        create_case_impl(&conn, sample_input("John Smith")).unwrap();

        let cases = list_cases_impl(&conn).unwrap();

        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].patient_name, "John Smith");
        assert_eq!(cases[1].patient_name, "Jane Doe");
    }

    #[test]
    fn delete_case_removes_it_from_the_list() {
        let conn = setup();
        let created = create_case_impl(&conn, sample_input("Jane Doe")).unwrap();

        delete_case_impl(&conn, created.id.parse().unwrap()).unwrap();

        assert!(list_cases_impl(&conn).unwrap().is_empty());
    }

    #[test]
    fn case_numbers_are_never_reused_after_delete() {
        let conn = setup();
        let first = create_case_impl(&conn, sample_input("Jane Doe")).unwrap();
        delete_case_impl(&conn, first.id.parse().unwrap()).unwrap();

        let second = create_case_impl(&conn, sample_input("John Smith")).unwrap();

        assert_eq!(second.case_number, "PA-00002");
    }

    #[test]
    fn update_case_persists_changes_and_keeps_case_number() {
        let conn = setup();
        let created = create_case_impl(&conn, sample_input("Jane Doe")).unwrap();

        let mut changes = sample_input("Jane Doe");
        changes.status = "approved".to_string();
        changes.priority = "urgent".to_string();

        let updated =
            update_case_impl(&conn, created.id.parse().unwrap(), changes).unwrap();

        assert_eq!(updated.status, "approved");
        assert_eq!(updated.priority, "urgent");
        assert_eq!(updated.case_number, created.case_number);
        assert_ne!(updated.last_activity, "");
    }

    #[test]
    fn update_case_with_unknown_id_errors() {
        let conn = setup();

        let result = update_case_impl(&conn, 999, sample_input("Ghost"));

        assert!(result.is_err());
    }

    #[test]
    fn get_case_returns_the_matching_case() {
        let conn = setup();
        create_case_impl(&conn, sample_input("Jane Doe")).unwrap();
        let second = create_case_impl(&conn, sample_input("John Smith")).unwrap();

        let fetched = get_case_impl(&conn, second.id.parse().unwrap()).unwrap();

        assert_eq!(fetched.id, second.id);
        assert_eq!(fetched.patient_name, "John Smith");
    }

    #[test]
    fn get_case_with_unknown_id_errors() {
        let conn = setup();

        let result = get_case_impl(&conn, 999);

        assert!(result.is_err());
    }
}
