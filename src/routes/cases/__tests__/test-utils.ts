import type { CaseRow } from "$lib/types/case";

let nextId = 1;

export function makeCaseRow(overrides: Partial<CaseRow> = {}): CaseRow {
  const id = overrides.id ?? `case-${nextId++}`;

  return {
    id,
    caseNumber: overrides.caseNumber ?? `PA-${id}`,
    patientName: "Jane Doe",
    dateOfBirth: "1990-01-01",
    payer: "Blue Cross",
    memberId: "M12345",
    procedureCode: "70551",
    procedureDescription: "MRI brain without contrast",
    status: "needs_submission",
    priority: "normal",
    dueDate: "2026-08-01",
    summary: "",
    lastActivity: new Date().toISOString(),
    ...overrides,
  };
}
