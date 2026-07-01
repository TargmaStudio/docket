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

  lastActivity: string;
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