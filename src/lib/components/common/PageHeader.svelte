<script lang="ts">
  type Props = {
    title: string;
    description?: string;
    children?: import("svelte").Snippet;
  };

  let { title, description, children }: Props = $props();
</script>

<div class="flex items-start justify-between gap-4">
  <div>
    <h1 class="text-2xl font-semibold tracking-tight">{title}</h1>

    {#if description}
      <p class="mt-1 text-sm text-docket-muted">
        {description}
      </p>
    {/if}
  </div>

  {#if children}
    <div class="shrink-0">
      {@render children()}
    </div>
  {/if}
</div>
