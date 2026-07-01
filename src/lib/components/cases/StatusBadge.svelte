<script lang="ts">
  import { CASE_STATUS_LABELS, type CaseStatus } from "$lib/types/case";

  type Props = {
    status: CaseStatus;
  };

  let { status }: Props = $props();

  const STATUS_CLASSES: Record<CaseStatus, string> = {
    needs_submission: "bg-slate-100 text-slate-700",
    submitted_waiting: "bg-blue-100 text-blue-700",
    missing_documentation: "bg-amber-100 text-amber-800",
    insurance_followup: "bg-blue-50 text-blue-600",
    approved: "bg-green-100 text-green-700",
    denied: "bg-red-100 text-red-700",
    closed: "bg-slate-100 text-slate-500",
  };
</script>

<span
  class="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium {STATUS_CLASSES[
    status
  ]}"
>
  {CASE_STATUS_LABELS[status]}
</span>
