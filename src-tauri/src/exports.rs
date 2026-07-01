use crate::cases::{self, priority_label, status_label, CaseRow, Db};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};

const CSV_HEADER: [&str; 12] = [
    "Case Number",
    "Patient Name",
    "Date of Birth",
    "Payer",
    "Member ID",
    "Procedure Code",
    "Procedure Description",
    "Status",
    "Priority",
    "Due Date",
    "Summary",
    "Last Activity",
];

fn exports_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .document_dir()
        .map_err(|e| e.to_string())?
        .join("Docket")
        .join("exports");

    Ok(dir)
}

fn timestamp_for_filename(db: &State<Db>) -> Result<String, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT strftime('%Y%m%d-%H%M%S', 'now')", [], |row| {
        row.get(0)
    })
    .map_err(|e| e.to_string())
}

fn write_cases_csv(
    cases: &[CaseRow],
    dir: &Path,
    timestamp: &str,
) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    let file_path = dir.join(format!("cases-{timestamp}.csv"));

    let mut writer = csv::Writer::from_path(&file_path).map_err(|e| e.to_string())?;

    writer
        .write_record(CSV_HEADER)
        .map_err(|e| e.to_string())?;

    for case_row in cases {
        writer
            .write_record([
                case_row.case_number.as_str(),
                case_row.patient_name.as_str(),
                case_row.date_of_birth.as_str(),
                case_row.payer.as_str(),
                case_row.member_id.as_str(),
                case_row.procedure_code.as_str(),
                case_row.procedure_description.as_str(),
                status_label(&case_row.status),
                priority_label(&case_row.priority),
                case_row.due_date.as_str(),
                case_row.summary.as_str(),
                case_row.last_activity.as_str(),
            ])
            .map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;

    Ok(file_path)
}

#[tauri::command]
pub fn export_cases_csv(app: AppHandle, db: State<Db>) -> Result<String, String> {
    let all_cases = cases::list_cases(db.clone())?;
    let timestamp = timestamp_for_filename(&db)?;
    let dir = exports_dir(&app)?;

    let file_path = write_cases_csv(&all_cases, &dir, &timestamp)?;

    Ok(file_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_case(patient_name: &str, summary: &str) -> CaseRow {
        CaseRow {
            id: "1".to_string(),
            case_number: "PA-00001".to_string(),
            patient_name: patient_name.to_string(),
            date_of_birth: "1990-01-01".to_string(),
            payer: "Blue Cross".to_string(),
            member_id: "M123".to_string(),
            procedure_code: "12345".to_string(),
            procedure_description: "MRI lumbar spine".to_string(),
            status: "needs_submission".to_string(),
            priority: "urgent".to_string(),
            due_date: "2026-08-01".to_string(),
            summary: summary.to_string(),
            last_activity: "2026-07-01T12:00:00.000Z".to_string(),
        }
    }

    fn read_file(path: &Path) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn writes_a_header_and_one_row_per_case_with_readable_labels() {
        let dir = tempfile::tempdir().unwrap();
        let cases = vec![sample_case("Jane Doe", "Initial notes")];

        let file_path = write_cases_csv(&cases, dir.path(), "20260701-120000").unwrap();
        let contents = read_file(&file_path);

        assert!(file_path.ends_with("cases-20260701-120000.csv"));
        assert!(contents.starts_with("Case Number,Patient Name"));
        assert!(contents.contains("Needs Submission"));
        assert!(contents.contains("Urgent"));
        assert!(contents.contains("Jane Doe"));
    }

    #[test]
    fn escapes_commas_and_quotes_in_free_text_fields() {
        let dir = tempfile::tempdir().unwrap();
        let cases = vec![sample_case(
            "Jane Doe",
            "Missing docs, need \"authorization\" letter",
        )];

        let file_path = write_cases_csv(&cases, dir.path(), "20260701-120000").unwrap();
        let contents = read_file(&file_path);

        assert!(contents.contains("\"Missing docs, need \"\"authorization\"\" letter\""));

        let mut reader = csv::Reader::from_path(&file_path).unwrap();
        let record = reader.records().next().unwrap().unwrap();
        assert_eq!(
            record.get(10).unwrap(),
            "Missing docs, need \"authorization\" letter"
        );
    }

    #[test]
    fn writes_just_the_header_when_there_are_no_cases() {
        let dir = tempfile::tempdir().unwrap();

        let file_path = write_cases_csv(&[], dir.path(), "20260701-120000").unwrap();
        let contents = read_file(&file_path);

        assert_eq!(contents.trim(), CSV_HEADER.join(","));
    }

    #[test]
    fn creates_the_exports_directory_if_it_does_not_exist_yet() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("Docket").join("exports");
        assert!(!nested.exists());

        write_cases_csv(&[], &nested, "20260701-120000").unwrap();

        assert!(nested.exists());
    }
}
