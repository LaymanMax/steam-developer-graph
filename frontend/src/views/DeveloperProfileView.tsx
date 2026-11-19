import { ArrowLeft, Building2, Gamepad2, Sparkles, Users2 } from "lucide-react";
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
import { DeveloperProfile, Subgraph } from "../types";
import { formatCompactNumber, formatNumber } from "../utils/formatters";

interface DeveloperProfileViewProps {
  developerId: number;
  onBack: () => void;
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const DeveloperProfileView: React.FC<DeveloperProfileViewProps> = ({
  developerId,
  onBack,
  onSelectEntity,
}) => {
  const [profile, setProfile] = useState<DeveloperProfile | null>(null);
  const [network, setNetwork] = useState<Subgraph | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        const [profData, netData] = await Promise.all([
          api.getDeveloperProfile(developerId),
          api.getDeveloperNetwork(developerId),
        ]);
        setProfile(profData);
        setNetwork(netData);
      } catch (err: any) {
        setError(err.message || "Failed to load developer profile");
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, [developerId]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-slate-400 text-sm">
        Loading developer profile...
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
          <span>Back to Developers</span>
        </button>
        <div className="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm">
          {error || "Developer profile not found"}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Back button and profile header */}
      <div className="space-y-3">
        <button
          onClick={onBack}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 transition"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back to Developer Directory</span>
        </button>

        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800 pb-5">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <Badge variant="developer">DEVELOPER</Badge>
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

      {/* Summary KPI Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard
          label="Catalog Games"
          value={formatNumber(profile.game_count)}
          subValue="Associated titles"
          icon={<Gamepad2 className="w-4 h-4" />}
          variant="blue"
        />
        <StatCard
          label="Publishers Used"
          value={profile.publisher_count}
          subValue="Publishing partners"
          icon={<Building2 className="w-4 h-4" />}
          variant="emerald"
        />
        <StatCard
          label="Collaborators"
          value={profile.collaborator_count}
          subValue="Co-credited studios"
          icon={<Users2 className="w-4 h-4" />}
          variant="purple"
        />
        <StatCard
          label="Estimated Midpoint Owners"
          value={`~${formatCompactNumber(profile.total_owner_midpoint)}`}
          subValue={`Range: ${formatCompactNumber(profile.total_owner_lower)} – ${formatCompactNumber(profile.total_owner_upper)}`}
          icon={<Sparkles className="w-4 h-4" />}
          variant="amber"
        />
      </div>

      {/* Local Network Graph */}
      {network && network.nodes.length > 1 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-sm font-semibold text-slate-200">
                Local Relationship Network
              </h3>
              <p className="text-xs text-slate-400">
                Immediate network of associated games, publishing entities, and
                co-developers
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

      {/* Timeline chart */}
      {profile.timeline.length > 0 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200">
            Release History by Year
          </h3>
          <div className="h-44 w-full">
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
                  name="Games Released"
                  fill="#3b82f6"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      )}

      {/* Connected Publishers and Collaborators Side-by-Side */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Connected Publishers */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
            <Building2 className="w-4 h-4 text-emerald-400" />
            <span>Publishing Relationships ({profile.publishers.length})</span>
          </h3>
          {profile.publishers.length === 0 ? (
            <p className="text-xs text-slate-500 py-4 text-center">
              No distinct external publishers
            </p>
          ) : (
            <div className="divide-y divide-slate-800/60 max-h-64 overflow-y-auto">
              {profile.publishers.map((pub) => (
                <div
                  key={pub.publisher_id}
                  onClick={() => onSelectEntity("publisher", pub.publisher_id)}
                  className="py-2.5 flex items-center justify-between hover:bg-slate-800/40 px-2 rounded-lg cursor-pointer transition"
                >
                  <div>
                    <span className="text-xs font-medium text-slate-200 hover:text-emerald-400 block truncate">
                      {pub.publisher_name}
                    </span>
                    {pub.first_year && (
                      <span className="text-[10px] text-slate-500">
                        {pub.first_year} – {pub.latest_year}
                      </span>
                    )}
                  </div>
                  <span className="text-xs font-semibold text-emerald-400 shrink-0">
                    {pub.shared_games_count} games
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Collaborators */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
            <Users2 className="w-4 h-4 text-purple-400" />
            <span>
              Co-Development Collaborators ({profile.collaborators.length})
            </span>
          </h3>
          {profile.collaborators.length === 0 ? (
            <p className="text-xs text-slate-500 py-4 text-center">
              No multi-developer collaborations recorded
            </p>
          ) : (
            <div className="divide-y divide-slate-800/60 max-h-64 overflow-y-auto">
              {profile.collaborators.map((collab) => (
                <div
                  key={collab.developer_id}
                  onClick={() =>
                    onSelectEntity("developer", collab.developer_id)
                  }
                  className="py-2.5 flex items-center justify-between hover:bg-slate-800/40 px-2 rounded-lg cursor-pointer transition"
                >
                  <div>
                    <span className="text-xs font-medium text-slate-200 hover:text-purple-400 block truncate">
                      {collab.developer_name}
                    </span>
                    {collab.first_year && (
                      <span className="text-[10px] text-slate-500">
                        {collab.first_year} – {collab.latest_year}
                      </span>
                    )}
                  </div>
                  <span className="text-xs font-semibold text-purple-400 shrink-0">
                    {collab.shared_games_count} games
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Associated Games Catalog */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl overflow-hidden shadow-sm space-y-3 p-5">
        <h3 className="text-sm font-semibold text-slate-200">
          Associated Games Catalog ({profile.catalog.length})
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs text-slate-300">
            <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
              <tr>
                <th className="px-3 py-2.5">Title</th>
                <th className="px-3 py-2.5">Release Date</th>
                <th className="px-3 py-2.5">Publishers</th>
                <th className="px-3 py-2.5">Genres</th>
                <th className="px-3 py-2.5 text-right">Est. Owners</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60">
              {profile.catalog.map((game) => (
                <tr
                  key={game.id}
                  onClick={() => onSelectEntity("game", game.id)}
                  className="hover:bg-slate-800/40 cursor-pointer transition"
                >
                  <td className="px-3 py-2.5 font-medium text-slate-200 hover:text-blue-400">
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
                    {game.publishers.map((p) => p.name).join(", ") ||
                      "Self-published"}
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
