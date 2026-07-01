<script lang="ts">
  import { CASE_PRIORITY_LABELS, type CasePriority } from "$lib/types/case";

  type Props = {
    priority: CasePriority;
  };

  let { priority }: Props = $props();

  const PRIORITY_CLASSES: Record<CasePriority, string> = {
    low: "bg-slate-100 text-slate-600",
    normal: "bg-blue-50 text-blue-600",
    high: "bg-amber-100 text-amber-800",
    urgent: "bg-red-100 text-red-700",
  };
</script>

<span
  class="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium {PRIORITY_CLASSES[
    priority
  ]}"
>
  {CASE_PRIORITY_LABELS[priority]}
</span>
