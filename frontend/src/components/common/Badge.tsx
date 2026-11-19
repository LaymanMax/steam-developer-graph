import React from "react";

interface BadgeProps {
  children: React.ReactNode;
  variant?:
    | "developer"
    | "publisher"
    | "game"
    | "sparse"
    | "playtest"
    | "neutral"
    | "success"
    | "warning"
    | "error";
  size?: "sm" | "md";
}

export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = "neutral",
  size = "sm",
}) => {
  const baseClasses = "inline-flex items-center font-medium rounded-full";
  const sizeClasses =
    size === "sm" ? "px-2 py-0.5 text-xs" : "px-2.5 py-1 text-sm";

  const variantClasses = {
    developer: "bg-blue-500/15 text-blue-400 border border-blue-500/30",
    publisher:
      "bg-emerald-500/15 text-emerald-400 border border-emerald-500/30",
    game: "bg-amber-500/15 text-amber-400 border border-amber-500/30",
    sparse: "bg-orange-500/15 text-orange-400 border border-orange-500/30",
    playtest: "bg-purple-500/15 text-purple-400 border border-purple-500/30",
    neutral: "bg-slate-800 text-slate-300 border border-slate-700",
    success: "bg-emerald-500/15 text-emerald-400 border border-emerald-500/30",
    warning: "bg-amber-500/15 text-amber-400 border border-amber-500/30",
    error: "bg-rose-500/15 text-rose-400 border border-rose-500/30",
  }[variant];

  return (
    <span className={`${baseClasses} ${sizeClasses} ${variantClasses}`}>
      {children}
    </span>
  );
};
