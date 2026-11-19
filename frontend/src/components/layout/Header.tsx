import { Activity, Database } from "lucide-react";
import React from "react";
import { GlobalSearch } from "./GlobalSearch";
import { ViewType } from "./Sidebar";

interface HeaderProps {
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
  onNavigate: (view: ViewType) => void;
  datasetHasData: boolean;
}

export const Header: React.FC<HeaderProps> = ({
  onSelectEntity,
  onNavigate,
  datasetHasData,
}) => {
  return (
    <header className="h-16 bg-slate-950/80 backdrop-blur-md border-b border-slate-800/80 px-6 flex items-center justify-between gap-4 z-40">
      {/* Search Bar */}
      <div className="flex-1 max-w-lg">
        <GlobalSearch onSelectEntity={onSelectEntity} />
      </div>

      {/* Right controls */}
      <div className="flex items-center gap-3">
        <button
          onClick={() => onNavigate("dataset")}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-slate-700/80 bg-slate-900/80 hover:bg-slate-800 text-xs font-medium text-slate-300 transition"
        >
          <Database className="w-3.5 h-3.5 text-blue-400" />
          <span>Dataset Status</span>
        </button>

        <button
          onClick={() => onNavigate("resolution")}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-slate-700/80 bg-slate-900/80 hover:bg-slate-800 text-xs font-medium text-slate-300 transition"
        >
          <Activity className="w-3.5 h-3.5 text-emerald-400" />
          <span>Entity Resolution</span>
        </button>
      </div>
    </header>
  );
};
