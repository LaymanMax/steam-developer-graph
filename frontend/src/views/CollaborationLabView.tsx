import { Download, Gamepad2, Users2 } from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { EmptyState } from "../components/common/EmptyState";
import { DeveloperCollaboration, PublisherDeveloperRelation } from "../types";

interface CollaborationLabViewProps {
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const CollaborationLabView: React.FC<CollaborationLabViewProps> = ({
  onSelectEntity,
}) => {
  const [collabs, setCollabs] = useState<DeveloperCollaboration[]>([]);
  const [pubRelations, setPubRelations] = useState<
    PublisherDeveloperRelation[]
  >([]);
  const [activeTab, setActiveTab] = useState<"dev-dev" | "pub-dev">("dev-dev");
  const [minShared, setMinShared] = useState<number>(2);
  const [genre, setGenre] = useState<string>("");
  const [minYear, setMinYear] = useState<number | undefined>(undefined);
  const [maxYear, setMaxYear] = useState<number | undefined>(undefined);
  const [selectedCollab, setSelectedCollab] =
    useState<DeveloperCollaboration | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        if (activeTab === "dev-dev") {
          const data = await api.getTopCollaborations({
            minShared,
            genre: genre.trim() || undefined,
            minYear,
            maxYear,
            limit: 50,
          });
          setCollabs(data);
          if (data.length > 0 && !selectedCollab) {
            setSelectedCollab(data[0]);
          }
        } else {
          const data = await api.getPublisherDeveloperRelations({
            minShared,
            limit: 50,
          });
          setPubRelations(data);
        }
      } catch (err) {
        console.error("Failed to load collaborations:", err);
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, [activeTab, minShared, genre, minYear, maxYear]);

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-slate-800 pb-5">
        <div>
          <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
            <Users2 className="w-5 h-5 text-purple-400" />
            <span>Developer Collaboration Lab</span>
          </h1>
          <p className="text-xs text-slate-400 mt-0.5">
            Identify co-development studios, repeat collaborations, and
            multi-publisher working relationships
          </p>
        </div>

        <a
          href={api.getExportCollaborationsUrl(
            minShared,
            genre || undefined,
            minYear,
            maxYear,
          )}
          download="steam_developer_collaborations.csv"
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800/80 hover:bg-slate-700 text-xs font-medium text-slate-200 transition"
        >
          <Download className="w-3.5 h-3.5" />
          <span>Export Collaborations (CSV)</span>
        </a>
      </div>

