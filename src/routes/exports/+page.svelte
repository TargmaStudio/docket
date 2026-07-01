<script lang="ts">
  import { onMount } from "svelte";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { FileDown, FolderOpen } from "@lucide/svelte";

  import * as casesApi from "$lib/api/cases";
  import { exportCasesToCsv } from "$lib/api/exports";

  let caseCount = $state<number | null>(null);
  let isLoadingCount = $state(true);
  let isExporting = $state(false);
  let errorMessage = $state("");
  let exportedPath = $state("");

  onMount(() => {
    loadCaseCount();
  });

  async function loadCaseCount() {
    isLoadingCount = true;

    try {
      const cases = await casesApi.listCases();
      caseCount = cases.length;
    } catch (error) {
      console.error(error);
    } finally {
      isLoadingCount = false;
    }
  }

  async function handleExport() {
    isExporting = true;
    errorMessage = "";
    exportedPath = "";

    try {
      exportedPath = await exportCasesToCsv();
    } catch (error) {
      errorMessage = "Unable to export cases to CSV.";
      console.error(error);
    } finally {
      isExporting = false;
    }
  }

  async function revealExport() {
    try {
      await revealItemInDir(exportedPath);
    } catch (error) {
      console.error(error);
    }
  }
</script>

<svelte:head>
  <title>Docket · Exports</title>
</svelte:head>

<div class="space-y-6">
  <div>
    <h1 class="text-2xl font-semibold tracking-tight">Exports</h1>
    <p class="mt-1 text-sm text-docket-muted">
      Export your cases to a CSV file for backup or use outside Docket.
    </p>
  </div>

  {#if errorMessage}
    <div
      class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {errorMessage}
    </div>
  {/if}

  <div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
    <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-500">
      Cases CSV
    </h2>

    <p class="mt-2 text-sm text-docket-muted">
      {#if isLoadingCount}
        Loading case count…
      {:else}
        {caseCount} case{caseCount === 1 ? "" : "s"} will be exported, including
        patient, insurance, procedure, workflow, and notes fields.
      {/if}
    </p>

    <div class="mt-4">
      <button
        type="button"
        onclick={handleExport}
        disabled={isExporting}
        class="inline-flex items-center rounded-lg bg-docket-primary px-4 py-2.5 text-sm font-medium text-white shadow-sm transition hover:bg-docket-primary-hover focus:outline-none focus:ring-2 focus:ring-blue-200 disabled:cursor-not-allowed disabled:opacity-60"
      >
        <FileDown class="mr-2 h-4 w-4" />
        {isExporting ? "Exporting…" : "Export to CSV"}
      </button>
    </div>

    {#if exportedPath}
      <div
        class="mt-4 flex items-center justify-between gap-4 rounded-lg border border-green-200 bg-green-50 px-4 py-3 text-sm text-green-800"
      >
        <span class="truncate">Saved to {exportedPath}</span>
        <button
          type="button"
          onclick={revealExport}
          class="inline-flex shrink-0 items-center rounded-lg border border-green-300 bg-white px-3 py-1.5 text-sm font-medium text-green-800 transition hover:bg-green-100"
        >
          <FolderOpen class="mr-1.5 h-4 w-4" />
          Reveal in Finder
        </button>
      </div>
    {/if}
  </div>
</div>
