export type CaseStatus =
  | "needs_submission"
  | "submitted_waiting"
  | "missing_documentation"
  | "insurance_followup"
  | "approved"
  | "denied"
  | "closed";

export type CasePriority = "low" | "normal" | "high" | "urgent";

export type CaseRow = {
  id: string;
  caseNumber: string;

  patientName: string;
  dateOfBirth: string;

  payer: string;
  memberId: string;

  procedureCode: string;
  procedureDescription: string;

  status: CaseStatus;
  priority: CasePriority;

  dueDate: string;
  summary: string;

  /** ISO timestamp; format with `formatRelativeTime` for display. */
  lastActivity: string;
};

/** New case input sent to the `create_case` Tauri command. */
export type NewCasePayload = {
  patientName: string;
  dateOfBirth: string;

  payer: string;
  memberId: string;

  procedureCode: string;
  procedureDescription: string;

  status: CaseStatus;
  priority: CasePriority;

  dueDate: string;
  summary: string;
};

export type NewCaseForm = {
  patientName: string;
  dateOfBirth: Date | undefined;

  payer: string;
  memberId: string;

  procedureCode: string;
  procedureDescription: string;

  status: CaseStatus;
  priority: CasePriority;

  dueDate: Date | undefined;
  summary: string;
};

export const DEFAULT_CASE_STATUS: CaseStatus = "needs_submission";
export const DEFAULT_CASE_PRIORITY: CasePriority = "normal";

export const CASE_STATUS_LABELS: Record<CaseStatus, string> = {
  needs_submission: "Needs Submission",
  submitted_waiting: "Waiting on Insurance",
  missing_documentation: "Missing Documentation",
  insurance_followup: "Insurance Follow-up",
  approved: "Approved",
  denied: "Denied",
  closed: "Closed",
};

export const CASE_PRIORITY_LABELS: Record<CasePriority, string> = {
  low: "Low",
  normal: "Normal",
  high: "High",
  urgent: "Urgent",
};

export function formatRelativeTime(isoTimestamp: string): string {
  const then = new Date(isoTimestamp).getTime();
  const diffMinutes = Math.floor((Date.now() - then) / 60_000);

  if (diffMinutes < 1) return "Just now";
  if (diffMinutes < 60) {
    return `${diffMinutes} minute${diffMinutes === 1 ? "" : "s"} ago`;
  }

  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) {
    return `${diffHours} hour${diffHours === 1 ? "" : "s"} ago`;
  }

  const diffDays = Math.floor(diffHours / 24);
  return `${diffDays} day${diffDays === 1 ? "" : "s"} ago`;
}

export function createEmptyNewCaseForm(): NewCaseForm {
  return {
    patientName: "",
    dateOfBirth: undefined,

    payer: "",
    memberId: "",

    procedureCode: "",
    procedureDescription: "",

    status: DEFAULT_CASE_STATUS,
    priority: DEFAULT_CASE_PRIORITY,

    dueDate: undefined,
    summary: "",
  };
}

export function caseRowToForm(caseRow: CaseRow): NewCaseForm {
  return {
    patientName: caseRow.patientName,
    dateOfBirth: caseRow.dateOfBirth ? new Date(caseRow.dateOfBirth) : undefined,

    payer: caseRow.payer,
    memberId: caseRow.memberId,

    procedureCode: caseRow.procedureCode,
    procedureDescription: caseRow.procedureDescription,

    status: caseRow.status,
    priority: caseRow.priority,

    dueDate: caseRow.dueDate ? new Date(caseRow.dueDate) : undefined,
    summary: caseRow.summary,
  };
}