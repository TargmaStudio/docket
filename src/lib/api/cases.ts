import { invoke } from "@tauri-apps/api/core";
import type { CaseRow, NewCaseForm, NewCasePayload } from "$lib/types/case";

function formatDateForPayload(date: Date | undefined): string {
  if (!date) {
    return "";
  }

  return date.toISOString().slice(0, 10);
}

function toPayload(form: NewCaseForm): NewCasePayload {
  return {
    patientName: form.patientName,
    dateOfBirth: formatDateForPayload(form.dateOfBirth),
    payer: form.payer,
    memberId: form.memberId,
    procedureCode: form.procedureCode,
    procedureDescription: form.procedureDescription,
    status: form.status,
    priority: form.priority,
    dueDate: formatDateForPayload(form.dueDate),
    summary: form.summary,
  };
}

export function listCases(): Promise<CaseRow[]> {
  return invoke<CaseRow[]>("list_cases");
}

export function getCase(id: string): Promise<CaseRow> {
  return invoke<CaseRow>("get_case", { id });
}

export function createCase(form: NewCaseForm): Promise<CaseRow> {
  return invoke<CaseRow>("create_case", { input: toPayload(form) });
}

export function updateCase(id: string, form: NewCaseForm): Promise<CaseRow> {
  return invoke<CaseRow>("update_case", { id, input: toPayload(form) });
}

export function deleteCase(id: string): Promise<void> {
  return invoke("delete_case", { id });
}
