import React from "react";

export const GraphLegend: React.FC = () => {
  return (
    <div className="bg-slate-900/90 backdrop-blur-md border border-slate-800 rounded-xl p-3 shadow-lg text-xs space-y-2 pointer-events-auto">
      <div className="text-[11px] font-semibold tracking-wider uppercase text-slate-400">
        Legend
      </div>
      <div className="flex flex-col gap-1.5">
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 rounded-full bg-blue-500 shadow-sm shadow-blue-500/50" />
          <span className="text-slate-300">Developer</span>
          <span className="text-[10px] text-slate-500 ml-auto">
            Size = games
          </span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 rounded-full bg-emerald-500 shadow-sm shadow-emerald-500/50" />
          <span className="text-slate-300">Publisher</span>
          <span className="text-[10px] text-slate-500 ml-auto">
            Size = catalog
          </span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-2.5 h-2.5 rounded-full bg-amber-500 shadow-sm shadow-amber-500/50" />
          <span className="text-slate-300">Game</span>
          <span className="text-[10px] text-slate-500 ml-auto">
            Size = owners
          </span>
        </div>
      </div>
      <div className="pt-2 border-t border-slate-800 text-[10px] text-slate-500 leading-tight">
        Click node to inspect & expand neighbors.
        <br />
        Click edge to view supporting games.
      </div>
    </div>
  );
};
