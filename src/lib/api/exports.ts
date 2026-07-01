import { invoke } from "@tauri-apps/api/core";

export function exportCasesToCsv(): Promise<string> {
  return invoke<string>("export_cases_csv");
}
