import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";

import CasesPage from "../+page.svelte";
import { makeCaseRow } from "./test-utils";
import * as casesApiMock from "./cases-api.mock";

vi.mock("$lib/api/cases", () => import("./cases-api.mock"));

vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
}));

const confirmMock = vi.fn();
vi.mock("@tauri-apps/plugin-dialog", () => ({
  confirm: (...args: unknown[]) => confirmMock(...args),
}));

import { goto } from "$app/navigation";

describe("cases list page", () => {
  beforeEach(() => {
    casesApiMock.resetCasesApiMock();
    confirmMock.mockReset();
    vi.mocked(goto).mockReset();
  });

  it("shows a loading state and then renders case rows once cases resolve", async () => {
    casesApiMock.seedCases([
      makeCaseRow({ id: "case-1", patientName: "Alice Anderson" }),
      makeCaseRow({ id: "case-2", patientName: "Bob Brown" }),
    ]);

    render(CasesPage);

    expect(screen.getAllByText("Loading cases…").length).toBeGreaterThan(0);

    expect(await screen.findByText("Alice Anderson")).toBeInTheDocument();
    expect(screen.getByText("Bob Brown")).toBeInTheDocument();
    expect(screen.queryByText("Loading cases…")).not.toBeInTheDocument();
  });

  it("renders the empty state when there are no cases", async () => {
    casesApiMock.seedCases([]);

    render(CasesPage);

    expect(await screen.findByText("No cases yet")).toBeInTheDocument();
    expect(
      screen.getByText("No cases have been created yet."),
    ).toBeInTheDocument();
  });

  it("creates a case via the New Case modal and shows the new row in the table", async () => {
    const user = userEvent.setup();
    casesApiMock.seedCases([]);

    render(CasesPage);

    await screen.findByText("No cases yet");

    const [newCaseButton] = screen.getAllByRole("button", { name: /new case/i });
    await user.click(newCaseButton);

    await user.type(screen.getByLabelText("Patient name"), "Charlie Chaplin");
    await user.type(screen.getByLabelText("Payer"), "Aetna");
    await user.type(
      screen.getByLabelText("Procedure description"),
      "Knee arthroscopy",
    );

    await user.click(screen.getByRole("button", { name: "Create Case" }));

    expect(await screen.findByText("Charlie Chaplin")).toBeInTheDocument();
    expect(casesApiMock.createCase).toHaveBeenCalledTimes(1);
    expect(casesApiMock.createCase).toHaveBeenCalledWith(
      expect.objectContaining({
        patientName: "Charlie Chaplin",
        payer: "Aetna",
        procedureDescription: "Knee arthroscopy",
      }),
    );
  });

  it("deletes a case after confirming, removing the row from the table", async () => {
    const user = userEvent.setup();
    confirmMock.mockResolvedValue(true);

    casesApiMock.seedCases([
      makeCaseRow({ id: "case-1", patientName: "Dana Delgado" }),
    ]);

    render(CasesPage);

    await screen.findByText("Dana Delgado");

    const actionsButton = screen.getByRole("button", {
      name: /actions for case/i,
    });
    await user.click(actionsButton);

    const deleteItem = await screen.findByText("Delete");
    await user.click(deleteItem);

    expect(confirmMock).toHaveBeenCalledTimes(1);
    expect(await screen.findByText("No cases yet")).toBeInTheDocument();
    expect(casesApiMock.deleteCase).toHaveBeenCalledWith("case-1");
    expect(screen.queryByText("Dana Delgado")).not.toBeInTheDocument();
  });

  it("does not delete when the confirmation dialog is declined", async () => {
    const user = userEvent.setup();
    confirmMock.mockResolvedValue(false);

    casesApiMock.seedCases([
      makeCaseRow({ id: "case-1", patientName: "Erin Evans" }),
    ]);

    render(CasesPage);

    await screen.findByText("Erin Evans");

    await user.click(screen.getByRole("button", { name: /actions for case/i }));
    await user.click(await screen.findByText("Delete"));

    expect(confirmMock).toHaveBeenCalledTimes(1);
    expect(casesApiMock.deleteCase).not.toHaveBeenCalled();
    expect(screen.getByText("Erin Evans")).toBeInTheDocument();
  });

  it("filters visible rows by patient name via the search input", async () => {
    const user = userEvent.setup();
    casesApiMock.seedCases([
      makeCaseRow({ id: "case-1", patientName: "Frank Foster" }),
      makeCaseRow({ id: "case-2", patientName: "Grace Green" }),
    ]);

    render(CasesPage);

    await screen.findByText("Frank Foster");
    expect(screen.getByText("Grace Green")).toBeInTheDocument();

    const searchInput = screen.getByLabelText("Search cases");
    await user.type(searchInput, "Frank");

    expect(screen.getByText("Frank Foster")).toBeInTheDocument();
    expect(screen.queryByText("Grace Green")).not.toBeInTheDocument();
  });
});
