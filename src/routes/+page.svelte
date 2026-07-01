<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "flowbite-svelte";
  import { FilePlus } from "@lucide/svelte";

  import * as casesApi from "$lib/api/cases";
  import CaseQueueCard from "$lib/components/cases/CaseQueueCard.svelte";
  import CaseEmptyState from "$lib/components/cases/CaseEmptyState.svelte";
  import { CASE_STATUS_LABELS, type CaseRow } from "$lib/types/case";

  let cases = $state<CaseRow[]>([]);
  let isLoadingCases = $state(true);
  let errorMessage = $state("");

  let queueCounts = $derived.by(() => {
    const today = new Date().toISOString().slice(0, 10);

    return [
      {
        label: CASE_STATUS_LABELS.needs_submission,
        count: cases.filter((c) => c.status === "needs_submission").length,
        accentClass: "border-t-slate-400",
        numberClass: "text-slate-700",
      },
      {
        label: CASE_STATUS_LABELS.submitted_waiting,
        count: cases.filter((c) => c.status === "submitted_waiting").length,
        accentClass: "border-t-blue-500",
        numberClass: "text-blue-700",
      },
      {
        label: CASE_STATUS_LABELS.missing_documentation,
        count: cases.filter((c) => c.status === "missing_documentation").length,
        accentClass: "border-t-amber-500",
        numberClass: "text-amber-700",
      },
      {
        label: "Due Today",
        count: cases.filter((c) => c.dueDate === today).length,
        accentClass: "border-t-red-500",
        numberClass: "text-red-700",
      },
    ];
  });

  onMount(() => {
    loadCases();
  });

  async function loadCases() {
    isLoadingCases = true;
    errorMessage = "";

    try {
      cases = await casesApi.listCases();
    } catch (error) {
      errorMessage = "Unable to load cases from the local database.";
      console.error(error);
    } finally {
      isLoadingCases = false;
    }
  }
</script>

<svelte:head>
  <title>Docket · Dashboard</title>
</svelte:head>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Dashboard</h1>
      <p class="mt-1 text-sm text-docket-muted">
        Work queues for prior authorization cases.
      </p>
    </div>

    <Button href="/cases">
      <FilePlus class="mr-2 h-4 w-4" />
      View Cases
    </Button>
  </div>

  {#if errorMessage}
    <div
      class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {errorMessage}
    </div>
  {/if}

  {#if isLoadingCases}
    <div
      class="rounded-xl border border-docket-border bg-white p-8 text-center text-sm text-docket-muted shadow-sm"
    >
      Loading dashboard…
    </div>
  {:else if cases.length === 0}
    <CaseEmptyState />
  {:else}
    <section class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
      {#each queueCounts as queue (queue.label)}
        <CaseQueueCard
          label={queue.label}
          count={queue.count}
          accentClass={queue.accentClass}
          numberClass={queue.numberClass}
        />
      {/each}
    </section>
  {/if}
</div>
