import { vi } from "vitest";
import type { CaseRow, NewCaseForm } from "$lib/types/case";

let store: CaseRow[] = [];
let nextCaseNumber = 1;

function formatDateForRow(date: Date | undefined): string {
  if (!date) {
    return "";
  }

  return date.toISOString().slice(0, 10);
}

export function seedCases(cases: CaseRow[]): void {
  store = [...cases];
}

export function getStoredCases(): CaseRow[] {
  return store;
}

export const listCases = vi.fn(async (): Promise<CaseRow[]> => {
  return [...store];
});

export const getCase = vi.fn(async (id: string): Promise<CaseRow> => {
  const found = store.find((caseRow) => caseRow.id === id);

  if (!found) {
    throw new Error(`Case ${id} not found`);
  }

  return found;
});

export const createCase = vi.fn(async (form: NewCaseForm): Promise<CaseRow> => {
  const created: CaseRow = {
    id: `case-${nextCaseNumber}`,
    caseNumber: `PA-${String(nextCaseNumber).padStart(4, "0")}`,
    patientName: form.patientName,
    dateOfBirth: formatDateForRow(form.dateOfBirth),
    payer: form.payer,
    memberId: form.memberId,
    procedureCode: form.procedureCode,
    procedureDescription: form.procedureDescription,
    status: form.status,
    priority: form.priority,
    dueDate: formatDateForRow(form.dueDate),
    summary: form.summary,
    lastActivity: new Date().toISOString(),
  };

  nextCaseNumber += 1;
  store = [created, ...store];

  return created;
});

export const updateCase = vi.fn(
  async (id: string, form: NewCaseForm): Promise<CaseRow> => {
    const existing = store.find((caseRow) => caseRow.id === id);

    if (!existing) {
      throw new Error(`Case ${id} not found`);
    }

    const updated: CaseRow = {
      ...existing,
      patientName: form.patientName,
      dateOfBirth: formatDateForRow(form.dateOfBirth),
      payer: form.payer,
      memberId: form.memberId,
      procedureCode: form.procedureCode,
      procedureDescription: form.procedureDescription,
      status: form.status,
      priority: form.priority,
      dueDate: formatDateForRow(form.dueDate),
      summary: form.summary,
      lastActivity: new Date().toISOString(),
    };

    store = store.map((caseRow) => (caseRow.id === id ? updated : caseRow));

    return updated;
  },
);

export const deleteCase = vi.fn(async (id: string): Promise<void> => {
  store = store.filter((caseRow) => caseRow.id !== id);
});

export function resetCasesApiMock(): void {
  store = [];
  nextCaseNumber = 1;
  listCases.mockClear();
  getCase.mockClear();
  createCase.mockClear();
  updateCase.mockClear();
  deleteCase.mockClear();
}
