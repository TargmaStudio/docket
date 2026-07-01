<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { FilePlus } from "@lucide/svelte";

  import PageHeader from "$lib/components/common/PageHeader.svelte";
  import CaseFilterBar from "$lib/components/cases/CaseFilterBar.svelte";
  import CaseTableEmptyState from "$lib/components/cases/CaseTableEmptyState.svelte";
  import NewCaseModal from "$lib/components/cases/NewCaseModal.svelte";
  import StatusBadge from "$lib/components/cases/StatusBadge.svelte";
  import PriorityBadge from "$lib/components/cases/PriorityBadge.svelte";
  import type { CaseRow, NewCaseForm } from "$lib/types/case";

  let searchQuery = $state("");
  let selectedStatus = $state("all");
  let selectedPriority = $state("all");
  let cases = $state<CaseRow[]>([]);
  let isNewCaseModalOpen = $state(false);

  let filteredCases = $derived.by(() => {
    const query = searchQuery.trim().toLowerCase();

    return cases.filter((caseRow) => {
      const matchesQuery =
        query === "" ||
        caseRow.patientName.toLowerCase().includes(query) ||
        caseRow.payer.toLowerCase().includes(query) ||
        caseRow.procedureDescription.toLowerCase().includes(query) ||
        caseRow.caseNumber.toLowerCase().includes(query);

      const matchesStatus =
        selectedStatus === "all" || caseRow.status === selectedStatus;

      const matchesPriority =
        selectedPriority === "all" || caseRow.priority === selectedPriority;

      return matchesQuery && matchesStatus && matchesPriority;
    });
  });

  function openNewCaseModal() {
    isNewCaseModalOpen = true;
  }

  function clearFilters() {
    searchQuery = "";
    selectedStatus = "all";
    selectedPriority = "all";
  }

  function createMockCase(form: NewCaseForm) {
    const nextNumber = cases.length + 1;

    const newCase: CaseRow = {
      id: crypto.randomUUID(),
      caseNumber: `PA-${String(nextNumber).padStart(5, "0")}`,

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

      lastActivity: "Just now",
    };

    cases = [newCase, ...cases];
  }

  function formatDateForRow(date: Date | undefined): string {
    if (!date) {
      return "";
    }

    return date.toISOString().slice(0, 10);
  }
</script>

<svelte:head>
  <title>Docket · Cases</title>
</svelte:head>

<div class="space-y-6">
  <PageHeader
    title="Cases"
    description="Manage prior authorization cases, statuses, due dates, notes, and documents."
  >
    <button
      type="button"
      onclick={openNewCaseModal}
      class="inline-flex items-center rounded-lg bg-docket-primary px-4 py-2.5 text-sm font-medium text-white shadow-sm transition hover:bg-docket-primary-hover focus:outline-none focus:ring-2 focus:ring-blue-200"
    >
      <FilePlus class="mr-2 h-4 w-4" />
      New Case
    </button>
  </PageHeader>

  <CaseFilterBar bind:searchQuery bind:selectedStatus bind:selectedPriority />

  <div
    class="overflow-hidden rounded-xl border border-docket-border bg-white shadow-sm"
  >
    <div
      class="flex items-center justify-between border-b border-docket-border px-5 py-4"
    >
      <div>
        <h2 class="text-lg font-semibold">Case queue</h2>
        <p class="mt-1 text-sm text-docket-muted">
          {#if cases.length === 0}
            No cases have been created yet.
          {:else if filteredCases.length === cases.length}
            Showing {cases.length} case{cases.length === 1 ? "" : "s"}.
          {:else}
            Showing {filteredCases.length} of {cases.length} case{cases.length ===
            1
              ? ""
              : "s"}.
          {/if}
        </p>
      </div>
    </div>

    <div class="overflow-x-auto">
      <table class="w-full text-left text-sm">
        <thead
          class="border-b border-docket-border bg-slate-50 text-xs uppercase tracking-wide text-slate-500"
        >
          <tr>
            <th class="px-5 py-3 font-semibold">Case #</th>
            <th class="px-5 py-3 font-semibold">Patient</th>
            <th class="px-5 py-3 font-semibold">Payer</th>
            <th class="px-5 py-3 font-semibold">Procedure</th>
            <th class="px-5 py-3 font-semibold">Status</th>
            <th class="px-5 py-3 font-semibold">Priority</th>
            <th class="px-5 py-3 font-semibold">Due Date</th>
            <th class="px-5 py-3 font-semibold">Last Activity</th>
          </tr>
        </thead>

        <tbody>
          {#if cases.length === 0}
            <tr>
              <td colspan="8" class="px-5 py-16 text-center">
                <CaseTableEmptyState onNewCase={openNewCaseModal} />
              </td>
            </tr>
          {:else if filteredCases.length === 0}
            <tr>
              <td colspan="8" class="px-5 py-16 text-center">
                <div class="mx-auto max-w-md">
                  <h3 class="text-base font-semibold text-docket-text">
                    No matching cases
                  </h3>
                  <p class="mt-2 text-sm leading-6 text-docket-muted">
                    No cases match your current search or filters.
                  </p>
                  <div class="mt-6">
                    <button
                      type="button"
                      onclick={clearFilters}
                      class="inline-flex items-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 shadow-sm transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-blue-100"
                    >
                      Clear filters
                    </button>
                  </div>
                </div>
              </td>
            </tr>
          {:else}
            {#each filteredCases as caseRow (caseRow.id)}
              <tr
                class="border-b border-docket-border last:border-b-0 hover:bg-slate-50"
              >
                <td class="px-5 py-4 font-medium text-slate-900">
                  {caseRow.caseNumber}
                </td>
                <td class="px-5 py-4">{caseRow.patientName}</td>
                <td class="px-5 py-4">{caseRow.payer}</td>
                <td class="px-5 py-4">
                  <div class="font-medium text-slate-900">
                    {caseRow.procedureDescription}
                  </div>

                  {#if caseRow.procedureCode}
                    <div class="mt-1 text-xs text-docket-muted">
                      {caseRow.procedureCode}
                    </div>
                  {/if}
                </td>
                <td class="px-5 py-4"><StatusBadge status={caseRow.status} /></td>
                <td class="px-5 py-4">
                  <PriorityBadge priority={caseRow.priority} />
                </td>
                <td class="px-5 py-4">{caseRow.dueDate || "—"}</td>
                <td class="px-5 py-4 text-docket-muted"
                  >{caseRow.lastActivity}</td
                >
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
    </div>
  </div>

  <NewCaseModal bind:open={isNewCaseModalOpen} onCreate={createMockCase} />
</div>
