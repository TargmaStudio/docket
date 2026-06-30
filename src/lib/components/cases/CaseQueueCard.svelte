<script lang="ts">
  type Props = {
    label: string;
    count: number;
    accentClass?: string;
    numberClass?: string;
  };

  let {
    label,
    count,
    accentClass = "border-t-slate-400",
    numberClass = "text-slate-700",
  }: Props = $props();
</script>

<div
  class={`overflow-hidden rounded-xl border border-docket-border border-t-4 bg-white shadow-sm ${accentClass}`}
>
  <div class="px-5 py-4 text-center">
    <h2 class="text-sm font-semibold leading-none text-slate-700">
      {label}
    </h2>
  </div>

  <div class="border-t border-docket-border px-5 py-5">
    <p
      class={`text-center text-4xl font-semibold leading-none tracking-tight ${numberClass}`}
    >
      {count}
    </p>
  </div>
</div>
