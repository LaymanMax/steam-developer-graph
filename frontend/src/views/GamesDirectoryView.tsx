import {
  AlertTriangle,
  ExternalLink,
  Gamepad2,
  Search,
  ShieldCheck,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { EmptyState } from "../components/common/EmptyState";
import { Pagination } from "../components/common/Pagination";
import { GameSummaryItem } from "../types";
import {
  formatNumber,
  formatOwnerRange,
  formatPrice,
} from "../utils/formatters";

interface GamesDirectoryViewProps {
  onSelectGame: (appId: number) => void;
  onSelectDeveloper?: (id: number) => void;
  onSelectPublisher?: (id: number) => void;
}

export const GamesDirectoryView: React.FC<GamesDirectoryViewProps> = ({
  onSelectGame,
}) => {
  const [games, setGames] = useState<GameSummaryItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [totalCount, setTotalCount] = useState(0);
  const [excludePlaytests, setExcludePlaytests] = useState(false);
  const [excludeSparse, setExcludeSparse] = useState(false);

  useEffect(() => {
    loadGames();
  }, [page, search, excludePlaytests, excludeSparse]);

  const loadGames = async () => {
    try {
      setLoading(true);
      setError(null);
      const res = await api.listGames({
        page,
        pageSize: 25,
        search: search.trim() || undefined,
        excludePlaytests,
        excludeSparse,
      });
      setGames(res.items);
      setTotalPages(res.total_pages);
      setTotalCount(res.total);
    } catch (err: any) {
      setError(err.message || "Failed to load games");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-100 flex items-center gap-2.5">
            <Gamepad2 className="w-6 h-6 text-cyan-400" />
            <span>Games Catalog</span>
          </h1>
          <p className="text-sm text-slate-400 mt-1">
            Browse published Steam titles, inspect ownership brackets, and
            explore developer/publisher links.
          </p>
        </div>
      </div>

      {/* Filter Toolbar */}
      <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 flex flex-col md:flex-row gap-4 items-center justify-between">
        <div className="relative w-full md:w-80">
          <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
          <input
            type="text"
            placeholder="Search by game title..."
            value={search}
            onChange={(e) => {
              setSearch(e.target.value);
              setPage(1);
            }}
            className="w-full bg-slate-950/80 border border-slate-800 rounded-lg pl-9 pr-3 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500"
          />
        </div>

        <div className="flex flex-wrap items-center gap-4 text-xs text-slate-400 w-full md:w-auto">
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={!excludePlaytests}
              onChange={(e) => {
                setExcludePlaytests(!e.target.checked);
                setPage(1);
              }}
              className="rounded border-slate-700 bg-slate-900 text-cyan-500 focus:ring-0"
            />
            <span>Include Playtests</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={!excludeSparse}
              onChange={(e) => {
                setExcludeSparse(!e.target.checked);
                setPage(1);
              }}
              className="rounded border-slate-700 bg-slate-900 text-cyan-500 focus:ring-0"
            />
            <span>Include Sparse Records</span>
          </label>

          <span className="text-slate-600">|</span>
          <span className="font-mono text-slate-300">
            {formatNumber(totalCount)} games found
          </span>
        </div>
      </div>

      {/* Table */}
      <div className="rounded-xl border border-slate-800 bg-slate-900/40 overflow-hidden">
        {loading ? (
          <div className="p-12 text-center text-sm text-slate-500 animate-pulse">
            Loading games catalog...
          </div>
        ) : error ? (
          <div className="p-12 text-center text-sm text-rose-400">{error}</div>
        ) : games.length === 0 ? (
          <EmptyState
            title="No games found"
            description="Try adjusting your search criteria or enabling playtests and sparse records."
          />
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs">
              <thead className="bg-slate-950/60 border-b border-slate-800 text-slate-400 font-medium uppercase tracking-wider text-[10px]">
                <tr>
                  <th className="py-3 px-4">Title & AppID</th>
                  <th className="py-3 px-4">Release Date</th>
                  <th className="py-3 px-4">Estimated Owners</th>
                  <th className="py-3 px-4">Price</th>
                  <th className="py-3 px-4">Data Quality</th>
                  <th className="py-3 px-4 text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60">
                {games.map((g) => (
                  <tr
                    key={g.id}
                    onClick={() => onSelectGame(g.app_id)}
                    className="hover:bg-slate-800/40 cursor-pointer transition"
                  >
                    <td className="py-3 px-4">
                      <div className="font-medium text-slate-200 hover:text-cyan-400 transition">
                        {g.name}
                      </div>
                      <div className="text-[11px] font-mono text-slate-500">
                        AppID: {g.app_id}
                      </div>
                    </td>
                    <td className="py-3 px-4 text-slate-300 font-mono">
                      {g.release_date ||
                        (g.release_year ? `${g.release_year}` : "Unknown")}
                    </td>
                    <td className="py-3 px-4">
                      <div className="font-mono text-slate-200">
                        {formatOwnerRange(g.owner_lower, g.owner_upper)}
                      </div>
                      <div className="text-[10px] text-slate-500">
                        Midpoint: ~{formatNumber(g.owner_midpoint)}
                      </div>
                    </td>
                    <td className="py-3 px-4">
                      <div className="text-slate-300 font-mono">
                        {formatPrice(g.price)}
                      </div>
                    </td>
                    <td className="py-3 px-4">
                      <div className="flex flex-wrap gap-1">
                        {g.is_likely_playtest && (
                          <Badge variant="warning" size="sm">
                            <AlertTriangle className="w-3 h-3 mr-1" />
                            Playtest
                          </Badge>
                        )}
                        {g.is_sparse && (
                          <Badge variant="neutral" size="sm">
                            Sparse
                          </Badge>
                        )}
                        {!g.is_likely_playtest && !g.is_sparse && (
                          <Badge variant="success" size="sm">
                            <ShieldCheck className="w-3 h-3 mr-1" />
                            Valid
                          </Badge>
                        )}
                      </div>
                    </td>
                    <td className="py-3 px-4 text-right">
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          onSelectGame(g.app_id);
                        }}
                        className="p-1.5 rounded-lg border border-slate-700/60 bg-slate-800/80 hover:bg-cyan-500/20 hover:border-cyan-500/40 text-slate-300 hover:text-cyan-400 transition"
                      >
                        <ExternalLink className="w-3.5 h-3.5" />
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {/* Pagination */}
        <div className="p-4 border-t border-slate-800 bg-slate-950/40">
          <Pagination
            page={page}
            totalPages={totalPages}
            totalItems={totalCount}
            pageSize={25}
            onPageChange={setPage}
          />
        </div>
      </div>
    </div>
  );
};
