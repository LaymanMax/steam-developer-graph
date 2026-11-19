import {
  Database,
  Download,
  PlayCircle,
  RefreshCw,
  ShieldAlert,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { api } from "../api/client";
import { Badge } from "../components/common/Badge";
import { DatasetStatus, QualityReport } from "../types";
import { formatNumber } from "../utils/formatters";

export const DatasetStatusView: React.FC = () => {
  const [status, setStatus] = useState<DatasetStatus | null>(null);
  const [quality, setQuality] = useState<QualityReport | null>(null);
  const [customPath, setCustomPath] = useState("");
  const [customUrl, setCustomUrl] = useState("");
  const [isImporting, setIsImporting] = useState(false);
  const [isDownloading, setIsDownloading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const loadStatus = async () => {
    try {
      setIsLoading(true);
      const [sData, qData] = await Promise.all([
        api.getDatasetStatus(),
        api.getDataQualityReport().catch(() => null),
      ]);
      setStatus(sData);
      setQuality(qData);
    } catch (err) {
      console.error("Failed to load dataset status:", err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadStatus();
  }, []);

  const handleImport = async () => {
    try {
      setIsImporting(true);
      setMessage(
        "Importing dataset records into SQLite... This may take a few moments.",
      );
      const res = await api.triggerImport(customPath.trim() || undefined);
      setMessage(
        `Successfully imported ${formatNumber(res.rows_imported)} games in ${res.duration_seconds.toFixed(2)} seconds!`,
      );
      loadStatus();
    } catch (err: any) {
      setMessage(`Import failed: ${err.message}`);
    } finally {
      setIsImporting(false);
    }
  };

  const handleDownload = async () => {
    try {
      setIsDownloading(true);
      const msg = await api.triggerDownload(customUrl.trim() || undefined);
      setMessage(msg);
      loadStatus();
    } catch (err: any) {
      setMessage(`Download failed: ${err.message}`);
    } finally {
      setIsDownloading(false);
    }
  };

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-slate-800 pb-5">
        <div>
          <h1 className="text-xl font-bold text-slate-100 flex items-center gap-2">
            <Database className="w-5 h-5 text-blue-400" />
            <span>Dataset Acquisition & Data Quality</span>
          </h1>
          <p className="text-xs text-slate-400 mt-0.5">
            Dataset provenance, local Parquet file integrity, ingestion pipeline
            triggers, and data quality audits
          </p>
        </div>

        <button
          onClick={loadStatus}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-slate-700 bg-slate-800/80 hover:bg-slate-700 text-xs font-medium text-slate-200 transition self-start sm:self-auto"
        >
          <RefreshCw className="w-3.5 h-3.5" />
          <span>Refresh Status</span>
        </button>
      </div>

      {message && (
        <div className="p-4 bg-blue-600/10 border border-blue-500/30 rounded-xl text-blue-300 text-xs leading-relaxed flex items-center justify-between">
          <span>{message}</span>
          <button
            onClick={() => setMessage(null)}
            className="text-blue-400 hover:text-blue-200 ml-4 font-bold"
          >
            ✕
          </button>
        </div>
      )}

      {/* Dataset Provenance Card */}
      <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6 space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-slate-800 pb-3">
          <div>
            <h3 className="text-sm font-semibold text-slate-200">
              Source Dataset: FronkonGames/steam-games-dataset
            </h3>
            <p className="text-xs text-slate-400 mt-0.5">
              Hosted on Hugging Face • MIT License • Collected from Steam Web
              API & SteamSpy
            </p>
          </div>
          <span className="text-[11px] font-mono text-slate-400 bg-slate-950 px-2.5 py-1 rounded border border-slate-800">
            Expected records: ~124,146
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          <div className="bg-slate-950/70 p-3.5 rounded-lg border border-slate-800/80 space-y-2">
            <span className="font-semibold text-slate-300 block">
              Local Parquet File Status:
            </span>
            <div className="flex items-center gap-2">
              <span
                className={`w-2.5 h-2.5 rounded-full ${
                  status?.local_parquet_exists
                    ? "bg-emerald-400"
                    : "bg-amber-400"
                }`}
              />
              <span className="text-slate-200 font-medium">
                {status?.local_parquet_exists
                  ? "Parquet File Available"
                  : "File Missing on Disk"}
              </span>
            </div>
            <p className="font-mono text-[11px] text-slate-400 truncate">
              Path: {status?.local_parquet_path}
            </p>
            {status?.local_parquet_size_bytes && (
              <p className="text-slate-400">
                Size:{" "}
                {(status.local_parquet_size_bytes / (1024 * 1024)).toFixed(1)}{" "}
                MB
              </p>
            )}
            {status?.local_parquet_checksum && (
              <p className="font-mono text-[10px] text-slate-500 truncate">
                SHA-256: {status.local_parquet_checksum}
              </p>
            )}
          </div>

          <div className="bg-slate-950/70 p-3.5 rounded-lg border border-slate-800/80 space-y-2">
            <span className="font-semibold text-slate-300 block">
              Current Database Contents:
            </span>
            <div className="flex items-center justify-between text-slate-300">
              <span>Games Ingested:</span>
              <span className="font-bold text-slate-100">
                {formatNumber(status?.total_games)}
              </span>
            </div>
            <div className="flex items-center justify-between text-slate-300">
              <span>Canonical Developers:</span>
              <span className="font-bold text-blue-400">
                {formatNumber(status?.total_developers)}
              </span>
            </div>
            <div className="flex items-center justify-between text-slate-300">
              <span>Canonical Publishers:</span>
              <span className="font-bold text-emerald-400">
                {formatNumber(status?.total_publishers)}
              </span>
            </div>
          </div>
        </div>

        {/* Action controls */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
          {/* Import file form */}
          <div className="p-4 rounded-xl bg-slate-950/80 border border-slate-800 space-y-3 text-xs">
            <span className="font-semibold text-slate-200 block">
              Run Local Ingestion:
            </span>
            <p className="text-slate-400 text-[11px]">
              Ingest existing Parquet or JSON file into the local SQLite
              database.
            </p>
            <input
              type="text"
              placeholder="Custom path (optional; defaults to data/steam_games.parquet)"
              value={customPath}
              onChange={(e) => setCustomPath(e.target.value)}
              className="w-full px-3 py-1.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
            />
            <button
              onClick={handleImport}
              disabled={isImporting}
              className="w-full py-2 rounded-lg bg-blue-600 hover:bg-blue-500 disabled:opacity-50 text-white font-medium transition flex items-center justify-center gap-2"
            >
              <PlayCircle className="w-4 h-4" />
              <span>
                {isImporting ? "Ingesting Dataset..." : "Start Ingestion"}
              </span>
            </button>
          </div>

          {/* Download from HF form */}
          <div className="p-4 rounded-xl bg-slate-950/80 border border-slate-800 space-y-3 text-xs">
            <span className="font-semibold text-slate-200 block">
              Acquire from Hugging Face:
            </span>
            <p className="text-slate-400 text-[11px]">
              Download latest Parquet release directly from Hugging Face
              (requires network).
            </p>
            <input
              type="text"
              placeholder="Custom URL (optional; defaults to FronkonGames repo)"
              value={customUrl}
              onChange={(e) => setCustomUrl(e.target.value)}
              className="w-full px-3 py-1.5 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500"
            />
            <button
              onClick={handleDownload}
              disabled={isDownloading}
              className="w-full py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white font-medium transition flex items-center justify-center gap-2"
            >
              <Download className="w-4 h-4" />
              <span>
                {isDownloading
                  ? "Downloading File..."
                  : "Download from Hugging Face"}
              </span>
            </button>
          </div>
        </div>
      </div>

      {/* Data Quality Classification Report */}
      {quality && (
        <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6 space-y-6">
          <div>
            <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2">
              <ShieldAlert className="w-4 h-4 text-amber-400" />
              <span>Transparent Data Quality Audit</span>
            </h3>
            <p className="text-xs text-slate-400 mt-0.5">
              Classification of incomplete commercial games, missing release
              dates, and sparse playtest records
            </p>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-xs">
            <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800">
              <span className="text-slate-500 block uppercase text-[10px]">
                Sparse Metadata
              </span>
              <span className="text-base font-bold text-amber-400">
                {formatNumber(quality.sparse_records)}
              </span>
              <span className="text-[10px] text-slate-500 block mt-0.5">
                Minimal metadata
              </span>
            </div>

            <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800">
              <span className="text-slate-500 block uppercase text-[10px]">
                Suspected Playtests
              </span>
              <span className="text-base font-bold text-purple-400">
                {formatNumber(quality.likely_playtests)}
              </span>
              <span className="text-[10px] text-slate-500 block mt-0.5">
                Title contains 'playtest'
              </span>
            </div>

            <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800">
              <span className="text-slate-500 block uppercase text-[10px]">
                Missing Developers
              </span>
              <span className="text-base font-bold text-rose-400">
                {formatNumber(quality.missing_developers)}
              </span>
              <span className="text-[10px] text-slate-500 block mt-0.5">
                Uncredited studio
              </span>
            </div>

            <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800">
              <span className="text-slate-500 block uppercase text-[10px]">
                Complete Credits
              </span>
              <span className="text-base font-bold text-emerald-400">
                {formatNumber(quality.games_with_complete_credits)}
              </span>
              <span className="text-[10px] text-slate-500 block mt-0.5">
                Full dev, pub & genre
              </span>
            </div>
          </div>

          {/* Breakdown of issue counts */}
          {quality.issue_breakdown.length > 0 && (
            <div className="space-y-3">
              <h4 className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
                Quality Issues Audit Log
              </h4>
              <div className="overflow-x-auto">
                <table className="w-full text-left text-xs text-slate-300">
                  <thead className="bg-slate-950/70 border-b border-slate-800 text-[11px] font-semibold text-slate-400 tracking-wider uppercase">
                    <tr>
                      <th className="px-4 py-2.5">Issue Classification</th>
                      <th className="px-4 py-2.5">Severity Level</th>
                      <th className="px-4 py-2.5 text-right">
                        Affected Records
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-800/60">
                    {quality.issue_breakdown.map((item, idx) => (
                      <tr
                        key={idx}
                        className="hover:bg-slate-800/30 transition"
                      >
                        <td className="px-4 py-2.5 font-mono text-slate-200 capitalize">
                          {item.issue_type.replace(/_/g, " ")}
                        </td>
                        <td className="px-4 py-2.5">
                          <Badge
                            variant={
                              item.severity === "error"
                                ? "error"
                                : item.severity === "warning"
                                  ? "warning"
                                  : "neutral"
                            }
                          >
                            {item.severity.toUpperCase()}
                          </Badge>
                        </td>
                        <td className="px-4 py-2.5 text-right font-semibold text-slate-100">
                          {formatNumber(item.count)}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
