import { ChevronLeft, ChevronRight } from "lucide-react";
import React from "react";

interface PaginationProps {
  page: number;
  totalPages: number;
  totalItems: number;
  pageSize: number;
  onPageChange: (newPage: number) => void;
}

export const Pagination: React.FC<PaginationProps> = ({
  page,
  totalPages,
  totalItems,
  pageSize,
  onPageChange,
}) => {
  if (totalPages <= 1) return null;

  const startItem = (page - 1) * pageSize + 1;
  const endItem = Math.min(page * pageSize, totalItems);

  return (
    <div className="flex flex-col sm:flex-row items-center justify-between gap-3 px-4 py-3 bg-slate-900/40 border-t border-slate-800 text-sm">
      <div className="text-slate-400 text-xs">
        Showing <span className="text-slate-200 font-medium">{startItem}</span>{" "}
        to <span className="text-slate-200 font-medium">{endItem}</span> of{" "}
        <span className="text-slate-200 font-medium">
          {totalItems.toLocaleString()}
        </span>{" "}
        entries
      </div>
      <div className="flex items-center gap-1.5">
        <button
          onClick={() => onPageChange(page - 1)}
          disabled={page <= 1}
          className="p-1.5 rounded-lg border border-slate-700 bg-slate-800/80 text-slate-300 hover:bg-slate-700 disabled:opacity-40 disabled:cursor-not-allowed transition"
          aria-label="Previous page"
        >
          <ChevronLeft className="w-4 h-4" />
        </button>
        <span className="px-3 py-1 rounded-md text-xs font-medium text-slate-300 bg-slate-800 border border-slate-700">
          Page {page} of {totalPages}
        </span>
        <button
          onClick={() => onPageChange(page + 1)}
          disabled={page >= totalPages}
          className="p-1.5 rounded-lg border border-slate-700 bg-slate-800/80 text-slate-300 hover:bg-slate-700 disabled:opacity-40 disabled:cursor-not-allowed transition"
          aria-label="Next page"
        >
          <ChevronRight className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
