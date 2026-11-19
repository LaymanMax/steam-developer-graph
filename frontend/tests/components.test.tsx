/// <reference types="@testing-library/jest-dom/vitest" />
import "@testing-library/jest-dom/vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { Users2 } from "lucide-react";
import { describe, expect, it, vi } from "vitest";
import { Badge } from "../src/components/common/Badge";
import { EmptyState } from "../src/components/common/EmptyState";
import { Pagination } from "../src/components/common/Pagination";
import { StatCard } from "../src/components/common/StatCard";
import { Sidebar } from "../src/components/layout/Sidebar";

describe("Common Components", () => {
  it("renders Badge with correct text and variant classes", () => {
    const { container } = render(<Badge variant="success">Confirmed</Badge>);
    expect(screen.getByText("Confirmed")).toBeInTheDocument();
    expect(container.firstChild).toHaveClass("text-emerald-400");
  });

  it("renders StatCard with label, value, and subValue", () => {
    render(
      <StatCard
        label="Total Developers"
        value="14,250"
        subValue="Catalog active"
        icon={<Users2 data-testid="icon" />}
      />,
    );
    expect(screen.getByText("Total Developers")).toBeInTheDocument();
    expect(screen.getByText("14,250")).toBeInTheDocument();
    expect(screen.getByText("Catalog active")).toBeInTheDocument();
    expect(screen.getByTestId("icon")).toBeInTheDocument();
  });

  it("renders EmptyState with action button", () => {
    const handleAction = vi.fn();
    render(
      <EmptyState
        title="No Results"
        description="Try relaxing your filters."
        action={{ label: "Reset Filters", onClick: handleAction }}
      />,
    );
    expect(screen.getByText("No Results")).toBeInTheDocument();
    expect(screen.getByText("Try relaxing your filters.")).toBeInTheDocument();
    const btn = screen.getByText("Reset Filters");
    fireEvent.click(btn);
    expect(handleAction).toHaveBeenCalledTimes(1);
  });

  it("renders Pagination and handles page changes", () => {
    const handlePageChange = vi.fn();
    render(
      <Pagination
        page={2}
        totalPages={5}
        totalItems={100}
        pageSize={20}
        onPageChange={handlePageChange}
      />,
    );
    expect(screen.getByText(/Page 2 of 5/)).toBeInTheDocument();
    const prevBtn = screen.getByLabelText("Previous page");
    fireEvent.click(prevBtn);
    expect(handlePageChange).toHaveBeenCalledWith(1);
  });
});

describe("Sidebar Component", () => {
  it("renders navigation items and fires onNavigate", () => {
    const onNavigate = vi.fn();
    render(
      <Sidebar
        currentView="overview"
        onNavigate={onNavigate}
        datasetHasData={true}
      />,
    );

    expect(screen.getByText("Overview")).toBeInTheDocument();
    expect(screen.getByText("Network Explorer")).toBeInTheDocument();
    expect(screen.getByText("Collaboration Lab")).toBeInTheDocument();
    expect(screen.getByText("Dataset Active")).toBeInTheDocument();

    fireEvent.click(screen.getByText("Network Explorer"));
    expect(onNavigate).toHaveBeenCalledWith("network");
  });
});
