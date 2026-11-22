import {
  DatasetStatus,
  DeveloperCollaboration,
  DeveloperListItem,
  DeveloperProfile,
  GameDetail,
  GameSummaryItem,
  GenreYearTrend,
  HistoricalYearActivity,
  MergeHistoryItem,
  OverviewSummary,
  PaginatedResponse,
  PublisherDeveloperRelation,
  PublisherListItem,
  PublisherProfile,
  QualityReport,
  ResolutionCandidateItem,
  ResolutionReport,
  SearchResultItem,
  Subgraph,
} from "../types";

const API_BASE =
  (import.meta as any).env?.VITE_API_URL || "http://127.0.0.1:3001";

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const url = `${API_BASE}${path}`;
  const res = await fetch(url, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...options?.headers,
    },
  });

  const json: ApiResponse<T> = await res.json();
  if (!json.success || json.error) {
    throw new Error(json.error || `HTTP error ${res.status}`);
  }

  return json.data as T;
}

export const api = {
  // Dataset & Ingestion
  getDatasetStatus: () => request<DatasetStatus>("/api/dataset/status"),
  getDataQualityReport: () => request<QualityReport>("/api/dataset/quality"),
  triggerImport: (filePath?: string) =>
    request<{ rows_imported: number; duration_seconds: number }>(
      "/api/dataset/import",
      {
        method: "POST",
        body: JSON.stringify({ file_path: filePath }),
      },
    ),
  triggerDownload: (sourceUrl?: string) =>
    request<string>("/api/dataset/download", {
      method: "POST",
      body: JSON.stringify({ source_url: sourceUrl }),
    }),

  // Overview & Search
  getOverviewSummary: () => request<OverviewSummary>("/api/overview/summary"),
  searchGlobal: (q: string, limit = 12) =>
    request<SearchResultItem[]>(
      `/api/search?q=${encodeURIComponent(q)}&limit=${limit}`,
    ),

  // Developers
  listDevelopers: (params: {
    page?: number;
    pageSize?: number;
    search?: string;
    genre?: string;
    minGames?: number;
    sortBy?: string;
    sortDesc?: boolean;
  }) => {
    const q = new URLSearchParams();
    if (params.page) q.set("page", params.page.toString());
    if (params.pageSize) q.set("page_size", params.pageSize.toString());
    if (params.search) q.set("search", params.search);
    if (params.genre) q.set("genre", params.genre);
    if (params.minGames) q.set("min_games", params.minGames.toString());
    if (params.sortBy) q.set("sort_by", params.sortBy);
    if (params.sortDesc !== undefined)
      q.set("sort_desc", params.sortDesc.toString());
    return request<PaginatedResponse<DeveloperListItem>>(
      `/api/developers?${q.toString()}`,
    );
  },
  getDeveloperProfile: (id: number) =>
    request<DeveloperProfile>(`/api/developers/${id}`),
  getDeveloperNetwork: (id: number) =>
    request<Subgraph>(`/api/developers/${id}/network`),

  // Publishers
  listPublishers: (params: {
    page?: number;
    pageSize?: number;
    search?: string;
    genre?: string;
    minGames?: number;
    sortBy?: string;
    sortDesc?: boolean;
  }) => {
    const q = new URLSearchParams();
    if (params.page) q.set("page", params.page.toString());
    if (params.pageSize) q.set("page_size", params.pageSize.toString());
    if (params.search) q.set("search", params.search);
    if (params.genre) q.set("genre", params.genre);
    if (params.minGames) q.set("min_games", params.minGames.toString());
    if (params.sortBy) q.set("sort_by", params.sortBy);
    if (params.sortDesc !== undefined)
      q.set("sort_desc", params.sortDesc.toString());
    return request<PaginatedResponse<PublisherListItem>>(
      `/api/publishers?${q.toString()}`,
    );
  },
  getPublisherProfile: (id: number) =>
    request<PublisherProfile>(`/api/publishers/${id}`),
  getPublisherNetwork: (id: number) =>
    request<Subgraph>(`/api/publishers/${id}/network`),

  // Games
  listGames: (params: {
    page?: number;
    pageSize?: number;
    search?: string;
    genre?: string;
    developerId?: number;
    publisherId?: number;
    minYear?: number;
    maxYear?: number;
    minOwners?: number;
    maxOwners?: number;
    excludeSparse?: boolean;
    excludePlaytests?: boolean;
    sortBy?: string;
    sortDesc?: boolean;
  }) => {
    const q = new URLSearchParams();
    if (params.page) q.set("page", params.page.toString());
    if (params.pageSize) q.set("page_size", params.pageSize.toString());
    if (params.search) q.set("search", params.search);
    if (params.genre) q.set("genre", params.genre);
    if (params.developerId)
      q.set("developer_id", params.developerId.toString());
    if (params.publisherId)
      q.set("publisher_id", params.publisherId.toString());
    if (params.minYear) q.set("min_year", params.minYear.toString());
    if (params.maxYear) q.set("max_year", params.maxYear.toString());
    if (params.minOwners) q.set("min_owners", params.minOwners.toString());
    if (params.maxOwners) q.set("max_owners", params.maxOwners.toString());
    if (params.excludeSparse) q.set("exclude_sparse", "true");
    if (params.excludePlaytests) q.set("exclude_playtests", "true");
    if (params.sortBy) q.set("sort_by", params.sortBy);
    if (params.sortDesc !== undefined)
      q.set("sort_desc", params.sortDesc.toString());
    return request<PaginatedResponse<GameSummaryItem>>(
      `/api/games?${q.toString()}`,
    );
  },
  getGameDetails: (id: number) => request<GameDetail>(`/api/games/${id}`),
  getGameNetwork: (id: number) => request<Subgraph>(`/api/games/${id}/network`),

  // Graph
  getGraphOverview: (filters?: {
    limit?: number;
    minYear?: number;
    maxYear?: number;
  }) => {
    const q = new URLSearchParams();
    if (filters?.limit) q.set("limit", filters.limit.toString());
    if (filters?.minYear) q.set("min_year", filters.minYear.toString());
    if (filters?.maxYear) q.set("max_year", filters.maxYear.toString());
    return request<Subgraph>(`/api/graph/overview?${q.toString()}`);
  },
  queryNeighborhood: (params: {
    nodeType: "developer" | "publisher" | "game";
    id: number;
    depth?: number;
    minYear?: number;
    maxYear?: number;
    genre?: string;
    minCollaborations?: number;
    excludeSparse?: boolean;
    excludePlaytests?: boolean;
    limit?: number;
  }) => {
    const q = new URLSearchParams();
    q.set("node_type", params.nodeType);
    q.set("id", params.id.toString());
    if (params.depth) q.set("depth", params.depth.toString());
    if (params.minYear) q.set("min_year", params.minYear.toString());
    if (params.maxYear) q.set("max_year", params.maxYear.toString());
    if (params.genre) q.set("genre", params.genre);
    if (params.minCollaborations)
      q.set("min_collaborations", params.minCollaborations.toString());
    if (params.excludeSparse) q.set("exclude_sparse", "true");
    if (params.excludePlaytests) q.set("exclude_playtests", "true");
    if (params.limit) q.set("limit", params.limit.toString());
    return request<Subgraph>(`/api/graph/neighborhood?${q.toString()}`);
  },

  // Collaborations
  getTopCollaborations: (params?: {
    minShared?: number;
    genre?: string;
    minYear?: number;
    maxYear?: number;
    limit?: number;
  }) => {
    const q = new URLSearchParams();
    if (params?.minShared) q.set("min_shared", params.minShared.toString());
    if (params?.genre) q.set("genre", params.genre);
    if (params?.minYear) q.set("min_year", params.minYear.toString());
    if (params?.maxYear) q.set("max_year", params.maxYear.toString());
    if (params?.limit) q.set("limit", params.limit.toString());
    return request<DeveloperCollaboration[]>(
      `/api/collaborations?${q.toString()}`,
    );
  },
  getDeveloperPairDetails: (dev1Id: number, dev2Id: number) =>
    request<DeveloperCollaboration>(`/api/collaborations/${dev1Id}/${dev2Id}`),
  getPublisherDeveloperRelations: (params?: {
    minShared?: number;
    limit?: number;
  }) => {
    const q = new URLSearchParams();
    if (params?.minShared) q.set("min_shared", params.minShared.toString());
    if (params?.limit) q.set("limit", params.limit.toString());
    return request<PublisherDeveloperRelation[]>(
      `/api/relationships/publisher-developer?${q.toString()}`,
    );
  },

  // Historical
  getHistoricalTimeline: () =>
    request<HistoricalYearActivity[]>("/api/historical/timeline"),
  getGenreGrowth: (topN = 8) =>
    request<GenreYearTrend[]>(`/api/historical/genres?top_n=${topN}`),

  // Entity Resolution
  listCandidates: (params?: {
    entityType?: string;
    status?: string;
    minSimilarity?: number;
    page?: number;
    pageSize?: number;
  }) => {
    const q = new URLSearchParams();
    if (params?.entityType) q.set("entity_type", params.entityType);
    if (params?.status) q.set("status", params.status);
    if (params?.minSimilarity)
      q.set("min_similarity", params.minSimilarity.toString());
    if (params?.page) q.set("page", params.page.toString());
    if (params?.pageSize) q.set("page_size", params.pageSize.toString());
    return request<PaginatedResponse<ResolutionCandidateItem>>(
      `/api/entity-resolution/candidates?${q.toString()}`,
    );
  },
  getResolutionReport: () =>
    request<ResolutionReport>("/api/entity-resolution/report"),
  getMergeHistory: () =>
    request<MergeHistoryItem[]>("/api/entity-resolution/history"),
  mergeEntities: (payload: {
    entityType: string;
    sourceId: number;
    targetId: number;
    reason?: string;
  }) =>
    request<string>("/api/entity-resolution/merge", {
      method: "POST",
      body: JSON.stringify({
        entity_type: payload.entityType,
        source_id: payload.sourceId,
        target_id: payload.targetId,
        reason: payload.reason,
      }),
    }),
  rejectCandidate: (payload: {
    entityType: string;
    entityAId: number;
    entityBId: number;
  }) =>
    request<string>("/api/entity-resolution/reject", {
      method: "POST",
      body: JSON.stringify({
        entity_type: payload.entityType,
        entity_a_id: payload.entityAId,
        entity_b_id: payload.entityBId,
      }),
    }),
  markDistinct: (payload: {
    entityType: string;
    entityAId: number;
    entityBId: number;
  }) =>
    request<string>("/api/entity-resolution/mark-distinct", {
      method: "POST",
      body: JSON.stringify({
        entity_type: payload.entityType,
        entity_a_id: payload.entityAId,
        entity_b_id: payload.entityBId,
      }),
    }),
  assignAlias: (payload: {
    entityType: string;
    rawName: string;
    canonicalId: number;
  }) =>
    request<string>("/api/entity-resolution/assign-alias", {
      method: "POST",
      body: JSON.stringify({
        entity_type: payload.entityType,
        raw_name: payload.rawName,
        canonical_id: payload.canonicalId,
      }),
    }),
  undoMerge: (mergeEventId: number) =>
    request<string>("/api/entity-resolution/undo", {
      method: "POST",
      body: JSON.stringify({ merge_event_id: mergeEventId }),
    }),
  generateCandidates: () =>
    request<string>("/api/entity-resolution/generate", {
      method: "POST",
      body: JSON.stringify({}),
    }),

  // Export URLs
  getExportDevelopersUrl: (
    search?: string,
    genre?: string,
    minGames?: number,
  ) => {
    const q = new URLSearchParams();
    if (search) q.set("search", search);
    if (genre) q.set("genre", genre);
    if (minGames) q.set("min_games", minGames.toString());
    return `${API_BASE}/api/export/developers?${q.toString()}`;
  },
  getExportPublishersUrl: (
    search?: string,
    genre?: string,
    minGames?: number,
  ) => {
    const q = new URLSearchParams();
    if (search) q.set("search", search);
    if (genre) q.set("genre", genre);
    if (minGames) q.set("min_games", minGames.toString());
    return `${API_BASE}/api/export/publishers?${q.toString()}`;
  },
  getExportCollaborationsUrl: (
    minShared = 2,
    genre?: string,
    minYear?: number,
    maxYear?: number,
  ) => {
    const q = new URLSearchParams();
    q.set("min_shared", minShared.toString());
    if (genre) q.set("genre", genre);
    if (minYear) q.set("min_year", minYear.toString());
    if (maxYear) q.set("max_year", maxYear.toString());
    return `${API_BASE}/api/export/collaborations?${q.toString()}`;
  },
  getExportGraphUrl: (nodeType?: string, id?: number) => {
    const q = new URLSearchParams();
    if (nodeType && id) {
      q.set("node_type", nodeType);
      q.set("id", id.toString());
    }
    return `${API_BASE}/api/export/graph?${q.toString()}`;
  },
};

export const apiClient = api;
