import {
  AlertTriangle,
  ArrowRight,
  Briefcase,
  Building2,
  Gamepad2,
  Share2,
  Sparkles,
  Users2,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import {
  Area,
  AreaChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { StatCard } from "../components/common/StatCard";
import { ViewType } from "../components/layout/Sidebar";
import { HistoricalYearActivity, OverviewSummary } from "../types";
import { formatNumber } from "../utils/formatters";

interface OverviewViewProps {
  onNavigate: (view: ViewType) => void;
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const OverviewView: React.FC<OverviewViewProps> = ({
  onNavigate,
  onSelectEntity,
}) => {
  const [summary, setSummary] = useState<OverviewSummary | null>(null);
  const [timeline, setTimeline] = useState<HistoricalYearActivity[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadData() {
      try {
        setIsLoading(true);
        const [sumData, timeData] = await Promise.all([
          api.getOverviewSummary(),
          api.getHistoricalTimeline(),
        ]);
        setSummary(sumData);
        setTimeline(timeData);
      } catch (err: any) {
        setError(err.message || "Failed to load ecosystem summary");
      } finally {
        setIsLoading(false);
      }
    }
    loadData();
  }, []);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-slate-400 text-sm">
        Loading Steam Developer Network Graph ecosystem...
      </div>
    );
  }

  if (error || !summary) {
    return (
      <div className="p-8 max-w-4xl mx-auto">
        <div className="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm">
          {error || "No overview summary data available"}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Hero Title */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800/80 pb-6">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-100 flex items-center gap-2.5">
            <span>Steam Developer Network Graph</span>
            <Badge variant="developer">Ecosystem Intelligence</Badge>
          </h1>
          <p className="text-sm text-slate-400 mt-1 max-w-2xl">
            Interactive structural analysis of businesses behind Steam games.
            Explore publisher portfolios, developer collaborations, catalog
            histories, and entity relationships.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={() => onNavigate("network")}
            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-blue-600 hover:bg-blue-500 text-white text-xs font-semibold shadow-md shadow-blue-600/20 transition"
          >
            <Share2 className="w-4 h-4" />
            <span>Launch Network Explorer</span>
          </button>
        </div>
      </div>

      {/* Top Metric Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard
          label="Total Games Ingested"
          value={formatNumber(summary.total_games)}
          subValue={`${formatNumber(summary.complete_credit_games)} with full credits`}
          icon={<Gamepad2 className="w-4 h-4" />}
          variant="amber"
        />
        <StatCard
          label="Canonical Developers"
          value={formatNumber(summary.total_developers)}
          subValue={`${formatNumber(summary.dev_collab_relationships)} collaborations`}
          icon={<Briefcase className="w-4 h-4" />}
          variant="blue"
        />
        <StatCard
          label="Canonical Publishers"
          value={formatNumber(summary.total_publishers)}
          subValue={`${formatNumber(summary.dev_pub_relationships)} relationships`}
          icon={<Building2 className="w-4 h-4" />}
          variant="emerald"
        />
        <StatCard
          label="Entity Aliases Resolved"
          value={formatNumber(summary.total_aliases)}
          subValue={`${formatNumber(summary.outstanding_candidates)} pending review`}
          icon={<Sparkles className="w-4 h-4" />}
          variant="purple"
        />
      </div>

      {/* Data Quality Pill Bar */}
      <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-4 flex flex-wrap items-center justify-between gap-4 text-xs">
        <div className="flex items-center gap-2 text-slate-300">
          <AlertTriangle className="w-4 h-4 text-amber-400 shrink-0" />
          <span className="font-medium">Data Quality Classification:</span>
          <span className="text-slate-400">
            {formatNumber(summary.sparse_records)} sparse records •{" "}
            {formatNumber(summary.likely_playtests)} suspected playtests
            identified
          </span>
        </div>
        <button
          onClick={() => onNavigate("dataset")}
          className="text-blue-400 hover:text-blue-300 font-medium flex items-center gap-1 transition"
        >
          <span>View Quality Audit Report</span>
          <ArrowRight className="w-3.5 h-3.5" />
        </button>
      </div>

      {/* Release Timeline Preview */}
      {timeline.length > 0 && (
        <div className="bg-slate-900/50 border border-slate-800/80 rounded-xl p-5 space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-sm font-semibold text-slate-200">
                Catalog Activity by Year
              </h3>
              <p className="text-xs text-slate-400">
                Annual volume of game releases and active developers
              </p>
            </div>
            <button
              onClick={() => onNavigate("historical")}
              className="text-xs text-blue-400 hover:text-blue-300 font-medium flex items-center gap-1"
            >
              <span>Full Historical Analysis</span>
              <ArrowRight className="w-3.5 h-3.5" />
            </button>
          </div>
          <div className="h-56 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart
                data={timeline.filter((t) => t.year >= 2005 && t.year <= 2025)}
              >
                <defs>
                  <linearGradient id="colorGames" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.4} />
                    <stop offset="95%" stopColor="#3b82f6" stopOpacity={0.0} />
                  </linearGradient>
                </defs>
                <XAxis dataKey="year" stroke="#64748b" fontSize={11} />
                <YAxis stroke="#64748b" fontSize={11} />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "#0f172a",
                    borderColor: "#334155",
                    borderRadius: "0.75rem",
                    fontSize: "12px",
                  }}
                />
                <Area
                  type="monotone"
                  dataKey="games_released"
                  name="Games Released"
                  stroke="#3b82f6"
                  strokeWidth={2}
                  fillOpacity={1}
                  fill="url(#colorGames)"
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      )}

      {/* Ecosystem Rankings Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Publishers with most developers */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <Building2 className="w-4 h-4 text-emerald-400" />
              <span>Publishers by Distinct Developers</span>
            </h3>
            <button
              onClick={() => onNavigate("publishers")}
              className="text-[11px] text-slate-400 hover:text-slate-200"
            >
              View all
            </button>
          </div>
          <div className="divide-y divide-slate-800/60">
            {summary.top_publishers_by_devs.map((pub, idx) => (
              <div
                key={pub.id}
                onClick={() => onSelectEntity("publisher", pub.id)}
                className="py-2.5 flex items-center justify-between hover:bg-slate-800/30 px-2 rounded-lg cursor-pointer transition"
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <span className="text-xs font-mono text-slate-500 w-4">
                    {idx + 1}
                  </span>
                  <span className="text-xs font-medium text-slate-200 truncate hover:text-emerald-400">
                    {pub.name}
                  </span>
                </div>
                <div className="text-right shrink-0">
                  <span className="text-xs font-semibold text-emerald-400">
                    {pub.metric_value} devs
                  </span>
                  {pub.secondary_label && (
                    <span className="text-[10px] text-slate-500 block">
                      {pub.secondary_label}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Developers with most publishers */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <Briefcase className="w-4 h-4 text-blue-400" />
              <span>Multi-Publisher Developers</span>
            </h3>
            <button
              onClick={() => onNavigate("developers")}
              className="text-[11px] text-slate-400 hover:text-slate-200"
            >
              View all
            </button>
          </div>
          <div className="divide-y divide-slate-800/60">
            {summary.top_developers_by_pubs.map((dev, idx) => (
              <div
                key={dev.id}
                onClick={() => onSelectEntity("developer", dev.id)}
                className="py-2.5 flex items-center justify-between hover:bg-slate-800/30 px-2 rounded-lg cursor-pointer transition"
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <span className="text-xs font-mono text-slate-500 w-4">
                    {idx + 1}
                  </span>
                  <span className="text-xs font-medium text-slate-200 truncate hover:text-blue-400">
                    {dev.name}
                  </span>
                </div>
                <div className="text-right shrink-0">
                  <span className="text-xs font-semibold text-blue-400">
                    {dev.metric_value} pubs
                  </span>
                  {dev.secondary_label && (
                    <span className="text-[10px] text-slate-500 block">
                      {dev.secondary_label}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Most Frequent Developer Collaborations */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <Users2 className="w-4 h-4 text-purple-400" />
              <span>Frequent Collaborations</span>
            </h3>
            <button
              onClick={() => onNavigate("collaboration")}
              className="text-[11px] text-slate-400 hover:text-slate-200"
            >
              View all
            </button>
          </div>
          <div className="divide-y divide-slate-800/60">
            {summary.top_collaborations.map((collab, idx) => (
              <div
                key={`${collab.dev1_id}-${collab.dev2_id}`}
                onClick={() => onNavigate("collaboration")}
                className="py-2.5 flex items-center justify-between hover:bg-slate-800/30 px-2 rounded-lg cursor-pointer transition"
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <span className="text-xs font-mono text-slate-500 w-4">
                    {idx + 1}
                  </span>
                  <div className="truncate">
                    <span className="text-xs font-medium text-slate-200 block truncate">
                      {collab.dev1_name}
                    </span>
                    <span className="text-[10px] text-slate-400 block truncate">
                      + {collab.dev2_name}
                    </span>
                  </div>
                </div>
                <span className="text-xs font-semibold text-purple-400 shrink-0">
                  {collab.shared_games} games
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Publishers with highest genre diversity (Shannon entropy) */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <Sparkles className="w-4 h-4 text-amber-400" />
              <span>Genre Diversity (Entropy)</span>
            </h3>
            <span className="text-[10px] text-slate-500">min 8 games</span>
          </div>
          <div className="divide-y divide-slate-800/60">
            {summary.top_publishers_by_entropy.map((pub, idx) => (
              <div
                key={pub.id}
                onClick={() => onSelectEntity("publisher", pub.id)}
                className="py-2.5 flex items-center justify-between hover:bg-slate-800/30 px-2 rounded-lg cursor-pointer transition"
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <span className="text-xs font-mono text-slate-500 w-4">
                    {idx + 1}
                  </span>
                  <span className="text-xs font-medium text-slate-200 truncate hover:text-amber-400">
                    {pub.name}
                  </span>
                </div>
                <div className="text-right shrink-0">
                  <span className="text-xs font-semibold text-amber-400">
                    H = {pub.metric_value.toFixed(2)}
                  </span>
                  {pub.secondary_label && (
                    <span className="text-[10px] text-slate-500 block">
                      {pub.secondary_label}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Largest Developer Catalogs */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <Briefcase className="w-4 h-4 text-blue-400" />
              <span>Largest Developer Catalogs</span>
            </h3>
            <button
              onClick={() => onNavigate("developers")}
              className="text-[11px] text-slate-400 hover:text-slate-200"
            >
              View all
            </button>
          </div>
          <div className="divide-y divide-slate-800/60">
            {summary.top_developers_by_games.map((dev, idx) => (
              <div
                key={dev.id}
                onClick={() => onSelectEntity("developer", dev.id)}
                className="py-2.5 flex items-center justify-between hover:bg-slate-800/30 px-2 rounded-lg cursor-pointer transition"
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <span className="text-xs font-mono text-slate-500 w-4">
                    {idx + 1}
                  </span>
                  <span className="text-xs font-medium text-slate-200 truncate hover:text-blue-400">
                    {dev.name}
                  </span>
                </div>
                <div className="text-right shrink-0">
                  <span className="text-xs font-semibold text-blue-400">
                    {dev.metric_value} games
                  </span>
                  {dev.secondary_label && (
                    <span className="text-[10px] text-slate-500 block">
                      {dev.secondary_label}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Direct Action Hub */}
        <div className="bg-gradient-to-br from-slate-900/80 to-blue-950/30 border border-blue-500/20 rounded-xl p-5 space-y-4 flex flex-col justify-between">
          <div className="space-y-2">
            <Badge variant="developer">Quick Workspaces</Badge>
            <h4 className="text-sm font-semibold text-slate-200">
              Investigate the Steam Business Graph
            </h4>
            <p className="text-xs text-slate-400 leading-relaxed">
              Drill down into developer collaboration records, review proposed
              entity duplicate matches, or export raw CSV relationship datasets.
            </p>
          </div>
          <div className="space-y-2 pt-2">
            <button
              onClick={() => onNavigate("collaboration")}
              className="w-full flex items-center justify-between px-3 py-2 rounded-lg bg-slate-800/80 hover:bg-slate-700 text-xs font-medium text-slate-200 transition"
            >
              <span>Collaboration Lab</span>
              <ArrowRight className="w-3.5 h-3.5" />
            </button>
            <button
              onClick={() => onNavigate("resolution")}
              className="w-full flex items-center justify-between px-3 py-2 rounded-lg bg-slate-800/80 hover:bg-slate-700 text-xs font-medium text-slate-200 transition"
            >
              <span>Entity Resolution Center</span>
              <ArrowRight className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
