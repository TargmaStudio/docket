<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { ClipboardList, FilePlus } from "@lucide/svelte";

  type Props = {
    title?: string;
    message?: string;
    actionLabel?: string;
    actionHref?: string;
  };

  let {
    title = "No cases yet",
    message = "Once you create prior authorization cases, your queues and deadlines will appear here.",
    actionLabel = "Go to Cases",
    actionHref = "/cases",
  }: Props = $props();
</script>

<div class="rounded-xl border border-docket-border bg-white p-6 shadow-sm">
  <div class="mx-auto max-w-md py-16 text-center">
    <ClipboardList class="mx-auto h-10 w-10 text-slate-400" />

    <h2 class="mt-4 text-lg font-semibold text-docket-text">
      {title}
    </h2>

    <p class="mt-2 text-sm leading-6 text-docket-muted">
      {message}
    </p>

    <div class="mt-6">
      <Button href={actionHref}>
        <FilePlus class="mr-2 h-4 w-4" />
        {actionLabel}
      </Button>
    </div>
  </div>
</div>
