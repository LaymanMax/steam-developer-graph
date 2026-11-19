import {
  Briefcase,
  Building2,
  Database,
  Gamepad2,
  GitMerge,
  LayoutDashboard,
  Share2,
  TrendingUp,
  Users2,
} from "lucide-react";
import React from "react";

export type ViewType =
  | "overview"
  | "network"
  | "developers"
  | "developer-profile"
  | "publishers"
  | "publisher-profile"
  | "games"
  | "game-detail"
  | "collaboration"
  | "historical"
  | "resolution"
  | "dataset";

interface SidebarProps {
  currentView: ViewType;
  onNavigate: (view: ViewType) => void;
  datasetHasData: boolean;
}

export const Sidebar: React.FC<SidebarProps> = ({
  currentView,
  onNavigate,
  datasetHasData,
}) => {
  const navItems: {
    id: ViewType;
    label: string;
    icon: any;
    highlight?: boolean;
  }[] = [
    { id: "overview", label: "Overview", icon: LayoutDashboard },
    { id: "network", label: "Network Explorer", icon: Share2, highlight: true },
    { id: "developers", label: "Developers", icon: Briefcase },
    { id: "publishers", label: "Publishers", icon: Building2 },
    { id: "games", label: "Games", icon: Gamepad2 },
    { id: "collaboration", label: "Collaboration Lab", icon: Users2 },
    { id: "historical", label: "Historical Analysis", icon: TrendingUp },
    { id: "resolution", label: "Entity Resolution", icon: GitMerge },
    { id: "dataset", label: "Dataset & Quality", icon: Database },
  ];

  return (
    <aside className="w-64 bg-slate-950/90 border-r border-slate-800/80 flex flex-col shrink-0 select-none">
      {/* Brand */}
      <div className="h-16 flex items-center px-5 border-b border-slate-800/80 gap-3">
        <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-blue-600 via-indigo-600 to-emerald-500 flex items-center justify-center text-white shadow-md shadow-blue-500/20">
          <Share2 className="w-4 h-4 stroke-[2.5]" />
        </div>
        <div>
          <span className="font-bold text-sm tracking-tight text-slate-100 block">
            Steam Network
          </span>
          <span className="text-[10px] text-slate-500 tracking-wider uppercase font-semibold block">
            Developer Graph
          </span>
        </div>
      </div>

      {/* Nav List */}
      <nav className="p-3 space-y-1 flex-1 overflow-y-auto">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive =
            currentView === item.id ||
            (item.id === "developers" && currentView === "developer-profile") ||
            (item.id === "publishers" && currentView === "publisher-profile") ||
            (item.id === "games" && currentView === "game-detail");

          return (
            <button
              key={item.id}
              onClick={() => onNavigate(item.id as ViewType)}
              className={`w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-medium transition-all ${
                isActive
                  ? "bg-blue-600/15 text-blue-400 border border-blue-500/30 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-900/80 border border-transparent"
              }`}
            >
              <Icon
                className={`w-4 h-4 ${
                  isActive
                    ? "text-blue-400"
                    : "text-slate-500 group-hover:text-slate-300"
                }`}
              />
              <span className="flex-1 text-left">{item.label}</span>
              {item.highlight && (
                <span className="w-1.5 h-1.5 rounded-full bg-blue-400 animate-pulse" />
              )}
            </button>
          );
        })}
      </nav>

      {/* System Footnote */}
      <div className="p-4 border-t border-slate-800/80 text-[11px] text-slate-500 space-y-1.5 bg-slate-950/40">
        <div className="flex items-center gap-2">
          <span
            className={`w-2 h-2 rounded-full ${
              datasetHasData ? "bg-emerald-400" : "bg-amber-400 animate-ping"
            }`}
          />
          <span className="text-slate-400 font-medium">
            {datasetHasData ? "Dataset Active" : "No Data Loaded"}
          </span>
        </div>
        <p className="text-[10px] text-slate-600 leading-tight">
          Observational network from Steam & SteamSpy metadata.
        </p>
      </div>
    </aside>
  );
};
