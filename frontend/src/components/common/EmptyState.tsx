import { FilterX } from "lucide-react";
import React from "react";

interface EmptyStateProps {
  title: string;
  description: string;
  icon?: React.ReactNode;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  description,
  icon,
  action,
}) => {
  return (
    <div className="flex flex-col items-center justify-center p-12 text-center bg-slate-900/30 rounded-xl border border-slate-800/80 my-4">
      <div className="p-3 bg-slate-800/80 rounded-2xl text-slate-400 mb-3 border border-slate-700/60 shadow-inner">
        {icon || <FilterX className="w-8 h-8 text-slate-400" />}
      </div>
      <h3 className="text-base font-semibold text-slate-200 mb-1">{title}</h3>
      <p className="text-sm text-slate-400 max-w-sm mb-4 leading-relaxed">
        {description}
      </p>
      {action && (
        <button
          onClick={action.onClick}
          className="px-4 py-2 text-xs font-medium rounded-lg bg-blue-600 hover:bg-blue-500 text-white shadow-sm transition"
        >
          {action.label}
        </button>
      )}
    </div>
  );
};
