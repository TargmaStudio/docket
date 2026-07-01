<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { RotateCcw, Search, SlidersHorizontal } from "@lucide/svelte";
  import { CASE_STATUS_LABELS, CASE_PRIORITY_LABELS } from "$lib/types/case";

  type FilterOption = {
    value: string;
    label: string;
  };

  const DEFAULT_STATUS = "all";
  const DEFAULT_PRIORITY = "all";
  const DEFAULT_SEARCH_QUERY = "";

  const statusOptions: FilterOption[] = [
    { value: "all", label: "All statuses" },
    ...Object.entries(CASE_STATUS_LABELS).map(([value, label]) => ({
      value,
      label,
    })),
  ];

  const priorityOptions: FilterOption[] = [
    { value: "all", label: "All priorities" },
    ...Object.entries(CASE_PRIORITY_LABELS).map(([value, label]) => ({
      value,
      label,
    })),
  ];

  type Props = {
    searchQuery?: string;
    selectedStatus?: string;
    selectedPriority?: string;
  };

  let {
    searchQuery = $bindable(DEFAULT_SEARCH_QUERY),
    selectedStatus = $bindable(DEFAULT_STATUS),
    selectedPriority = $bindable(DEFAULT_PRIORITY),
  }: Props = $props();

  let hasActiveFilters = $derived(
    searchQuery.trim() !== DEFAULT_SEARCH_QUERY ||
      selectedStatus !== DEFAULT_STATUS ||
      selectedPriority !== DEFAULT_PRIORITY,
  );

  function resetFilters() {
    searchQuery = DEFAULT_SEARCH_QUERY;
    selectedStatus = DEFAULT_STATUS;
    selectedPriority = DEFAULT_PRIORITY;
  }
</script>

<div class="rounded-xl border border-docket-border bg-white p-5 shadow-sm">
  <div class="grid gap-4 xl:grid-cols-[1fr_220px_220px_auto_auto]">
    <div>
      <label
        for="case-search"
        class="mb-2 block text-sm font-medium text-slate-700"
      >
        Search cases
      </label>

      <div class="relative">
        <Search
          class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400"
        />

        <input
          id="case-search"
          bind:value={searchQuery}
          type="search"
          placeholder="Search patient, payer, procedure, or case number"
          class="block w-full rounded-lg border border-slate-300 bg-white py-2.5 pl-10 pr-3 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
        />
      </div>
    </div>

    <div>
      <label
        for="status-filter"
        class="mb-2 block text-sm font-medium text-slate-700"
      >
        Status
      </label>

      <select
        id="status-filter"
        bind:value={selectedStatus}
        class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
      >
        {#each statusOptions as option (option.value)}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>

    <div>
      <label
        for="priority-filter"
        class="mb-2 block text-sm font-medium text-slate-700"
      >
        Priority
      </label>

      <select
        id="priority-filter"
        bind:value={selectedPriority}
        class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
      >
        {#each priorityOptions as option (option.value)}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>

    <div class="flex items-end">
      <Button
        color="alternative"
        class="w-full"
        onclick={resetFilters}
        disabled={!hasActiveFilters}
      >
        <RotateCcw class="mr-2 h-4 w-4" />
        Reset
      </Button>
    </div>

    <div class="flex items-end">
      <Button color="alternative" class="w-full">
        <SlidersHorizontal class="mr-2 h-4 w-4" />
        Filters
      </Button>
    </div>
  </div>
</div>