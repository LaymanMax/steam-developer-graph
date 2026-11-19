import { ArrowUpDown, Building2, Download, Search } from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { EmptyState } from "../components/common/EmptyState";
import { Pagination } from "../components/common/Pagination";
import { PaginatedResponse, PublisherListItem } from "../types";
import { formatCompactNumber, formatNumber } from "../utils/formatters";

interface PublisherDirectoryViewProps {
  onSelectPublisher: (id: number) => void;
}

export const PublisherDirectoryView: React.FC<PublisherDirectoryViewProps> = ({
  onSelectPublisher,
}) => {
  const [data, setData] = useState<PaginatedResponse<PublisherListItem> | null>(
    null,
  );
  const [page, setPage] = useState(1);
  const [pageSize] = useState(25);
  const [search, setSearch] = useState("");
  const [genre, setGenre] = useState("");
  const [minGames, setMinGames] = useState<number | undefined>(undefined);
  const [sortBy, setSortBy] = useState("game_count");
  const [sortDesc, setSortDesc] = useState(true);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        const res = await api.listPublishers({
          page,
          pageSize,
          search: search.trim() || undefined,
          genre: genre.trim() || undefined,
          minGames,
          sortBy,
          sortDesc,
        });
        setData(res);
      } catch (err) {
        console.error("Failed to load publishers:", err);
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, [page, pageSize, search, genre, minGames, sortBy, sortDesc]);

  const handleSort = (column: string) => {
    if (sortBy === column) {
      setSortDesc(!sortDesc);
    } else {
      setSortBy(column);
      setSortDesc(true);
    }
  };

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-slate-800/80 pb-5">
        <div>
          <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
            <Building2 className="w-5 h-5 text-emerald-400" />
            <span>Publisher Directory</span>
          </h1>
          <p className="text-xs text-slate-400 mt-0.5">
            Directory of publishing businesses, distribution labels, and
            portfolio diversification metrics
          </p>
        </div>

        <a
          href={api.getExportPublishersUrl(
            search || undefined,
            genre || undefined,
            minGames,
          )}
          download="steam_publishers.csv"
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800/80 hover:bg-slate-700 text-xs font-medium text-slate-200 transition"
        >
          <Download className="w-3.5 h-3.5" />
          <span>Export Directory (CSV)</span>
        </a>
      </div>

      {/* Filter Row */}
      <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-3 flex flex-wrap items-center gap-3 text-xs">
        <div className="relative flex-1 min-w-[200px] max-w-xs">
          <Search className="w-3.5 h-3.5 absolute left-3 top-2.5 text-slate-500" />
          <input
            type="text"
            placeholder="Search publisher names..."
            value={search}
            onChange={(e) => {
              setSearch(e.target.value);
              setPage(1);
            }}
            className="w-full pl-8 pr-3 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500"
          />
        </div>

        <div className="flex items-center gap-2">
          <span className="text-slate-400">Genre:</span>
          <input
            type="text"
            placeholder="e.g. RPG"
            value={genre}
            onChange={(e) => {
              setGenre(e.target.value);
              setPage(1);
            }}
            className="w-28 px-2 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500"
          />
        </div>

        <div className="flex items-center gap-2">
          <span className="text-slate-400">Min Games:</span>
          <select
            value={minGames || ""}
            onChange={(e) => {
              setMinGames(
                e.target.value ? parseInt(e.target.value) : undefined,
              );
              setPage(1);
            }}
            className="px-2 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-100 focus:outline-none focus:border-blue-500"
          >
            <option value="">All</option>
            <option value="2">2+ games</option>
            <option value="5">5+ games</option>
            <option value="15">15+ games</option>
          </select>
        </div>
      </div>

      {/* Table */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm">
        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs text-slate-300">
            <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
              <tr>
                <th
                  onClick={() => handleSort("canonical_name")}
                  className="px-4 py-3 cursor-pointer hover:text-slate-200"
                >
                  <div className="flex items-center gap-1.5">
                    <span>Publisher Name</span>
                    <ArrowUpDown className="w-3 h-3 text-slate-500" />
                  </div>
                </th>
                <th
                  onClick={() => handleSort("game_count")}
                  className="px-4 py-3 cursor-pointer hover:text-slate-200 text-right"
                >
                  <div className="flex items-center justify-end gap-1.5">
                    <span>Portfolio</span>
                    <ArrowUpDown className="w-3 h-3 text-slate-500" />
                  </div>
                </th>
                <th
                  onClick={() => handleSort("developer_count")}
                  className="px-4 py-3 cursor-pointer hover:text-slate-200 text-right"
                >
                  <div className="flex items-center justify-end gap-1.5">
                    <span>Distinct Devs</span>
                    <ArrowUpDown className="w-3 h-3 text-slate-500" />
                  </div>
                </th>
                <th
                  onClick={() => handleSort("entropy")}
                  className="px-4 py-3 cursor-pointer hover:text-slate-200 text-right"
                >
                  <div className="flex items-center justify-end gap-1.5">
                    <span>Genre Diversity (H)</span>
                    <ArrowUpDown className="w-3 h-3 text-slate-500" />
                  </div>
                </th>
                <th className="px-4 py-3 text-right">Years Active</th>
                <th
                  onClick={() => handleSort("ownership")}
                  className="px-4 py-3 cursor-pointer hover:text-slate-200 text-right"
                >
                  <div className="flex items-center justify-end gap-1.5">
                    <span>Est. Owners</span>
                    <ArrowUpDown className="w-3 h-3 text-slate-500" />
                  </div>
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60">
              {isLoading ? (
                <tr>
                  <td colSpan={6} className="py-12 text-center text-slate-500">
                    Loading publishers...
                  </td>
                </tr>
              ) : !data || data.items.length === 0 ? (
                <tr>
                  <td colSpan={6} className="py-12">
                    <EmptyState
                      title="No publishers match your filters"
                      description="Try adjusting your search query, genre, or minimum game count threshold."
                    />
                  </td>
                </tr>
              ) : (
                data.items.map((pub) => (
                  <tr
                    key={pub.id}
                    onClick={() => onSelectPublisher(pub.id)}
                    className="hover:bg-slate-800/40 cursor-pointer transition"
                  >
                    <td className="px-4 py-3 font-medium text-slate-100 hover:text-emerald-400">
                      {pub.canonical_name}
                    </td>
                    <td className="px-4 py-3 text-right font-semibold text-emerald-400">
                      {formatNumber(pub.game_count)}
                    </td>
                    <td className="px-4 py-3 text-right text-slate-300">
                      {pub.developer_count}
                    </td>
                    <td className="px-4 py-3 text-right text-amber-400 font-mono">
                      {pub.shannon_entropy > 0
                        ? pub.shannon_entropy.toFixed(2)
                        : "0.00"}
                    </td>
                    <td className="px-4 py-3 text-right text-slate-400">
                      {pub.earliest_release_year && pub.latest_release_year
                        ? `${pub.earliest_release_year} – ${pub.latest_release_year}`
                        : "N/A"}
                    </td>
                    <td className="px-4 py-3 text-right font-mono text-slate-400 text-[11px]">
                      ~{formatCompactNumber(pub.total_owner_midpoint)}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        {data && (
          <Pagination
            page={page}
            totalPages={data.total_pages}
            totalItems={data.total}
            pageSize={pageSize}
            onPageChange={setPage}
          />
        )}
      </div>
    </div>
  );
};