      {/* Tabs & Filters */}
      <div className="flex flex-wrap items-center justify-between gap-4 bg-slate-900/60 border border-slate-800/80 p-3 rounded-xl text-xs">
        {/* Toggle Mode */}
        <div className="flex items-center gap-1 bg-slate-950 p-1 rounded-lg border border-slate-800">
          <button
            onClick={() => setActiveTab("dev-dev")}
            className={`px-3 py-1.5 rounded-md font-medium transition ${
              activeTab === "dev-dev"
                ? "bg-purple-600/20 text-purple-300 border border-purple-500/30"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            Developer ↔ Developer Pairs
          </button>
          <button
            onClick={() => setActiveTab("pub-dev")}
            className={`px-3 py-1.5 rounded-md font-medium transition ${
              activeTab === "pub-dev"
                ? "bg-blue-600/20 text-blue-300 border border-blue-500/30"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            Publisher ↔ Developer Relationships
          </button>
        </div>

        {/* Filter inputs */}
        <div className="flex flex-wrap items-center gap-3">
          <div className="flex items-center gap-2">
            <span className="text-slate-400">Min Shared Games:</span>
            <select
              value={minShared}
              onChange={(e) => setMinShared(parseInt(e.target.value))}
              className="px-2 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-100 focus:outline-none focus:border-blue-500"
            >
              <option value="1">1+ shared game</option>
              <option value="2">2+ repeated collaborations</option>
              <option value="3">3+ strong partnerships</option>
              <option value="5">5+ frequent collaborations</option>
            </select>
          </div>

          <div className="flex items-center gap-2">
            <span className="text-slate-400">Genre:</span>
            <input
              type="text"
              placeholder="Filter genre..."
              value={genre}
              onChange={(e) => setGenre(e.target.value)}
              className="w-28 px-2 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Main Content Area */}
      {activeTab === "dev-dev" ? (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* List of Collaboration Pairs */}
          <div className="lg:col-span-2 bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm">
            <div className="p-4 border-b border-slate-800 flex items-center justify-between">
              <span className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
                Ranked Developer Collaborations ({collabs.length})
              </span>
            </div>

            <div className="overflow-x-auto">
              <table className="w-full text-left text-xs text-slate-300">
                <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
                  <tr>
                    <th className="px-4 py-3">Developer Studio 1</th>
                    <th className="px-4 py-3">Developer Studio 2</th>
                    <th className="px-4 py-3 text-right">Shared Games</th>
                    <th className="px-4 py-3 text-right">Years</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-800/60">
                  {isLoading ? (
                    <tr>
                      <td
                        colSpan={4}
                        className="py-12 text-center text-slate-500"
                      >
                        Analyzing developer collaborations...
                      </td>
                    </tr>
                  ) : collabs.length === 0 ? (
                    <tr>
                      <td colSpan={4} className="py-12">
                        <EmptyState
                          title="No developer collaborations found"
                          description="Try reducing the minimum shared games threshold or changing the genre filter."
                        />
                      </td>
                    </tr>
                  ) : (
                    collabs.map((c) => {
                      const isSelected =
                        selectedCollab?.dev1_id === c.dev1_id &&
                        selectedCollab?.dev2_id === c.dev2_id;

                      return (
                        <tr
                          key={`${c.dev1_id}-${c.dev2_id}`}
                          onClick={() => setSelectedCollab(c)}
                          className={`cursor-pointer transition ${
                            isSelected
                              ? "bg-purple-600/15 border-l-2 border-purple-500"
                              : "hover:bg-slate-800/40"
                          }`}
                        >
                          <td className="px-4 py-3 font-medium text-slate-200">
                            {c.dev1_name}
                          </td>
                          <td className="px-4 py-3 font-medium text-slate-200">
                            {c.dev2_name}
                          </td>
                          <td className="px-4 py-3 text-right font-semibold text-purple-400">
                            {c.shared_games_count}
                          </td>
                          <td className="px-4 py-3 text-right text-slate-400">
                            {c.first_year && c.latest_year
                              ? `${c.first_year} – ${c.latest_year}`
                              : "N/A"}
                          </td>
                        </tr>
                      );
                    })
                  )}
                </tbody>
              </table>
            </div>
          </div>

          {/* Side Inspector: Supporting Titles */}
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-5 space-y-4 h-fit">
            <h3 className="text-sm font-semibold text-slate-200 border-b border-slate-800 pb-2">
              Collaboration Evidence
            </h3>

            {selectedCollab ? (
              <div className="space-y-4 text-xs">
                <div>
                  <div className="flex items-center gap-1.5 text-slate-200 font-medium">
                    <span
                      onClick={() =>
                        onSelectEntity("developer", selectedCollab.dev1_id)
                      }
                      className="hover:text-blue-400 cursor-pointer underline"
                    >
                      {selectedCollab.dev1_name}
                    </span>
                    <span className="text-slate-500">&</span>
                    <span
                      onClick={() =>
                        onSelectEntity("developer", selectedCollab.dev2_id)
                      }
                      className="hover:text-blue-400 cursor-pointer underline"
                    >
                      {selectedCollab.dev2_name}
                    </span>
                  </div>
                  <p className="text-slate-400 text-[11px] mt-1">
                    Credited together on{" "}
                    <span className="text-purple-400 font-semibold">
                      {selectedCollab.shared_games_count}
                    </span>{" "}
                    different Steam titles
                  </p>
                </div>

                <div className="space-y-2">
                  <span className="text-[11px] font-semibold text-slate-400 uppercase tracking-wider block">
                    Shared Titles:
                  </span>
                  {selectedCollab.shared_games &&
                  selectedCollab.shared_games.length > 0 ? (
                    <div className="space-y-1.5 max-h-72 overflow-y-auto">
                      {selectedCollab.shared_games.map((g) => (
                        <div
                          key={g.game_id}
                          onClick={() => onSelectEntity("game", g.game_id)}
                          className="flex items-center justify-between p-2 rounded-lg bg-slate-950/80 border border-slate-800 hover:border-slate-700 cursor-pointer transition"
                        >
                          <div className="flex items-center gap-2 min-w-0">
                            <Gamepad2 className="w-3.5 h-3.5 text-amber-400 shrink-0" />
                            <span className="truncate text-slate-200 hover:text-amber-400 font-medium">
                              {g.name}
                            </span>
                          </div>
                          {g.release_year && (
                            <span className="text-[10px] text-slate-500 shrink-0">
                              {g.release_year}
                            </span>
                          )}
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-slate-500 italic">
                      Select a pair to view titles
                    </p>
                  )}
                </div>
              </div>
            ) : (
              <p className="text-slate-500 text-xs py-4 text-center">
                Click a collaboration pair to inspect evidence
              </p>
            )}
          </div>
        </div>
      ) : (
        /* Publisher ↔ Developer Tab */
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm">
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs text-slate-300">
              <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
                <tr>
                  <th className="px-4 py-3">Publisher</th>
                  <th className="px-4 py-3">Developer Studio</th>
                  <th className="px-4 py-3 text-right">Shared Games</th>
                  <th className="px-4 py-3 text-right">Timeline</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60">
                {pubRelations.map((rel, idx) => (
                  <tr
                    key={idx}
                    className="hover:bg-slate-800/40 cursor-pointer transition"
                  >
                    <td
                      onClick={() =>
                        onSelectEntity("publisher", rel.publisher_id)
                      }
                      className="px-4 py-3 font-medium text-emerald-400 hover:underline"
                    >
                      {rel.publisher_name}
                    </td>
                    <td
                      onClick={() =>
                        onSelectEntity("developer", rel.developer_id)
                      }
                      className="px-4 py-3 font-medium text-blue-400 hover:underline"
                    >
                      {rel.developer_name}
                    </td>
                    <td className="px-4 py-3 text-right font-semibold text-slate-200">
                      {rel.shared_games_count}
                    </td>
                    <td className="px-4 py-3 text-right text-slate-400">
                      {rel.first_year && rel.latest_year
                        ? `${rel.first_year} – ${rel.latest_year}`
                        : "N/A"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};
