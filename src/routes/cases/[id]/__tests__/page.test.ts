import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";

import CaseDetailPage from "../+page.svelte";
import { makeCaseRow } from "../../__tests__/test-utils";
import * as casesApiMock from "../../__tests__/cases-api.mock";

vi.mock("$lib/api/cases", () => import("../../__tests__/cases-api.mock"));

vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  confirm: vi.fn(),
}));

vi.mock("$app/state", () => ({
  page: {
    params: { id: "case-1" },
  },
}));

describe("case detail page", () => {
  beforeEach(() => {
    casesApiMock.resetCasesApiMock();
  });

  it("loads and displays a case's details from getCase", async () => {
    casesApiMock.seedCases([
      makeCaseRow({
        id: "case-1",
        caseNumber: "PA-0001",
        patientName: "Helen Hunt",
        payer: "United Healthcare",
        procedureDescription: "Lumbar MRI",
      }),
    ]);

    render(CaseDetailPage);

    expect(await screen.findByText("PA-0001")).toBeInTheDocument();
    expect(screen.getByText("United Healthcare")).toBeInTheDocument();
    expect(screen.getByText("Lumbar MRI")).toBeInTheDocument();
    expect(casesApiMock.getCase).toHaveBeenCalledWith("case-1");
  });

  it("shows an error message when the case fails to load", async () => {
    casesApiMock.seedCases([]);

    render(CaseDetailPage);

    expect(
      await screen.findByText("Unable to load this case. It may have been deleted."),
    ).toBeInTheDocument();
  });
});
