import { Maximize2, ZoomIn, ZoomOut } from "lucide-react";
import React, { useCallback, useEffect, useRef, useState } from "react";
import ForceGraph2D, { ForceGraphMethods } from "react-force-graph-2d";
import { GraphLink, GraphNode, Subgraph } from "../../types";
import { GraphInspector } from "./GraphInspector";
import { GraphLegend } from "./GraphLegend";

interface NetworkGraphProps {
  data: Subgraph;
  width?: number;
  height?: number;
  onExpandNode?: (node: GraphNode) => void;
  onNavigateToEntity?: (
    type: "developer" | "publisher" | "game",
    id: number,
  ) => void;
}

export const NetworkGraph: React.FC<NetworkGraphProps> = ({
  data,
  width,
  height,
  onExpandNode,
  onNavigateToEntity,
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const fgRef = useRef<ForceGraphMethods<GraphNode, GraphLink> | undefined>(
    undefined,
  );
  const [dimensions, setDimensions] = useState({
    width: width || 800,
    height: height || 600,
  });
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [selectedLink, setSelectedLink] = useState<GraphLink | null>(null);
  const [hoveredNode, setHoveredNode] = useState<GraphNode | null>(null);

  // Resize handler
  useEffect(() => {
    if (width && height) {
      setDimensions({ width, height });
      return;
    }

    const updateDimensions = () => {
      if (containerRef.current) {
        const { clientWidth, clientHeight } = containerRef.current;
        setDimensions({
          width: clientWidth || 800,
          height: clientHeight || 600,
        });
      }
    };

    updateDimensions();
    window.addEventListener("resize", updateDimensions);
    return () => window.removeEventListener("resize", updateDimensions);
  }, [width, height]);

  // Center on load or data change
  useEffect(() => {
    if (fgRef.current && data.nodes.length > 0) {
      setTimeout(() => {
        fgRef.current?.zoomToFit(400, 60);
      }, 300);
    }
  }, [data]);

  const handleNodeClick = useCallback((node: GraphNode) => {
    setSelectedNode(node);
    setSelectedLink(null);
    if (fgRef.current && node.x !== undefined && node.y !== undefined) {
      fgRef.current.centerAt(node.x, node.y, 400);
      fgRef.current.zoom(1.8, 400);
    }
  }, []);

  const handleLinkClick = useCallback((link: GraphLink) => {
    setSelectedLink(link);
    setSelectedNode(null);
  }, []);

  const handleZoomIn = () => {
    if (fgRef.current) {
      fgRef.current.zoom(fgRef.current.zoom() * 1.3, 300);
    }
  };

  const handleZoomOut = () => {
    if (fgRef.current) {
      fgRef.current.zoom(fgRef.current.zoom() / 1.3, 300);
    }
  };

  const handleZoomToFit = () => {
    if (fgRef.current) {
      fgRef.current.zoomToFit(400, 50);
    }
  };

  // Canvas Node rendering
  const drawNode = useCallback(
    (node: GraphNode, ctx: CanvasRenderingContext2D, globalScale: number) => {
      const isSelected = selectedNode?.id === node.id;
      const isHovered = hoveredNode?.id === node.id;

      const baseRadius = Math.max(node.size * 0.7, 4);
      const radius = isSelected || isHovered ? baseRadius * 1.25 : baseRadius;

      // Color scheme
      let fillColor = "#3b82f6"; // Developer: Blue
      let ringColor = "rgba(59, 130, 246, 0.4)";
      if (node.node_type === "publisher") {
        fillColor = "#10b981"; // Publisher: Emerald
        ringColor = "rgba(16, 185, 129, 0.4)";
      } else if (node.node_type === "game") {
        fillColor = "#f59e0b"; // Game: Amber
        ringColor = "rgba(245, 158, 11, 0.4)";
      }

      // Outer focus halo
      if (isSelected || isHovered) {
        ctx.beginPath();
        ctx.arc(
          node.x || 0,
          node.y || 0,
          radius + 4 / globalScale,
          0,
          2 * Math.PI,
          false,
        );
        ctx.fillStyle = ringColor;
        ctx.fill();
      }

      // Main circle
      ctx.beginPath();
      ctx.arc(node.x || 0, node.y || 0, radius, 0, 2 * Math.PI, false);
      ctx.fillStyle = fillColor;
      ctx.fill();
      ctx.lineWidth = 1.5 / globalScale;
      ctx.strokeStyle = "#0f172a";
      ctx.stroke();

      // Label rendering at zoom levels or on select/hover
      const shouldDrawLabel =
        globalScale > 1.2 || isSelected || isHovered || node.size > 14;
      if (shouldDrawLabel) {
        const fontSize = Math.max(10 / globalScale, 3);
        ctx.font = `${isSelected ? "600" : "400"} ${fontSize}px sans-serif`;
        ctx.textAlign = "center";
        ctx.textBaseline = "top";

        // Draw label background pill
        const text =
          node.label.length > 20
            ? `${node.label.substring(0, 18)}…`
            : node.label;
        const textWidth = ctx.measureText(text).width;
        const pad = 2 / globalScale;
        const yOffset = radius + 2 / globalScale;

        ctx.fillStyle = "rgba(15, 23, 42, 0.85)";
        ctx.fillRect(
          (node.x || 0) - textWidth / 2 - pad,
          (node.y || 0) + yOffset - pad,
          textWidth + pad * 2,
          fontSize + pad * 2,
        );

        ctx.fillStyle = isSelected ? "#60a5fa" : "#f1f5f9";
        ctx.fillText(text, node.x || 0, (node.y || 0) + yOffset);
      }
    },
    [selectedNode, hoveredNode],
  );

  return (
    <div
      ref={containerRef}
      className="relative w-full h-full bg-slate-950 overflow-hidden select-none"
    >
      <ForceGraph2D
        ref={fgRef}
        width={dimensions.width}
        height={dimensions.height}
        graphData={{
          nodes: data.nodes,
          links: data.links,
        }}
        nodeId="id"
        nodeCanvasObject={drawNode}
        nodePointerAreaPaint={(
          node: GraphNode,
          color: string,
          ctx: CanvasRenderingContext2D,
        ) => {
          ctx.beginPath();
          ctx.arc(
            node.x || 0,
            node.y || 0,
            Math.max(node.size * 0.7, 6),
            0,
            2 * Math.PI,
            false,
          );
          ctx.fillStyle = color;
          ctx.fill();
        }}
        onNodeClick={handleNodeClick}
        onNodeHover={(node) => setHoveredNode(node || null)}
        onLinkClick={handleLinkClick}
        linkWidth={(link: GraphLink) => Math.min((link.weight || 1) * 1.5, 5)}
        linkColor={(link: GraphLink) => {
          if (selectedLink === link) return "#38bdf8";
          if (link.relationship === "collaborated")
            return "rgba(168, 85, 247, 0.45)";
          return "rgba(71, 85, 105, 0.35)";
        }}
        linkDirectionalParticles={1}
        linkDirectionalParticleSpeed={0.005}
        linkDirectionalParticleWidth={2}
        cooldownTicks={120}
        d3AlphaDecay={0.02}
        d3VelocityDecay={0.3}
      />

      {/* Floating Controls Overlay */}
      <div className="absolute top-4 left-4 z-10 flex flex-col gap-1.5 bg-slate-900/80 backdrop-blur-md p-1.5 rounded-xl border border-slate-800 shadow-lg">
        <button
          onClick={handleZoomIn}
          className="p-2 rounded-lg hover:bg-slate-800 text-slate-300 hover:text-white transition"
          title="Zoom In"
        >
          <ZoomIn className="w-4 h-4" />
        </button>
        <button
          onClick={handleZoomOut}
          className="p-2 rounded-lg hover:bg-slate-800 text-slate-300 hover:text-white transition"
          title="Zoom Out"
        >
          <ZoomOut className="w-4 h-4" />
        </button>
        <button
          onClick={handleZoomToFit}
          className="p-2 rounded-lg hover:bg-slate-800 text-slate-300 hover:text-white transition"
          title="Fit to View"
        >
          <Maximize2 className="w-4 h-4" />
        </button>
      </div>

      {/* Legend */}
      <div className="absolute bottom-4 left-4 z-10">
        <GraphLegend />
      </div>

      {/* Inspector Drawer */}
      <div className="absolute top-4 right-4 z-10">
        <GraphInspector
          selectedNode={selectedNode}
          selectedLink={selectedLink}
          onClose={() => {
            setSelectedNode(null);
            setSelectedLink(null);
          }}
          onExpandNode={onExpandNode}
          onNavigateToEntity={onNavigateToEntity}
        />
      </div>

      {/* Truncated subgraph disclaimer pill */}
      {data.is_truncated && (
        <div className="absolute bottom-4 right-4 z-10 bg-slate-900/90 border border-slate-800/90 text-slate-400 text-[11px] px-3 py-1.5 rounded-lg shadow-md">
          Showing bounded local neighborhood ({data.nodes.length} nodes)
        </div>
      )}
    </div>
  );
};
