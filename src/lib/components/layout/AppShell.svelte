<script lang="ts">
  import {
    Archive,
    ClipboardList,
    FolderOpen,
    Search,
    Settings,
  } from "@lucide/svelte";

  type Props = {
    workspaceName?: string;
    workspacePath?: string;
    children?: import("svelte").Snippet;
  };

  let {
    workspaceName = "Docket",
    workspacePath = "No workspace open",
    children,
  }: Props = $props();

  const navItems = [
    { label: "Dashboard", href: "/", icon: ClipboardList },
    { label: "Cases", href: "/cases", icon: FolderOpen },
    { label: "Search", href: "/search", icon: Search },
    { label: "Exports", href: "/exports", icon: Archive },
    { label: "Settings", href: "/settings", icon: Settings },
  ];
</script>

<main class="min-h-screen bg-docket-bg text-docket-text">
  <div class="flex min-h-screen">
    <aside class="w-64 border-r border-slate-800 bg-docket-sidebar text-slate-200">
      <div class="border-b border-slate-800 px-6 py-5">
        <h1 class="text-xl font-semibold tracking-tight text-white">Docket</h1>
        <p class="mt-1 text-xs text-slate-400">
          Prior authorization workspace
        </p>
      </div>

      <nav class="space-y-1 p-4 text-sm">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center gap-3 rounded-lg px-3 py-2 text-slate-400 hover:bg-slate-900 hover:text-white"
          >
            <item.icon class="h-4 w-4" />
            {item.label}
          </a>
        {/each}
      </nav>
    </aside>

    <section class="flex-1">
      <header class="flex items-center justify-between border-b border-docket-border bg-docket-surface px-8 py-4">
        <div>
          <p class="text-xs font-medium uppercase tracking-wide text-docket-muted">
            Workspace
          </p>
          <h2 class="text-lg font-semibold">{workspaceName}</h2>
          <p class="mt-1 text-xs text-docket-muted">{workspacePath}</p>
        </div>
      </header>

      <div class="p-8">
        {@render children?.()}
      </div>
    </section>
  </div>
</main>