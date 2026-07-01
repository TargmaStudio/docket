<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { ArrowLeft, Pencil, Trash2 } from "@lucide/svelte";

  import * as casesApi from "$lib/api/cases";
  import NewCaseModal from "$lib/components/cases/NewCaseModal.svelte";
  import StatusBadge from "$lib/components/cases/StatusBadge.svelte";
  import PriorityBadge from "$lib/components/cases/PriorityBadge.svelte";
  import { formatRelativeTime, type CaseRow, type NewCaseForm } from "$lib/types/case";

  let caseId = $derived(page.params.id ?? "");
  let caseRow = $state<CaseRow | null>(null);
  let isLoading = $state(true);
  let errorMessage = $state("");
  let isEditModalOpen = $state(false);

  onMount(() => {
    loadCase();
  });

  async function loadCase() {
    isLoading = true;
    errorMessage = "";

    try {
      caseRow = await casesApi.getCase(caseId);
    } catch (error) {
      errorMessage = "Unable to load this case. It may have been deleted.";
      console.error(error);
    } finally {
      isLoading = false;
    }
  }

  async function handleUpdate(id: string, form: NewCaseForm) {
    errorMessage = "";

    try {
      caseRow = await casesApi.updateCase(id, form);
    } catch (error) {
      errorMessage = "Unable to save changes to the local database.";
      console.error(error);
    }
  }

  async function handleDelete() {
    if (!caseRow) {
      return;
    }

    const confirmed = await confirm(
      `Delete case ${caseRow.caseNumber} for ${caseRow.patientName}? This can't be undone.`,
      { title: "Delete case", kind: "warning" },
    );

    if (!confirmed) {
      return;
    }

    try {
      await casesApi.deleteCase(caseRow.id);
      goto("/cases");
    } catch (error) {
      errorMessage = "Unable to delete the case.";
      console.error(error);
    }
  }
</script>

<svelte:head>
  <title>Docket · {caseRow ? caseRow.caseNumber : "Case"}</title>
</svelte:head>

<div class="space-y-6">
  <a
    href="/cases"
    class="inline-flex items-center gap-1.5 text-sm font-medium text-docket-muted transition hover:text-docket-text"
  >
    <ArrowLeft class="h-4 w-4" />
    Back to Cases
  </a>

  {#if errorMessage}
    <div
      class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {errorMessage}
    </div>
  {/if}

  {#if isLoading}
    <div
      class="rounded-xl border border-docket-border bg-white p-8 text-center text-sm text-docket-muted shadow-sm"
    >
      Loading case…
    </div>
  {:else if caseRow}
    <div
      class="flex items-start justify-between gap-4 rounded-xl border border-docket-border bg-white p-6 shadow-sm"
    >
      <div>
        <div class="flex items-center gap-3">
          <h1 class="text-2xl font-semibold tracking-tight">
            {caseRow.caseNumber}
          </h1>
          <StatusBadge status={caseRow.status} />
          <PriorityBadge priority={caseRow.priority} />
        </div>
        <p class="mt-1 text-sm text-docket-muted">
          {caseRow.patientName} · Last activity {formatRelativeTime(
            caseRow.lastActivity,
          )}
        </p>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <button
          type="button"
          onclick={() => (isEditModalOpen = true)}
          class="inline-flex items-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 shadow-sm transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-blue-100"
        >
          <Pencil class="mr-2 h-4 w-4" />
          Edit
        </button>
        <button
          type="button"
          onclick={handleDelete}
          class="inline-flex items-center rounded-lg border border-red-200 bg-white px-4 py-2.5 text-sm font-medium text-red-600 shadow-sm transition hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-red-100"
        >
          <Trash2 class="mr-2 h-4 w-4" />
          Delete
        </button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
          Patient
        </h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Name</dt>
            <dd class="font-medium text-slate-900">{caseRow.patientName}</dd>
          </div>
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Date of birth</dt>
            <dd class="font-medium text-slate-900">
              {caseRow.dateOfBirth || "—"}
            </dd>
          </div>
        </dl>
      </div>

      <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
          Insurance
        </h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Payer</dt>
            <dd class="font-medium text-slate-900">{caseRow.payer}</dd>
          </div>
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Member ID</dt>
            <dd class="font-medium text-slate-900">
              {caseRow.memberId || "—"}
            </dd>
          </div>
        </dl>
      </div>

      <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
          Procedure
        </h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Description</dt>
            <dd class="font-medium text-slate-900">
              {caseRow.procedureDescription}
            </dd>
          </div>
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Code</dt>
            <dd class="font-medium text-slate-900">
              {caseRow.procedureCode || "—"}
            </dd>
          </div>
        </dl>
      </div>

      <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
          Workflow
        </h2>
        <dl class="mt-4 space-y-3 text-sm">
          <div class="flex justify-between gap-4">
            <dt class="text-docket-muted">Due date</dt>
            <dd class="font-medium text-slate-900">
              {caseRow.dueDate || "—"}
            </dd>
          </div>
        </dl>
      </div>
    </div>

    <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
        Summary / Notes
      </h2>
      <p class="mt-4 whitespace-pre-wrap text-sm text-slate-900">
        {caseRow.summary || "No notes yet."}
      </p>
    </div>

    <NewCaseModal
      bind:open={isEditModalOpen}
      existingCase={caseRow}
      onUpdate={handleUpdate}
    />
  {/if}
</div>
