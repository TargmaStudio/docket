<script lang="ts">
  import "../app.css";
  import AppShell from "$lib/components/layout/AppShell.svelte";

  let { children } = $props();
</script>

<AppShell workspaceName="Docket" workspacePath="~/Documents/Docket">
  {@render children()}
</AppShell>