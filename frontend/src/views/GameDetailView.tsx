import {
  ArrowLeft,
  Briefcase,
  Building2,
  Clock,
  DollarSign,
  ExternalLink,
  ThumbsUp,
  Users,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { StatCard } from "../components/common/StatCard";
import { NetworkGraph } from "../components/graph/NetworkGraph";
import { GameDetail, Subgraph } from "../types";
import {
  formatCompactNumber,
  formatNumber,
  formatPlaytimeHours,
  formatPrice,
} from "../utils/formatters";

interface GameDetailViewProps {
  gameId: number;
  onBack: () => void;
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const GameDetailView: React.FC<GameDetailViewProps> = ({
  gameId,
  onBack,
  onSelectEntity,
}) => {
  const [game, setGame] = useState<GameDetail | null>(null);
  const [network, setNetwork] = useState<Subgraph | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        const [gameData, netData] = await Promise.all([
          api.getGameDetails(gameId),
          api.getGameNetwork(gameId),
        ]);
        setGame(gameData);
        setNetwork(netData);
      } catch (err: any) {
        setError(err.message || "Failed to load game details");
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, [gameId]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-slate-400 text-sm">
        Loading game details...
      </div>
    );
  }

  if (error || !game) {
    return (
      <div className="p-8 max-w-4xl mx-auto space-y-4">
        <button
          onClick={onBack}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 transition"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back</span>
        </button>
        <div className="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm">
          {error || "Game not found"}
        </div>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-8">
      {/* Back button and title */}
      <div className="space-y-3">
        <button
          onClick={onBack}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 transition"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back</span>
        </button>

        <div className="flex flex-col md:flex-row md:items-start justify-between gap-4 border-b border-slate-800 pb-5">
          <div className="space-y-2">
            <div className="flex flex-wrap items-center gap-2">
              <Badge variant="game">GAME</Badge>
              {game.is_likely_playtest && (
                <Badge variant="playtest">Likely Playtest</Badge>
              )}
              {game.is_sparse && (
                <Badge variant="sparse">Sparse Metadata</Badge>
              )}
              <span className="text-xs text-slate-400 font-mono">
                AppID: {game.app_id}
              </span>
            </div>
            <h1 className="text-2xl font-bold text-slate-100 tracking-tight">
              {game.name}
            </h1>
            {game.release_date && (
              <p className="text-xs text-slate-400">
                Released: {game.release_date}
              </p>
            )}
          </div>

          <a
            href={game.steam_store_url}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 px-4 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold border border-slate-700 shadow-sm transition shrink-0"
          >
            <span>View on Steam Store</span>
            <ExternalLink className="w-3.5 h-3.5" />
          </a>
        </div>
      </div>

      {/* Primary KPI Grid */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard
          label="Estimated Owners (Approx)"
          value={game.estimated_owners_raw || "0 - 0"}
          subValue={`Midpoint approximation: ~${formatNumber(game.owner_midpoint)}`}
          icon={<Users className="w-4 h-4" />}
          variant="amber"
        />
        <StatCard
          label="Price & CCU"
          value={formatPrice(game.price)}
          subValue={`Peak CCU: ${formatNumber(game.peak_ccu)}`}
          icon={<DollarSign className="w-4 h-4" />}
          variant="slate"
        />
        <StatCard
          label="User Reviews"
          value={`${formatCompactNumber(game.positive)} Positive`}
          subValue={`${formatCompactNumber(game.negative)} Negative`}
          icon={<ThumbsUp className="w-4 h-4" />}
          variant="emerald"
        />
        <StatCard
          label="Average Playtime"
          value={formatPlaytimeHours(game.average_playtime_forever)}
          subValue={`Median: ${formatPlaytimeHours(game.median_playtime_forever)}`}
          icon={<Clock className="w-4 h-4" />}
          variant="purple"
        />
      </div>

      {/* Hub of Business Relationships: Credited Developers & Publishers */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Credited Developers */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
            <Briefcase className="w-4 h-4 text-blue-400" />
            <span>Credited Developers</span>
          </h3>
          {game.developers.length === 0 ? (
            <p className="text-xs text-slate-500 py-2">
              No developers listed in source record
            </p>
          ) : (
            <div className="space-y-2">
              {game.developers.map((dev) => (
                <div
                  key={dev.id}
                  onClick={() => onSelectEntity("developer", dev.id)}
                  className="flex items-center justify-between p-3 rounded-lg bg-slate-950/60 border border-slate-800/80 hover:border-blue-500/40 cursor-pointer transition"
                >
                  <span className="text-xs font-medium text-slate-200 hover:text-blue-400">
                    {dev.name}
                  </span>
                  <ExternalLink className="w-3.5 h-3.5 text-slate-500" />
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Credited Publishers */}
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
            <Building2 className="w-4 h-4 text-emerald-400" />
            <span>Credited Publishers</span>
          </h3>
          {game.publishers.length === 0 ? (
            <p className="text-xs text-slate-500 py-2">
              Self-published or no publisher listed
            </p>
          ) : (
            <div className="space-y-2">
              {game.publishers.map((pub) => (
                <div
                  key={pub.id}
                  onClick={() => onSelectEntity("publisher", pub.id)}
                  className="flex items-center justify-between p-3 rounded-lg bg-slate-950/60 border border-slate-800/80 hover:border-emerald-500/40 cursor-pointer transition"
                >
                  <span className="text-xs font-medium text-slate-200 hover:text-emerald-400">
                    {pub.name}
                  </span>
                  <ExternalLink className="w-3.5 h-3.5 text-slate-500" />
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Genres and Categories */}
      {game.genres.length > 0 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200">
            Associated Genres
          </h3>
          <div className="flex flex-wrap gap-2">
            {game.genres.map((g, i) => (
              <span
                key={i}
                className="px-3 py-1 rounded-lg bg-slate-800 text-slate-300 text-xs font-medium border border-slate-700/60"
              >
                {g}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Local Network Graph: Game as Connection Hub */}
      {network && network.nodes.length > 1 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-sm font-semibold text-slate-200">
                Connection Hub Graph
              </h3>
              <p className="text-xs text-slate-400">
                This game acting as the central connecting edge between credited
                studios and distribution partners
              </p>
            </div>
          </div>
          <div className="h-72 w-full rounded-xl overflow-hidden border border-slate-800 relative">
            <NetworkGraph data={network} onNavigateToEntity={onSelectEntity} />
          </div>
        </div>
      )}

      {/* Related Games */}
      {game.related_games.length > 0 && (
        <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-5 space-y-3">
          <h3 className="text-sm font-semibold text-slate-200">
            Other Titles by the Same Developer
          </h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
            {game.related_games.map((rel) => (
              <div
                key={rel.id}
                onClick={() => onSelectEntity("game", rel.id)}
                className="bg-slate-950/60 border border-slate-800/80 hover:border-slate-700 p-3 rounded-lg cursor-pointer transition"
              >
                <span className="text-xs font-medium text-slate-200 hover:text-amber-400 block truncate">
                  {rel.name}
                </span>
                <span className="text-[10px] text-slate-500 block mt-0.5">
                  {rel.release_year ? `Year ${rel.release_year}` : "N/A"} • ~
                  {formatCompactNumber(rel.owner_midpoint)} owners
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};
