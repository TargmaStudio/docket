<script lang="ts">
  import { Button, Modal, Datepicker } from "flowbite-svelte";
  import { X } from "@lucide/svelte";

  import type { CaseRow, NewCaseForm } from "$lib/types/case";
  import { caseRowToForm, createEmptyNewCaseForm } from "$lib/types/case";

  type Props = {
    open?: boolean;
    existingCase?: CaseRow | null;
    onClose?: () => void;
    onCreate?: (form: NewCaseForm) => void;
    onUpdate?: (id: string, form: NewCaseForm) => void;
  };

  let {
    open = $bindable(false),
    existingCase = null,
    onClose,
    onCreate,
    onUpdate,
  }: Props = $props();

  let form = $state<NewCaseForm>(createEmptyNewCaseForm());

  $effect(() => {
    if (open) {
      form = existingCase ? caseRowToForm(existingCase) : createEmptyNewCaseForm();
    }
  });

  function closeModal() {
    open = false;
    onClose?.();
  }

  function handleCancel() {
    closeModal();
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const trimmedForm: NewCaseForm = {
      ...form,
      patientName: form.patientName.trim(),
      payer: form.payer.trim(),
      memberId: form.memberId.trim(),
      procedureCode: form.procedureCode.trim(),
      procedureDescription: form.procedureDescription.trim(),
      summary: form.summary.trim(),
    };

    if (existingCase) {
      onUpdate?.(existingCase.id, trimmedForm);
    } else {
      onCreate?.(trimmedForm);
    }

    closeModal();
  }
</script>

<Modal bind:open size="4xl" dismissable={false}>
  <form onsubmit={handleSubmit} class="flex max-h-[85vh] flex-col">
    <div
      class="flex items-start justify-between border-b border-docket-border pb-4"
    >
      <div>
        <h2 class="text-xl font-semibold tracking-tight text-docket-text">
          {existingCase ? "Edit Case" : "New Case"}
        </h2>
        <p class="mt-1 text-sm text-docket-muted">
          {existingCase
            ? "Update details for this prior authorization case."
            : "Create a prior authorization case in the local workspace."}
        </p>
      </div>

      <button
        type="button"
        onclick={handleCancel}
        class="rounded-lg p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700"
        aria-label="Close modal"
      >
        <X class="h-5 w-5" />
      </button>
    </div>

    <section class="space-y-4">
      <div>
        <h3
          class="text-sm font-semibold uppercase tracking-wide text-slate-500"
        >
          Patient
        </h3>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <div>
          <label
            for="patient-name"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Patient name
          </label>

          <input
            id="patient-name"
            bind:value={form.patientName}
            required
            type="text"
            placeholder="Jane Doe"
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          />
        </div>

        <div>
          <label
            for="date-of-birth"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Date of birth
          </label>

          <Datepicker
            id="date-of-birth"
            bind:value={form.dateOfBirth}
            placeholder="Select date of birth"
            class="w-full"
          />
        </div>
      </div>
    </section>

    <section class="space-y-4">
      <div>
        <h3
          class="text-sm font-semibold uppercase tracking-wide text-slate-500"
        >
          Insurance
        </h3>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <div>
          <label
            for="payer"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Payer
          </label>

          <input
            id="payer"
            bind:value={form.payer}
            required
            type="text"
            placeholder="Blue Cross"
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          />
        </div>

        <div>
          <label
            for="member-id"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Member ID
          </label>

          <input
            id="member-id"
            bind:value={form.memberId}
            type="text"
            placeholder="Optional"
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          />
        </div>
      </div>
    </section>

    <section class="space-y-4">
      <div>
        <h3
          class="text-sm font-semibold uppercase tracking-wide text-slate-500"
        >
          Procedure
        </h3>
      </div>

      <div class="grid gap-4 md:grid-cols-[180px_1fr]">
        <div>
          <label
            for="procedure-code"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Procedure code
          </label>

          <input
            id="procedure-code"
            bind:value={form.procedureCode}
            type="text"
            placeholder="CPT / HCPCS"
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          />
        </div>

        <div>
          <label
            for="procedure-description"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Procedure description
          </label>

          <input
            id="procedure-description"
            bind:value={form.procedureDescription}
            required
            type="text"
            placeholder="MRI lumbar spine without contrast"
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          />
        </div>
      </div>
    </section>

    <section class="space-y-4">
      <div>
        <h3
          class="text-sm font-semibold uppercase tracking-wide text-slate-500"
        >
          Workflow
        </h3>
      </div>

      <div class="grid gap-4 md:grid-cols-3">
        <div>
          <label
            for="status"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Status
          </label>

          <select
            id="status"
            bind:value={form.status}
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          >
            <option value="needs_submission">Needs Submission</option>
            <option value="submitted_waiting">Waiting on Insurance</option>
            <option value="missing_documentation">Missing Documentation</option>
            <option value="insurance_followup">Insurance Follow-up</option>
            <option value="approved">Approved</option>
            <option value="denied">Denied</option>
            <option value="closed">Closed</option>
          </select>
        </div>

        <div>
          <label
            for="priority"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Priority
          </label>

          <select
            id="priority"
            bind:value={form.priority}
            class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
          >
            <option value="low">Low</option>
            <option value="normal">Normal</option>
            <option value="high">High</option>
            <option value="urgent">Urgent</option>
          </select>
        </div>

        <div>
          <label
            for="due-date"
            class="mb-2 block text-sm font-medium text-slate-700"
          >
            Due date
          </label>

          <Datepicker
            id="due-date"
            bind:value={form.dueDate}
            placeholder="Select due date"
            class="w-full"
          />
        </div>
      </div>
    </section>

    <section>
      <label
        for="summary"
        class="mb-2 block text-sm font-medium text-slate-700"
      >
        Summary / notes
      </label>

      <textarea
        id="summary"
        bind:value={form.summary}
        rows="4"
        placeholder="Initial notes, missing documents, portal details, or follow-up instructions."
        class="block w-full rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100"
      ></textarea>
    </section>

    <div
      class="flex items-center justify-end gap-3 border-t border-docket-border bg-white pt-5"
    >
      <button
        type="button"
        onclick={handleCancel}
        class="inline-flex items-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 shadow-sm transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-blue-100"
      >
        Cancel
      </button>

      <button
        type="submit"
        class="inline-flex items-center rounded-lg bg-docket-primary px-4 py-2.5 text-sm font-medium text-white shadow-sm transition hover:bg-docket-primary-hover focus:outline-none focus:ring-2 focus:ring-blue-200"
      >
        {existingCase ? "Save Changes" : "Create Case"}
      </button>
    </div>
  </form>
</Modal>
