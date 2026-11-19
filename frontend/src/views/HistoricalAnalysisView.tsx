import { Info, TrendingUp } from "lucide-react";
import React, { useEffect, useState } from "react";
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  Legend,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { api } from "../api/client";
import { GenreYearTrend, HistoricalYearActivity } from "../types";

export const HistoricalAnalysisView: React.FC = () => {
  const [timeline, setTimeline] = useState<HistoricalYearActivity[]>([]);
  const [genreTrends, setGenreTrends] = useState<GenreYearTrend[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        setIsLoading(true);
        const [timeData, genreData] = await Promise.all([
          api.getHistoricalTimeline(),
          api.getGenreGrowth(6),
        ]);
        setTimeline(timeData);
        setGenreTrends(genreData);
      } catch (err) {
        console.error("Failed to load historical data:", err);
      } finally {
        setIsLoading(false);
      }
    }
    load();
  }, []);

  // Filter out ancient outliers (e.g. before 2005) for modern steam view
  const recentTimeline = timeline.filter(
    (t) => t.year >= 2005 && t.year <= 2025,
  );

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <div className="border-b border-slate-800 pb-5">
        <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-blue-400" />
          <span>Historical Catalog & Ecosystem Growth</span>
        </h1>
        <p className="text-xs text-slate-400 mt-0.5">
          Longitudinal analysis of game releases, participating developer
          studios, and new business relationships
        </p>
      </div>

      {/* Dataset Caveat Note */}
      <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-4 flex items-start gap-3 text-xs leading-relaxed">
        <Info className="w-4 h-4 text-blue-400 shrink-0 mt-0.5" />
        <p className="text-slate-400">
          <span className="font-semibold text-slate-200">
            Historical Data Caveat:
          </span>{" "}
          Release dates are based on the Steam store release dates recorded in
          the dataset. Early titles released before Steam's 2003 inception
          represent legacy additions to the store. Games with missing or
          unparseable release dates are excluded from annual aggregations.
        </p>
      </div>

      {/* Annual Volume Chart */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-6 space-y-4">
        <div>
          <h3 className="text-sm font-semibold text-slate-200">
            Annual Releases & Active Studios
          </h3>
          <p className="text-xs text-slate-400">
            Total game releases and unique active developers by year
          </p>
        </div>

        <div className="h-72 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={recentTimeline}>
              <defs>
                <linearGradient id="colorGamesH" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.4} />
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0.0} />
                </linearGradient>
                <linearGradient id="colorDevsH" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.4} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0.0} />
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
              <Legend wrapperStyle={{ fontSize: "12px" }} />
              <Area
                type="monotone"
                dataKey="games_released"
                name="Games Released"
                stroke="#3b82f6"
                strokeWidth={2}
                fillOpacity={1}
                fill="url(#colorGamesH)"
              />
              <Area
                type="monotone"
                dataKey="active_developers"
                name="Active Developers"
                stroke="#10b981"
                strokeWidth={2}
                fillOpacity={1}
                fill="url(#colorDevsH)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* New Relationships Over Time */}
      <div className="bg-slate-900/40 border border-slate-800/80 rounded-xl p-6 space-y-4">
        <div>
          <h3 className="text-sm font-semibold text-slate-200">
            Formation of New Business Relationships
          </h3>
          <p className="text-xs text-slate-400">
            First-time observed developer collaborations and publisher-developer
            relationships per year
          </p>
        </div>

        <div className="h-64 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={recentTimeline}>
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
              <Legend wrapperStyle={{ fontSize: "12px" }} />
              <Bar
                dataKey="new_publisher_relationships"
                name="New Pub-Dev Relationships"
                fill="#38bdf8"
                radius={[3, 3, 0, 0]}
              />
              <Bar
                dataKey="new_developer_collaborations"
                name="New Dev Collaborations"
                fill="#a855f7"
                radius={[3, 3, 0, 0]}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
};
