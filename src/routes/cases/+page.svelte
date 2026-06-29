<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { FilePlus } from "@lucide/svelte";

  import CaseQueueCard from "../../components/cases/CaseQueueCard.svelte";
  import CaseEmptyState from "../../components/cases/CaseEmptyState.svelte";

  const queues = [
    {
      label: "Needs Submission",
      count: 0,
      accentClass: "border-t-slate-400",
      numberClass: "text-slate-700",
    },
    {
      label: "Waiting on Insurance",
      count: 0,
      accentClass: "border-t-blue-500",
      numberClass: "text-blue-700",
    },
    {
      label: "Missing Documentation",
      count: 0,
      accentClass: "border-t-amber-500",
      numberClass: "text-amber-700",
    },
    {
      label: "Due Today",
      count: 0,
      accentClass: "border-t-red-500",
      numberClass: "text-red-700",
    },
  ];
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

  <section class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
    {#each queues as queue}
      <CaseQueueCard
        label={queue.label}
        count={queue.count}
        accentClass={queue.accentClass}
        numberClass={queue.numberClass}
      />
    {/each}
  </section>

  <CaseEmptyState />
</div>