import React, { useEffect, useState } from "react";
import { api } from "./api/client";
import { Header } from "./components/layout/Header";
import { Sidebar, ViewType } from "./components/layout/Sidebar";
import { DatasetStatus } from "./types";

// Views
import { CollaborationLabView } from "./views/CollaborationLabView";
import { DatasetStatusView } from "./views/DatasetStatusView";
import { DeveloperDirectoryView } from "./views/DeveloperDirectoryView";
import { DeveloperProfileView } from "./views/DeveloperProfileView";
import { EntityResolutionView } from "./views/EntityResolutionView";
import { GameDetailView } from "./views/GameDetailView";
import { GamesDirectoryView } from "./views/GamesDirectoryView";
import { HistoricalAnalysisView } from "./views/HistoricalAnalysisView";
import { NetworkExplorerView } from "./views/NetworkExplorerView";
import { OverviewView } from "./views/OverviewView";
import { PublisherDirectoryView } from "./views/PublisherDirectoryView";
import { PublisherProfileView } from "./views/PublisherProfileView";

export const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewType>("overview");
  const [selectedDeveloperId, setSelectedDeveloperId] = useState<number | null>(
    null,
  );
  const [selectedPublisherId, setSelectedPublisherId] = useState<number | null>(
    null,
  );
  const [selectedGameId, setSelectedGameId] = useState<number | null>(null);
  const [initialEntity, setInitialEntity] = useState<{
    type: "developer" | "publisher" | "game";
    id: number;
  } | null>(null);

  const [datasetStatus, setDatasetStatus] = useState<DatasetStatus | null>(
    null,
  );

  useEffect(() => {
    fetchStatus();
  }, []);

  const fetchStatus = async () => {
    try {
      const res = await api.getDatasetStatus();
      setDatasetStatus(res);
    } catch (err) {
      console.error("Failed to query dataset status", err);
    }
  };

  const handleNavigate = (view: ViewType) => {
    setCurrentView(view);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const handleSelectEntity = (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => {
    if (type === "developer") {
      setSelectedDeveloperId(id);
      setCurrentView("developer-profile");
    } else if (type === "publisher") {
      setSelectedPublisherId(id);
      setCurrentView("publisher-profile");
    } else if (type === "game") {
      setSelectedGameId(id);
      setCurrentView("game-detail");
    }
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const datasetHasData = Boolean(datasetStatus && datasetStatus.has_data);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-slate-950 text-slate-100 font-sans">
      {/* Primary Sidebar */}
      <Sidebar
        currentView={currentView}
        onNavigate={handleNavigate}
        datasetHasData={datasetHasData}
      />

      {/* Main Content Pane */}
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
        {/* Global Application Header */}
        <Header
          onSelectEntity={handleSelectEntity}
          onNavigate={handleNavigate}
          datasetHasData={datasetHasData}
        />

        {/* Viewport Canvas */}
        <main className="flex-1 overflow-y-auto p-6 md:p-8 bg-slate-950">
          <div className="max-w-7xl mx-auto">
            {currentView === "overview" && (
              <OverviewView
                onNavigate={handleNavigate}
                onSelectEntity={handleSelectEntity}
              />
            )}

            {currentView === "network" && (
              <NetworkExplorerView
                onNavigate={handleNavigate}
                onSelectEntity={handleSelectEntity}
                initialEntity={initialEntity}
              />
            )}

            {currentView === "developers" && (
              <DeveloperDirectoryView
                onSelectDeveloper={(id) => handleSelectEntity("developer", id)}
              />
            )}

            {currentView === "developer-profile" && selectedDeveloperId && (
              <DeveloperProfileView
                developerId={selectedDeveloperId}
                onBack={() => handleNavigate("developers")}
                onSelectEntity={handleSelectEntity}
              />
            )}

            {currentView === "publishers" && (
              <PublisherDirectoryView
                onSelectPublisher={(id) => handleSelectEntity("publisher", id)}
              />
            )}

            {currentView === "publisher-profile" && selectedPublisherId && (
              <PublisherProfileView
                publisherId={selectedPublisherId}
                onBack={() => handleNavigate("publishers")}
                onSelectEntity={handleSelectEntity}
              />
            )}

            {currentView === "games" && (
              <GamesDirectoryView
                onSelectGame={(appId) => handleSelectEntity("game", appId)}
                onSelectDeveloper={(id) => handleSelectEntity("developer", id)}
                onSelectPublisher={(id) => handleSelectEntity("publisher", id)}
              />
            )}

            {currentView === "game-detail" && selectedGameId && (
              <GameDetailView
                gameId={selectedGameId}
                onBack={() => handleNavigate("games")}
                onSelectEntity={handleSelectEntity}
              />
            )}

            {currentView === "collaboration" && (
              <CollaborationLabView onSelectEntity={handleSelectEntity} />
            )}

            {currentView === "historical" && <HistoricalAnalysisView />}

            {currentView === "resolution" && <EntityResolutionView />}

            {currentView === "dataset" && <DatasetStatusView />}
          </div>
        </main>
      </div>
    </div>
  );
};

export default App;
