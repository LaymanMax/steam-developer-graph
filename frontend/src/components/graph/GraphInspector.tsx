import {
  ArrowRight,
  ExternalLink,
  Gamepad2,
  PlusCircle,
  X,
} from "lucide-react";
import React from "react";
import { GraphLink, GraphNode } from "../../types";
import { formatCompactNumber, formatNumber } from "../../utils/formatters";
import { Badge } from "../common/Badge";

interface GraphInspectorProps {
  selectedNode: GraphNode | null;
  selectedLink: GraphLink | null;
  onClose: () => void;
  onExpandNode?: (node: GraphNode) => void;
  onNavigateToEntity?: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const GraphInspector: React.FC<GraphInspectorProps> = ({
  selectedNode,
  selectedLink,
  onClose,
  onExpandNode,
  onNavigateToEntity,
}) => {
  if (!selectedNode && !selectedLink) return null;

  return (
    <div className="w-80 bg-slate-900/95 backdrop-blur-md border border-slate-800 rounded-xl shadow-2xl p-4 text-xs space-y-3 pointer-events-auto max-h-[85vh] overflow-y-auto">
      <div className="flex items-center justify-between border-b border-slate-800 pb-2">
        <span className="font-semibold text-slate-300 tracking-wider uppercase text-[10px]">
          {selectedNode ? "Node Details" : "Relationship Evidence"}
        </span>
        <button
          onClick={onClose}
          className="p-1 rounded hover:bg-slate-800 text-slate-400 hover:text-slate-200 transition"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>

      {selectedNode && (
        <div className="space-y-3">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <Badge variant={selectedNode.node_type}>
                {selectedNode.node_type.toUpperCase()}
              </Badge>
              {selectedNode.year && (
                <span className="text-[11px] text-slate-400">
                  Year {selectedNode.year}
                </span>
              )}
            </div>
            <h4 className="text-base font-bold text-slate-100 leading-snug">
              {selectedNode.label}
            </h4>
            {selectedNode.sub_label && (
              <p className="text-slate-400 text-xs mt-0.5">
                {selectedNode.sub_label}
              </p>
            )}
          </div>

          <div className="bg-slate-950/60 p-2.5 rounded-lg border border-slate-800/80 space-y-1.5">
            <div className="flex justify-between text-slate-400">
              <span>Metric:</span>
              <span className="text-slate-200 font-medium">
                {selectedNode.node_type === "game"
                  ? `~${formatCompactNumber(selectedNode.metric)} owners`
                  : `${formatNumber(selectedNode.metric)} games`}
              </span>
            </div>
            <div className="flex justify-between text-slate-400">
              <span>Internal ID:</span>
              <span className="text-slate-200 font-mono text-[11px]">
                {selectedNode.id}
              </span>
            </div>
          </div>

          <div className="flex flex-col gap-2 pt-1">
            {onExpandNode && (
              <button
                onClick={() => onExpandNode(selectedNode)}
                className="w-full flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg bg-blue-600/20 hover:bg-blue-600/30 text-blue-400 border border-blue-500/30 font-medium transition"
              >
                <PlusCircle className="w-3.5 h-3.5" />
                <span>Expand Connections</span>
              </button>
            )}

            {onNavigateToEntity && (
              <button
                onClick={() =>
                  onNavigateToEntity(
                    selectedNode.node_type,
                    selectedNode.entity_id,
                  )
                }
                className="w-full flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 font-medium transition"
              >
                <span>View Full Profile</span>
                <ExternalLink className="w-3.5 h-3.5" />
              </button>
            )}
          </div>
        </div>
      )}

      {selectedLink && (
        <div className="space-y-3">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <Badge variant="neutral">RELATIONSHIP</Badge>
              <span className="text-slate-400 font-medium capitalize">
                {selectedLink.relationship}
              </span>
            </div>
            <div className="flex items-center gap-1.5 text-slate-300 font-medium text-xs mt-1">
              <span className="truncate">
                {typeof selectedLink.source === "object"
                  ? selectedLink.source.label
                  : selectedLink.source}
              </span>
              <ArrowRight className="w-3.5 h-3.5 text-slate-500 shrink-0" />
              <span className="truncate">
                {typeof selectedLink.target === "object"
                  ? selectedLink.target.label
                  : selectedLink.target}
              </span>
            </div>
          </div>

          <div className="bg-slate-950/60 p-2.5 rounded-lg border border-slate-800/80 space-y-1">
            <div className="flex justify-between text-slate-400">
              <span>Shared Game Credits:</span>
              <span className="text-emerald-400 font-semibold">
                {selectedLink.shared_game_count}
              </span>
            </div>
          </div>

          {selectedLink.shared_games &&
            selectedLink.shared_games.length > 0 && (
              <div className="space-y-1.5">
                <span className="text-slate-400 font-medium text-[11px] block">
                  Supporting Titles:
                </span>
                <div className="max-h-40 overflow-y-auto space-y-1">
                  {selectedLink.shared_games.map((title, i) => (
                    <div
                      key={i}
                      className="flex items-center gap-2 px-2 py-1.5 rounded bg-slate-800/60 text-slate-300 text-xs"
                    >
                      <Gamepad2 className="w-3 h-3 text-amber-400 shrink-0" />
                      <span className="truncate">{title}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}
        </div>
      )}
    </div>
  );
};
