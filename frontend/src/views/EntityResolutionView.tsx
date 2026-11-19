import {
  Check,
  Gamepad2,
  GitMerge,
  History,
  Plus,
  RotateCcw,
  Sparkles,
  Split,
  X,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { EmptyState } from "../components/common/EmptyState";
import { Pagination } from "../components/common/Pagination";
import {
  MergeHistoryItem,
  PaginatedResponse,
  ResolutionCandidateItem,
  ResolutionReport,
} from "../types";
import { formatNumber } from "../utils/formatters";

export const EntityResolutionView: React.FC = () => {
  const [report, setReport] = useState<ResolutionReport | null>(null);
  const [candidates, setCandidates] =
    useState<PaginatedResponse<ResolutionCandidateItem> | null>(null);
  const [history, setHistory] = useState<MergeHistoryItem[]>([]);
  const [activeTab, setActiveTab] = useState<"review" | "history" | "manual">(
    "review",
  );

  // Filters
  const [entityType, setEntityType] = useState<string>("developer");
  const [status, setStatus] = useState<string>("pending");
  const [minSimilarity, setMinSimilarity] = useState<number>(0.85);
  const [page, setPage] = useState<number>(1);
  const [pageSize] = useState<number>(10);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [actionMessage, setActionMessage] = useState<string | null>(null);

  // Manual alias form state
  const [manualType, setManualType] = useState<string>("developer");
  const [manualRawName, setManualRawName] = useState<string>("");
  const [manualCanonicalId, setManualCanonicalId] = useState<string>("");

  const loadData = async () => {
    try {
      setIsLoading(true);
      const [repData, candData, histData] = await Promise.all([
        api.getResolutionReport(),
        api.listCandidates({
          entityType,
          status,
          minSimilarity,
          page,
          pageSize,
        }),
        api.getMergeHistory(),
      ]);
      setReport(repData);
      setCandidates(candData);
      setHistory(histData);
    } catch (err) {
      console.error("Failed to load resolution data:", err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, [entityType, status, minSimilarity, page, pageSize]);

  const handleMerge = async (sourceId: number, targetId: number) => {
    try {
      const msg = await api.mergeEntities({
        entityType,
        sourceId,
        targetId,
        reason: "Manual review acceptance in Entity Resolution Center",
      });
      setActionMessage(msg);
      loadData();
    } catch (err: any) {
      alert(`Merge failed: ${err.message}`);
    }
  };

  const handleReject = async (aId: number, bId: number) => {
    try {
      await api.rejectCandidate({
        entityType,
        entityAId: aId,
        entityBId: bId,
      });
      setActionMessage("Candidate rejected.");
      loadData();
    } catch (err: any) {
      alert(`Reject failed: ${err.message}`);
    }
  };

  const handleMarkDistinct = async (aId: number, bId: number) => {
    try {
      await api.markDistinct({
        entityType,
        entityAId: aId,
        entityBId: bId,
      });
      setActionMessage("Entities marked as distinct.");
      loadData();
    } catch (err: any) {
      alert(`Failed to mark distinct: ${err.message}`);
    }
  };

  const handleUndoMerge = async (eventId: number) => {
    if (
      !confirm(
        "Are you sure you want to reverse this merge? All aliases and games will be restored.",
      )
    ) {
      return;
    }
    try {
      const msg = await api.undoMerge(eventId);
      setActionMessage(msg);
      loadData();
    } catch (err: any) {
      alert(`Undo failed: ${err.message}`);
    }
  };

  const handleAssignAlias = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!manualRawName.trim() || !manualCanonicalId.trim()) return;
    try {
      const id = parseInt(manualCanonicalId);
      const msg = await api.assignAlias({
        entityType: manualType,
        rawName: manualRawName.trim(),
        canonicalId: id,
      });
      setActionMessage(msg);
      setManualRawName("");
      setManualCanonicalId("");
      loadData();
    } catch (err: any) {
      alert(`Alias assignment failed: ${err.message}`);
    }
  };

  const handleRunCandidateGen = async () => {
    try {
      setIsLoading(true);
      const msg = await api.generateCandidates();
      setActionMessage(msg);
      loadData();
    } catch (err: any) {
      alert(`Candidate generation failed: ${err.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-slate-800 pb-5">
        <div>
          <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
            <GitMerge className="w-5 h-5 text-emerald-400" />
            <span>Entity Resolution Center</span>
          </h1>
          <p className="text-xs text-slate-400 mt-0.5">
            Audit duplicate corporate name variations, evaluate similarity
            candidates, and manage reversible merges
          </p>
        </div>

        <button
          onClick={handleRunCandidateGen}
          className="flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-blue-600/20 hover:bg-blue-600/30 text-blue-400 border border-blue-500/30 text-xs font-medium transition"
        >
          <Sparkles className="w-3.5 h-3.5" />
          <span>Scan for New Duplicate Candidates</span>
        </button>
      </div>

      {/* Action Notification */}
      {actionMessage && (
        <div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-xl text-emerald-400 text-xs flex items-center justify-between">
          <span>{actionMessage}</span>
          <button
            onClick={() => setActionMessage(null)}
            className="text-emerald-500 hover:text-emerald-300"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      )}

      {/* Coverage Metrics Banner */}
      {report && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3 bg-slate-900/60 border border-slate-800/80 p-4 rounded-xl text-xs">
          <div>
            <span className="text-slate-500 block uppercase text-[10px] tracking-wider">
              Observed Raw Names
            </span>
            <span className="text-base font-bold text-slate-100">
              {formatNumber(report.total_observed_names)}
            </span>
          </div>
          <div>
            <span className="text-slate-500 block uppercase text-[10px] tracking-wider">
              Canonical Entities
            </span>
            <span className="text-base font-bold text-slate-100">
              {formatNumber(
                report.total_canonical_developers +
                  report.total_canonical_publishers,
              )}
            </span>
            <span className="text-[10px] text-slate-400 block">
              {report.total_canonical_developers} devs •{" "}
              {report.total_canonical_publishers} pubs
            </span>
          </div>
          <div>
            <span className="text-slate-500 block uppercase text-[10px] tracking-wider">
              Resolved Aliases
            </span>
            <span className="text-base font-bold text-emerald-400">
              {formatNumber(
                report.auto_resolved_aliases + report.manual_confirmed_aliases,
              )}
            </span>
            <span className="text-[10px] text-slate-400 block">
              {report.auto_resolved_aliases} auto •{" "}
              {report.manual_confirmed_aliases} manual
            </span>
          </div>
          <div>
            <span className="text-slate-500 block uppercase text-[10px] tracking-wider">
              Pending Candidates
            </span>
            <span className="text-base font-bold text-amber-400">
              {formatNumber(report.outstanding_candidates)}
            </span>
            <span className="text-[10px] text-slate-400 block">
              {report.merged_entities_count} merges executed
            </span>
          </div>
        </div>
      )}

      {/* Mode Tabs */}
      <div className="flex items-center gap-1 bg-slate-900/60 p-1 rounded-xl border border-slate-800 w-fit text-xs">
        <button
          onClick={() => setActiveTab("review")}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-medium transition ${
            activeTab === "review"
              ? "bg-blue-600/20 text-blue-400 border border-blue-500/30"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <GitMerge className="w-3.5 h-3.5" />
          <span>Review Candidates</span>
        </button>
        <button
          onClick={() => setActiveTab("history")}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-medium transition ${
            activeTab === "history"
              ? "bg-blue-600/20 text-blue-400 border border-blue-500/30"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <History className="w-3.5 h-3.5" />
          <span>Merge Audit History</span>
        </button>
        <button
          onClick={() => setActiveTab("manual")}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-medium transition ${
            activeTab === "manual"
              ? "bg-blue-600/20 text-blue-400 border border-blue-500/30"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <Plus className="w-3.5 h-3.5" />
          <span>Assign Alias</span>
        </button>
      </div>

      {activeTab === "review" && (
        <div className="space-y-4">
          {/* Filters Bar */}
          <div className="bg-slate-900/60 border border-slate-800/80 p-3 rounded-xl flex flex-wrap items-center gap-4 text-xs">
            <div className="flex items-center gap-2">
              <span className="text-slate-400">Entity Type:</span>
              <select
                value={entityType}
                onChange={(e) => {
                  setEntityType(e.target.value);
                  setPage(1);
                }}
                className="px-2 py-1 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              >
                <option value="developer">Developers</option>
                <option value="publisher">Publishers</option>
              </select>
            </div>

            <div className="flex items-center gap-2">
              <span className="text-slate-400">Status:</span>
              <select
                value={status}
                onChange={(e) => {
                  setStatus(e.target.value);
                  setPage(1);
                }}
                className="px-2 py-1 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              >
                <option value="pending">Pending Review</option>
                <option value="merged">Merged</option>
                <option value="rejected">Rejected</option>
                <option value="distinct">Marked Distinct</option>
              </select>
            </div>

            <div className="flex items-center gap-2">
              <span className="text-slate-400">Min Similarity:</span>
              <select
                value={minSimilarity}
                onChange={(e) => {
                  setMinSimilarity(parseFloat(e.target.value));
                  setPage(1);
                }}
                className="px-2 py-1 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              >
                <option value="0.85">≥ 0.85 (High confidence)</option>
                <option value="0.90">≥ 0.90 (Very high confidence)</option>
                <option value="0.80">≥ 0.80 (Broad scan)</option>
              </select>
            </div>
          </div>

          {/* Candidates List */}
          {isLoading ? (
            <div className="p-12 text-center text-slate-500 text-xs">
              Loading candidates...
            </div>
          ) : !candidates || candidates.items.length === 0 ? (
            <EmptyState
              title="No duplicate candidates in this queue"
              description="Either all candidates have been reviewed, or no matches meet the current threshold. Click 'Scan for New Duplicate Candidates' above to discover new candidate pairs."
            />
          ) : (
            <div className="space-y-4">
              {candidates.items.map((cand) => (
                <div
                  key={cand.id}
                  className="bg-slate-900/50 border border-slate-800 rounded-xl p-5 space-y-4 shadow-sm"
                >
                  {/* Top Bar: Match explanation & similarity score */}
                  <div className="flex items-center justify-between border-b border-slate-800/80 pb-3">
                    <div className="flex items-center gap-2.5">
                      <span className="px-2.5 py-0.5 rounded-full font-mono text-xs font-bold bg-amber-500/15 text-amber-400 border border-amber-500/30">
                        {(cand.similarity_score * 100).toFixed(0)}% match
                      </span>
                      <span className="text-xs text-slate-300 font-medium">
                        {cand.matching_explanation}
                      </span>
                    </div>

                    <span className="text-[10px] uppercase font-semibold text-slate-500 tracking-wider">
                      Status: {cand.status}
                    </span>
                  </div>

                  {/* Side by side comparison */}
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {/* Entity A */}
                    <div className="bg-slate-950/70 border border-slate-800/80 rounded-lg p-3.5 space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="text-[10px] text-slate-500 uppercase font-semibold">
                          Entity Candidate A
                        </span>
                        <span className="text-xs font-semibold text-blue-400">
                          {cand.entity_a_games} games
                        </span>
                      </div>
                      <h4 className="text-sm font-bold text-slate-100">
                        {cand.entity_a_name}
                      </h4>

                      {cand.example_games_a.length > 0 && (
                        <div className="pt-2 border-t border-slate-800/60 space-y-1">
                          <span className="text-[10px] text-slate-500 block">
                            Example Games:
                          </span>
                          {cand.example_games_a.map((g, i) => (
                            <div
                              key={i}
                              className="text-xs text-slate-400 truncate flex items-center gap-1.5"
                            >
                              <Gamepad2 className="w-3 h-3 text-slate-500 shrink-0" />
                              <span className="truncate">{g}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>

                    {/* Entity B */}
                    <div className="bg-slate-950/70 border border-slate-800/80 rounded-lg p-3.5 space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="text-[10px] text-slate-500 uppercase font-semibold">
                          Entity Candidate B
                        </span>
                        <span className="text-xs font-semibold text-blue-400">
                          {cand.entity_b_games} games
                        </span>
                      </div>
                      <h4 className="text-sm font-bold text-slate-100">
                        {cand.entity_b_name}
                      </h4>

                      {cand.example_games_b.length > 0 && (
                        <div className="pt-2 border-t border-slate-800/60 space-y-1">
                          <span className="text-[10px] text-slate-500 block">
                            Example Games:
                          </span>
                          {cand.example_games_b.map((g, i) => (
                            <div
                              key={i}
                              className="text-xs text-slate-400 truncate flex items-center gap-1.5"
                            >
                              <Gamepad2 className="w-3 h-3 text-slate-500 shrink-0" />
                              <span className="truncate">{g}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Review Actions */}
                  {cand.status === "pending" && (
                    <div className="flex flex-wrap items-center justify-end gap-2 pt-2 border-t border-slate-800/80">
                      <button
                        onClick={() =>
                          handleMarkDistinct(cand.entity_a_id, cand.entity_b_id)
                        }
                        className="flex items-center gap-1 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800 text-slate-300 hover:bg-slate-700 text-xs font-medium transition"
                      >
                        <Split className="w-3.5 h-3.5" />
                        <span>Keep Distinct</span>
                      </button>

                      <button
                        onClick={() =>
                          handleReject(cand.entity_a_id, cand.entity_b_id)
                        }
                        className="flex items-center gap-1 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800 text-slate-300 hover:bg-slate-700 text-xs font-medium transition"
                      >
                        <X className="w-3.5 h-3.5" />
                        <span>Reject</span>
                      </button>

                      {/* Merge A into B */}
                      <button
                        onClick={() =>
                          handleMerge(cand.entity_a_id, cand.entity_b_id)
                        }
                        className="flex items-center gap-1 px-3 py-1.5 rounded-lg bg-emerald-600/20 hover:bg-emerald-600/30 text-emerald-400 border border-emerald-500/30 text-xs font-medium transition"
                      >
                        <Check className="w-3.5 h-3.5" />
                        <span>
                          Merge "{cand.entity_a_name}" → "{cand.entity_b_name}"
                        </span>
                      </button>

                      {/* Or Merge B into A */}
                      <button
                        onClick={() =>
                          handleMerge(cand.entity_b_id, cand.entity_a_id)
                        }
                        className="flex items-center gap-1 px-3 py-1.5 rounded-lg bg-emerald-600/20 hover:bg-emerald-600/30 text-emerald-400 border border-emerald-500/30 text-xs font-medium transition"
                      >
                        <Check className="w-3.5 h-3.5" />
                        <span>
                          Merge "{cand.entity_b_name}" → "{cand.entity_a_name}"
                        </span>
                      </button>
                    </div>
                  )}
                </div>
              ))}

              <Pagination
                page={page}
                totalPages={candidates.total_pages}
                totalItems={candidates.total}
                pageSize={pageSize}
                onPageChange={setPage}
              />
            </div>
          )}
        </div>
      )}

      {/* History Audit Tab */}
      {activeTab === "history" && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm">
          <div className="p-4 border-b border-slate-800">
            <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
              Merge Event History & Audit Log ({history.length})
            </h3>
          </div>

          {history.length === 0 ? (
            <div className="py-12">
              <EmptyState
                title="No merge events recorded yet"
                description="When duplicate entities are merged during review, their actions and timestamps will be logged here for audit and reversal."
              />
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-left text-xs text-slate-300">
                <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
                  <tr>
                    <th className="px-4 py-3">Timestamp</th>
                    <th className="px-4 py-3">Type</th>
                    <th className="px-4 py-3">Merged Source Entity</th>
                    <th className="px-4 py-3">Into Target Entity</th>
                    <th className="px-4 py-3">Reason</th>
                    <th className="px-4 py-3 text-right">Action</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-800/60">
                  {history.map((item) => (
                    <tr
                      key={item.id}
                      className="hover:bg-slate-800/30 transition"
                    >
                      <td className="px-4 py-3 text-slate-400 font-mono text-[11px]">
                        {item.created_at}
                      </td>
                      <td className="px-4 py-3">
                        <Badge variant="neutral">{item.entity_type}</Badge>
                      </td>
                      <td className="px-4 py-3 font-medium text-rose-400/90 line-through">
                        {item.source_canonical_name} (ID:{" "}
                        {item.source_entity_id})
                      </td>
                      <td className="px-4 py-3 font-medium text-emerald-400">
                        {item.target_canonical_name} (ID:{" "}
                        {item.target_entity_id})
                      </td>
                      <td className="px-4 py-3 text-slate-400 italic">
                        {item.reason || "Manual review"}
                      </td>
                      <td className="px-4 py-3 text-right">
                        {item.is_undone ? (
                          <span className="text-[10px] text-slate-500 font-medium">
                            Undone on {item.undone_at || ""}
                          </span>
                        ) : (
                          <button
                            onClick={() => handleUndoMerge(item.id)}
                            className="flex items-center gap-1 ml-auto px-2.5 py-1 rounded-lg bg-amber-600/15 hover:bg-amber-600/25 text-amber-400 border border-amber-500/30 font-medium text-xs transition"
                          >
                            <RotateCcw className="w-3 h-3" />
                            <span>Undo Merge</span>
                          </button>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}

      {/* Manual Alias Assignment Tab */}
      {activeTab === "manual" && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-6 max-w-xl mx-auto space-y-4">
          <div>
            <h3 className="text-sm font-semibold text-slate-200">
              Manual Alias Mapping
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Directly map an observed raw corporate string to an existing
              canonical entity identifier.
            </p>
          </div>

          <form onSubmit={handleAssignAlias} className="space-y-4 text-xs">
            <div>
              <label className="text-slate-400 block mb-1">Entity Type:</label>
              <select
                value={manualType}
                onChange={(e) => setManualType(e.target.value)}
                className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              >
                <option value="developer">Developer</option>
                <option value="publisher">Publisher</option>
              </select>
            </div>

            <div>
              <label className="text-slate-400 block mb-1">
                Raw Observed Name:
              </label>
              <input
                type="text"
                placeholder="e.g. Double Fine Productions, Inc."
                value={manualRawName}
                onChange={(e) => setManualRawName(e.target.value)}
                required
                className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              />
            </div>

            <div>
              <label className="text-slate-400 block mb-1">
                Canonical Entity Target ID:
              </label>
              <input
                type="number"
                placeholder="e.g. 12"
                value={manualCanonicalId}
                onChange={(e) => setManualCanonicalId(e.target.value)}
                required
                className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
              />
            </div>

            <button
              type="submit"
              className="w-full py-2.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-medium shadow-md transition"
            >
              Assign Alias Mapping
            </button>
          </form>
        </div>
      )}
    </div>
  );
};
