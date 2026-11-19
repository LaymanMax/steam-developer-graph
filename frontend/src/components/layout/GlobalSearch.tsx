import { Briefcase, Building2, Gamepad2, Search, X } from "lucide-react";
import React, { useEffect, useRef, useState } from "react";
import { api } from "../../api/client";
import { SearchResultItem } from "../../types";

interface GlobalSearchProps {
  onSelectEntity: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const GlobalSearch: React.FC<GlobalSearchProps> = ({
  onSelectEntity,
}) => {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResultItem[]>([]);
  const [isOpen, setIsOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      setIsOpen(false);
      return;
    }

    const timer = setTimeout(async () => {
      setIsLoading(true);
      try {
        const data = await api.searchGlobal(query.trim(), 8);
        setResults(data);
        setIsOpen(true);
      } catch (err) {
        console.error("Search failed:", err);
      } finally {
        setIsLoading(false);
      }
    }, 200);

    return () => clearTimeout(timer);
  }, [query]);

  // Click outside listener
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (
        containerRef.current &&
        !containerRef.current.contains(e.target as Node)
      ) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleSelect = (item: SearchResultItem) => {
    onSelectEntity(item.entity_type, item.id);
    setIsOpen(false);
    setQuery("");
  };

  return (
    <div ref={containerRef} className="relative w-full max-w-md">
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-slate-500">
          <Search className="w-4 h-4" />
        </div>
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onFocus={() => query.trim() && setIsOpen(true)}
          placeholder="Search developers, publishers, games..."
          className="w-full pl-9 pr-8 py-1.5 bg-slate-900/90 border border-slate-700/80 rounded-lg text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition"
        />
        {query && (
          <button
            onClick={() => setQuery("")}
            className="absolute inset-y-0 right-0 pr-2.5 flex items-center text-slate-500 hover:text-slate-300"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        )}
      </div>

      {isOpen && (
        <div className="absolute left-0 right-0 mt-1.5 bg-slate-900 border border-slate-700/90 rounded-xl shadow-2xl overflow-hidden z-50 divide-y divide-slate-800">
          {isLoading ? (
            <div className="p-4 text-xs text-slate-400 text-center">
              Searching ecosystem...
            </div>
          ) : results.length === 0 ? (
            <div className="p-4 text-xs text-slate-400 text-center">
              No entities found for "{query}"
            </div>
          ) : (
            <div className="max-h-80 overflow-y-auto divide-y divide-slate-800/60">
              {results.map((item) => {
                const icon = {
                  developer: (
                    <Briefcase className="w-3.5 h-3.5 text-blue-400" />
                  ),
                  publisher: (
                    <Building2 className="w-3.5 h-3.5 text-emerald-400" />
                  ),
                  game: <Gamepad2 className="w-3.5 h-3.5 text-amber-400" />,
                }[item.entity_type];

                const typeLabel = {
                  developer: "Developer",
                  publisher: "Publisher",
                  game: "Game",
                }[item.entity_type];

                return (
                  <button
                    key={`${item.entity_type}-${item.id}`}
                    onClick={() => handleSelect(item)}
                    className="w-full text-left px-3.5 py-2.5 hover:bg-slate-800/80 flex items-start gap-3 transition group"
                  >
                    <div className="p-1.5 rounded-lg bg-slate-800 border border-slate-700 mt-0.5">
                      {icon}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center justify-between gap-2">
                        <span className="text-sm font-medium text-slate-200 group-hover:text-blue-400 truncate">
                          {item.label}
                        </span>
                        <span className="text-[10px] font-semibold tracking-wider uppercase text-slate-500 bg-slate-800/80 px-1.5 py-0.5 rounded border border-slate-700/50">
                          {typeLabel}
                        </span>
                      </div>
                      {item.sub_label && (
                        <p className="text-xs text-slate-400 truncate mt-0.5">
                          {item.sub_label}
                        </p>
                      )}
                      {item.matched_alias && (
                        <p className="text-[11px] text-amber-400/90 italic truncate mt-0.5">
                          {item.matched_alias}
                        </p>
                      )}
                    </div>
                  </button>
                );
              })}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
