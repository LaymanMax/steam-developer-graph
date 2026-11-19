import { Download, RotateCcw, Share2, SlidersHorizontal } from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { NetworkGraph } from "../components/graph/NetworkGraph";
import { ViewType } from "../components/layout/Sidebar";
import { GraphNode, Subgraph } from "../types";

interface NetworkExplorerViewProps {
  onNavigate: (view: ViewType) => void;
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
  initialEntity?: {
    type: "developer" | "publisher" | "game";
    id: number;
  } | null;
}

export const NetworkExplorerView: React.FC<NetworkExplorerViewProps> = ({
  onNavigate,
  onSelectEntity,
  initialEntity,
}) => {
  const [graphData, setGraphData] = useState<Subgraph | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Filters state
  const [minYear, setMinYear] = useState<number | undefined>(undefined);
  const [maxYear, setMaxYear] = useState<number | undefined>(undefined);
  const [genre, setGenre] = useState<string>("");
  const [minCollabs, setMinCollabs] = useState<number>(1);
  const [excludeSparse, setExcludeSparse] = useState<boolean>(true);
  const [excludePlaytests, setExcludePlaytests] = useState<boolean>(true);
  const [nodeLimit, setNodeLimit] = useState<number>(90);
  const [showFilters, setShowFilters] = useState<boolean>(false);

  // Load graph data (either focused on initialEntity or ecosystem overview)
  const loadGraph = async () => {
    try {
      setIsLoading(true);
      setError(null);

      let data: Subgraph;
      if (initialEntity) {
        data = await api.queryNeighborhood({
          nodeType: initialEntity.type,
          id: initialEntity.id,
          depth: 1,
          minYear,
          maxYear,
          genre: genre || undefined,
          minCollaborations: minCollabs,
          excludeSparse,
          excludePlaytests,
          limit: nodeLimit,
        });
      } else {
        data = await api.getGraphOverview({
          limit: nodeLimit,
          minYear,
          maxYear,
        });
      }

      setGraphData(data);
    } catch (err: any) {
      setError(err.message || "Failed to load graph data");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadGraph();
  }, [
    initialEntity,
    minYear,
    maxYear,
    genre,
    minCollabs,
    excludeSparse,
    excludePlaytests,
    nodeLimit,
  ]);

  // Expand node handler
  const handleExpandNode = async (node: GraphNode) => {
    if (!graphData) return;
    try {
      const neighborData = await api.queryNeighborhood({
        nodeType: node.node_type,
        id: node.entity_id,
        depth: 1,
        minYear,
        maxYear,
        genre: genre || undefined,
        limit: 30,
      });

      // Merge new nodes and links without duplicates
      const existingNodeIds = new Set(graphData.nodes.map((n) => n.id));
      const newNodes = neighborData.nodes.filter(
        (n) => !existingNodeIds.has(n.id),
      );

      const existingLinkKeys = new Set(
        graphData.links.map((l) => {
          const s = typeof l.source === "object" ? l.source.id : l.source;
          const t = typeof l.target === "object" ? l.target.id : l.target;
          return `${s}->${t}`;
        }),
      );

      const newLinks = neighborData.links.filter((l) => {
        const s = typeof l.source === "object" ? l.source.id : l.source;
        const t = typeof l.target === "object" ? l.target.id : l.target;
        return !existingLinkKeys.has(`${s}->${t}`);
      });

      setGraphData({
        ...graphData,
        nodes: [...graphData.nodes, ...newNodes],
        links: [...graphData.links, ...newLinks],
        total_nodes: graphData.nodes.length + newNodes.length,
        total_links: graphData.links.length + newLinks.length,
      });
    } catch (err: any) {
      console.error("Failed to expand node:", err);
    }
  };

  const handleReset = () => {
    setMinYear(undefined);
    setMaxYear(undefined);
    setGenre("");
    setMinCollabs(1);
    setExcludeSparse(true);
    setExcludePlaytests(true);
    setNodeLimit(90);
  };

  return (
    <div className="relative w-full h-[calc(100vh-4rem)] flex flex-col bg-slate-950 overflow-hidden">
      {/* Top Explorer Filter & Action Bar */}
      <div className="h-12 bg-slate-900/90 backdrop-blur-md border-b border-slate-800 px-5 flex items-center justify-between gap-4 z-20 shrink-0 text-xs">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2 text-slate-300 font-semibold tracking-wide">
            <Share2 className="w-4 h-4 text-blue-400" />
            <span>Interactive Network Explorer</span>
          </div>

          {graphData && (
            <span className="text-slate-400 text-[11px] bg-slate-800/80 px-2.5 py-0.5 rounded-full border border-slate-700/60">
              {graphData.nodes.length} nodes • {graphData.links.length} links
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowFilters(!showFilters)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg border text-xs font-medium transition ${
              showFilters
                ? "bg-blue-600/20 border-blue-500/40 text-blue-400"
                : "bg-slate-800/80 border-slate-700 text-slate-300 hover:bg-slate-700"
            }`}
          >
            <SlidersHorizontal className="w-3.5 h-3.5" />
            <span>Filter Graph</span>
          </button>

          <button
            onClick={handleReset}
            className="p-1.5 rounded-lg border border-slate-700 bg-slate-800/80 text-slate-400 hover:text-slate-200 hover:bg-slate-700 transition"
            title="Reset Graph Filters"
          >
            <RotateCcw className="w-3.5 h-3.5" />
          </button>

          <a
            href={api.getExportGraphUrl(initialEntity?.type, initialEntity?.id)}
            download="steam_network_subgraph.json"
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800/80 text-slate-300 hover:bg-slate-700 transition font-medium"
            title="Export Graph to JSON"
          >
            <Download className="w-3.5 h-3.5" />
            <span>Export JSON</span>
          </a>
        </div>
      </div>

      {/* Expandable Filter Drawer */}
      {showFilters && (
        <div className="bg-slate-900/95 border-b border-slate-800 px-6 py-3 flex flex-wrap items-center gap-6 z-20 text-xs shadow-xl">
          <div className="flex items-center gap-2">
            <span className="text-slate-400 font-medium">Release Year:</span>
            <input
              type="number"
              placeholder="Min"
              value={minYear || ""}
              onChange={(e) =>
                setMinYear(
                  e.target.value ? parseInt(e.target.value) : undefined,
                )
              }
              className="w-20 px-2 py-1 bg-slate-950 border border-slate-700 rounded text-slate-200 focus:outline-none focus:border-blue-500"
            />
            <span className="text-slate-500">-</span>
            <input
              type="number"
              placeholder="Max"
              value={maxYear || ""}
              onChange={(e) =>
                setMaxYear(
                  e.target.value ? parseInt(e.target.value) : undefined,
                )
              }
              className="w-20 px-2 py-1 bg-slate-950 border border-slate-700 rounded text-slate-200 focus:outline-none focus:border-blue-500"
            />
          </div>

          <div className="flex items-center gap-2">
            <span className="text-slate-400 font-medium">Genre:</span>
            <input
              type="text"
              placeholder="e.g. Action, RPG"
              value={genre}
              onChange={(e) => setGenre(e.target.value)}
              className="w-32 px-2 py-1 bg-slate-950 border border-slate-700 rounded text-slate-200 focus:outline-none focus:border-blue-500"
            />
          </div>

          <div className="flex items-center gap-2">
            <span className="text-slate-400 font-medium">Node Budget:</span>
            <select
              value={nodeLimit}
              onChange={(e) => setNodeLimit(parseInt(e.target.value))}
              className="px-2 py-1 bg-slate-950 border border-slate-700 rounded text-slate-200 focus:outline-none focus:border-blue-500"
            >
              <option value="50">50 nodes (Fast)</option>
              <option value="90">90 nodes (Balanced)</option>
              <option value="150">150 nodes (Deep)</option>
            </select>
          </div>

          <label className="flex items-center gap-1.5 text-slate-300 cursor-pointer">
            <input
              type="checkbox"
              checked={excludeSparse}
              onChange={(e) => setExcludeSparse(e.target.checked)}
              className="rounded bg-slate-950 border-slate-700 text-blue-600 focus:ring-0"
            />
            <span>Exclude Sparse</span>
          </label>

          <label className="flex items-center gap-1.5 text-slate-300 cursor-pointer">
            <input
              type="checkbox"
              checked={excludePlaytests}
              onChange={(e) => setExcludePlaytests(e.target.checked)}
              className="rounded bg-slate-950 border-slate-700 text-blue-600 focus:ring-0"
            />
            <span>Exclude Playtests</span>
          </label>
        </div>
      )}

      {/* Main Graph Canvas */}
      <div className="flex-1 w-full h-full relative">
        {isLoading && (
          <div className="absolute inset-0 bg-slate-950/80 backdrop-blur-sm z-30 flex items-center justify-center text-slate-400 text-sm">
            Calculating force-directed network layout...
          </div>
        )}

        {error ? (
          <div className="flex items-center justify-center h-full p-8 text-center">
            <div className="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm max-w-md">
              {error}
            </div>
          </div>
        ) : graphData && graphData.nodes.length > 0 ? (
          <NetworkGraph
            data={graphData}
            onExpandNode={handleExpandNode}
            onNavigateToEntity={onSelectEntity}
          />
        ) : (
          !isLoading && (
            <div className="flex items-center justify-center h-full text-slate-500 text-sm">
              No matching nodes found for current filter criteria.
            </div>
          )
        )}
      </div>
    </div>
  );
};
