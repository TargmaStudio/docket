<script lang="ts">
  import { ClipboardList, FilePlus } from "@lucide/svelte";

  type Props = {
    onNewCase?: () => void;
  };

  let { onNewCase }: Props = $props();
</script>

<div class="mx-auto max-w-md">
  <ClipboardList class="mx-auto h-10 w-10 text-slate-400" />

  <h3 class="mt-4 text-base font-semibold text-docket-text">No cases yet</h3>

  <p class="mt-2 text-sm leading-6 text-docket-muted">
    Create your first prior authorization case to start building the local
    workspace.
  </p>

  <div class="mt-6">
    <button
      type="button"
      onclick={onNewCase}
      class="inline-flex items-center rounded-lg bg-docket-primary px-4 py-2.5 text-sm font-medium text-white shadow-sm transition hover:bg-docket-primary-hover focus:outline-none focus:ring-2 focus:ring-blue-200"
    >
      <FilePlus class="mr-2 h-4 w-4" />
      New Case
    </button>
  </div>
</div>
