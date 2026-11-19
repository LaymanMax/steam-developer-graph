import React from "react";

interface StatCardProps {
  label: string;
  value: string | number;
  subValue?: string;
  icon?: React.ReactNode;
  variant?: "blue" | "emerald" | "amber" | "purple" | "slate";
}

export const StatCard: React.FC<StatCardProps> = ({
  label,
  value,
  subValue,
  icon,
  variant = "slate",
}) => {
  const borderColors = {
    blue: "border-blue-500/20 hover:border-blue-500/40",
    emerald: "border-emerald-500/20 hover:border-emerald-500/40",
    amber: "border-amber-500/20 hover:border-amber-500/40",
    purple: "border-purple-500/20 hover:border-purple-500/40",
    slate: "border-slate-800 hover:border-slate-700",
  }[variant];

  const iconColors = {
    blue: "text-blue-400 bg-blue-500/10",
    emerald: "text-emerald-400 bg-emerald-500/10",
    amber: "text-amber-400 bg-amber-500/10",
    purple: "text-purple-400 bg-purple-500/10",
    slate: "text-slate-400 bg-slate-800",
  }[variant];

  return (
    <div
      className={`bg-slate-900/60 backdrop-blur-sm border ${borderColors} rounded-xl p-4 transition-all duration-200 shadow-sm`}
    >
      <div className="flex items-center justify-between mb-2">
        <span className="text-xs font-medium text-slate-400 tracking-wider uppercase">
          {label}
        </span>
        {icon && <div className={`p-1.5 rounded-lg ${iconColors}`}>{icon}</div>}
      </div>
      <div className="text-2xl font-bold text-slate-100 tracking-tight">
        {value}
      </div>
      {subValue && (
        <div className="text-xs text-slate-500 mt-1">{subValue}</div>
      )}
    </div>
  );
};
