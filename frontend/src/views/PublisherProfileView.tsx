import { ArrowLeft, Briefcase, Gamepad2, Info, Sparkles } from "lucide-react";
import React, { useEffect, useState } from "react";
import {
  Bar,
  BarChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { StatCard } from "../components/common/StatCard";
import { NetworkGraph } from "../components/graph/NetworkGraph";
import { PublisherProfile, Subgraph } from "../types";
import { formatCompactNumber, formatNumber } from "../utils/formatters";

interface PublisherProfileViewProps {
  publisherId: number;
  onBack: () => void;
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

const GENRE_COLORS = [
  "#3b82f6",
  "#10b981",
  "#f59e0b",
  "#a855f7",
  "#ec4899",
  "#06b6d4",
  "#84cc16",
  "#eab308",
];

export const PublisherProfileView: React.FC<PublisherProfileViewProps> = ({
  publisherId,
  onBack,
  onSelectEntity,
}) => {
  const [profile, setProfile] = useState<PublisherProfile | null>(null);
  const [network, setNetwork] = useState<Subgraph | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        const [profData, netData] = await Promise.all([
          api.getPublisherProfile(publisherId),
          api.getPublisherNetwork(publisherId),
        ]);
        setProfile(profData);
        setNetwork(netData);
      } catch (err: any) {
        setError(err.message || "Failed to load publisher profile");
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, [publisherId]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-slate-400 text-sm">
        Loading publisher profile...
      </div>
    );
  }

  if (error || !profile) {
    return (
      <div className="p-8 max-w-4xl mx-auto space-y-4">
        <button
          onClick={onBack}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 transition"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back to Publishers</span>
        </button>
        <div className="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm">
          {error || "Publisher profile not found"}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <div className="space-y-3">
        <button
          onClick={onBack}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 transition"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back to Publisher Directory</span>
        </button>

        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800 pb-5">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <Badge variant="publisher">PUBLISHER</Badge>
              {profile.earliest_release_year && (
                <span className="text-xs text-slate-400">
                  Active {profile.earliest_release_year} –{" "}
                  {profile.latest_release_year || "Present"}
                </span>
              )}
            </div>
            <h1 className="text-2xl font-bold text-slate-100 tracking-tight">
              {profile.canonical_name}
            </h1>
            {profile.aliases.length > 0 && (
              <div className="flex flex-wrap items-center gap-1.5 mt-2 text-xs text-slate-400">
                <span className="text-slate-500">Known aliases:</span>
                {profile.aliases.map((alias, i) => (
                  <span
                    key={i}
                    className="px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono text-[11px] border border-slate-700/60"
                  >
                    {alias}
                  </span>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard
          label="Portfolio Titles"
          value={formatNumber(profile.game_count)}
          subValue="Published games"
          icon={<Gamepad2 className="w-4 h-4" />}
          variant="emerald"
        />
        <StatCard
          label="Distinct Developers"
          value={profile.developer_count}
          subValue="Studio partners"
          icon={<Briefcase className="w-4 h-4" />}
          variant="blue"
        />
        <StatCard
          label="Genre Diversity (H)"
          value={
            profile.shannon_entropy > 0
              ? profile.shannon_entropy.toFixed(2)
              : "0.00"
          }
          subValue={`${profile.genre_count} genres represented`}
          icon={<Sparkles className="w-4 h-4" />}
          variant="amber"
        />
        <StatCard
          label="Estimated Midpoint Owners"
          value={`~${formatCompactNumber(profile.total_owner_midpoint)}`}
          subValue={`Range: ${formatCompactNumber(profile.total_owner_lower)} – ${formatCompactNumber(profile.total_owner_upper)}`}
          icon={<Sparkles className="w-4 h-4" />}
          variant="purple"
        />
      </div>

      {/* Shannon Entropy Educational Explainer Box */}
      <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-4 flex items-start gap-3 text-xs leading-relaxed">
        <Info className="w-4 h-4 text-amber-400 shrink-0 mt-0.5" />
        <div className="space-y-1">
          <span className="font-semibold text-slate-200">
            Portfolio Genre Diversity (Shannon Entropy Metric Explained):
          </span>
          <p className="text-slate-400">
            Calculated as{" "}
            <span className="font-mono text-amber-400">
              H = - Σ (p_i · ln p_i)
            </span>{" "}
            across genre proportions. A higher entropy score reflects a
            balanced, well-diversified publishing portfolio spanning multiple
            genres, whereas a lower score indicates heavy catalog concentration
            in a single genre.
          </p>
        </div>
      </div>

      {/* Local Network Graph */}
      {network && network.nodes.length > 1 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-sm font-semibold text-slate-200">
                Publisher Network
              </h3>
              <p className="text-xs text-slate-400">
                Connected developers and released titles across this publisher's
                history
              </p>
            </div>
            <span className="text-[11px] text-slate-400 bg-slate-800 px-2 py-1 rounded border border-slate-700">
              {network.nodes.length} entities connected
            </span>
          </div>
          <div className="h-80 w-full rounded-xl overflow-hidden border border-slate-800 relative">
            <NetworkGraph data={network} onNavigateToEntity={onSelectEntity} />
          </div>
        </div>
      )}

      {/* Genre Distribution & Timeline Charts */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Genre Distribution */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200">
            Genre Concentration
          </h3>
          <div className="h-52 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                data={profile.genre_distribution.slice(0, 7)}
                layout="vertical"
              >
                <XAxis type="number" stroke="#64748b" fontSize={11} />
                <YAxis
                  dataKey="genre_name"
                  type="category"
                  stroke="#64748b"
                  fontSize={11}
                  width={80}
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "#0f172a",
                    borderColor: "#334155",
                    borderRadius: "0.75rem",
                    fontSize: "12px",
                  }}
                />
                <Bar
                  dataKey="game_count"
                  name="Games"
                  fill="#10b981"
                  radius={[0, 4, 4, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Release Timeline */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200">
            Portfolio Growth Over Time
          </h3>
          <div className="h-52 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={profile.timeline}>
                <XAxis dataKey="year" stroke="#64748b" fontSize={11} />
                <YAxis stroke="#64748b" fontSize={11} allowDecimals={false} />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "#0f172a",
                    borderColor: "#334155",
                    borderRadius: "0.75rem",
                    fontSize: "12px",
                  }}
                />
                <Bar
                  dataKey="game_count"
                  name="Releases"
                  fill="#3b82f6"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Developer Relationships */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
          <Briefcase className="w-4 h-4 text-blue-400" />
          <span>
            Developer Relationships ({profile.developer_relationships.length})
          </span>
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
          {profile.developer_relationships.map((rel) => (
            <div
              key={rel.developer_id}
              onClick={() => onSelectEntity("developer", rel.developer_id)}
              className="bg-slate-950/60 border border-slate-800/80 hover:border-slate-700 p-3 rounded-lg cursor-pointer transition flex items-center justify-between"
            >
              <div className="min-w-0 pr-2">
                <span className="text-xs font-medium text-slate-200 hover:text-blue-400 block truncate">
                  {rel.developer_name}
                </span>
                {rel.first_year && (
                  <span className="text-[10px] text-slate-500">
                    {rel.first_year} – {rel.latest_year}
                  </span>
                )}
              </div>
              <span className="text-xs font-semibold text-blue-400 shrink-0">
                {rel.shared_games_count} games
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Portfolio Catalog */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm space-y-3 p-5">
        <h3 className="text-sm font-semibold text-slate-200">
          Published Titles ({profile.portfolio.length})
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs text-slate-300">
            <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
              <tr>
                <th className="px-3 py-2.5">Title</th>
                <th className="px-3 py-2.5">Release Date</th>
                <th className="px-3 py-2.5">Developers</th>
                <th className="px-3 py-2.5">Genres</th>
                <th className="px-3 py-2.5 text-right">Est. Owners</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60">
              {profile.portfolio.map((game) => (
                <tr
                  key={game.id}
                  onClick={() => onSelectEntity("game", game.id)}
                  className="hover:bg-slate-800/40 cursor-pointer transition"
                >
                  <td className="px-3 py-2.5 font-medium text-slate-200 hover:text-emerald-400">
                    <div className="flex items-center gap-2">
                      <span>{game.name}</span>
                      {game.is_likely_playtest && (
                        <Badge variant="playtest">Playtest</Badge>
                      )}
                    </div>
                  </td>
                  <td className="px-3 py-2.5 text-slate-400">
                    {game.release_date || "N/A"}
                  </td>
                  <td className="px-3 py-2.5 text-slate-300">
                    {game.developers.map((d) => d.name).join(", ") || "Unknown"}
                  </td>
                  <td className="px-3 py-2.5 text-slate-400">
                    {game.genres.slice(0, 3).join(", ")}
                  </td>
                  <td className="px-3 py-2.5 text-right font-mono text-slate-400 text-[11px]">
                    {game.estimated_owners_raw || "~0"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};
