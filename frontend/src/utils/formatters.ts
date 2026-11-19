export function formatNumber(num: number | undefined | null): string {
  if (num === undefined || num === null) return "—";
  return num.toLocaleString();
}

export function formatCompactNumber(num: number | undefined | null): string {
  if (num === undefined || num === null) return "0";
  if (num >= 1_000_000_000) {
    return `${(num / 1_000_000_000).toFixed(1)}B`;
  }
  if (num >= 1_000_000) {
    return `${(num / 1_000_000).toFixed(1)}M`;
  }
  if (num >= 1_000) {
    return `${(num / 1_000).toFixed(1)}K`;
  }
  return num.toString();
}

export function formatPrice(price: number | undefined | null): string {
  if (price === undefined || price === null || price === 0) return "Free";
  return `$${price.toFixed(2)}`;
}

export function formatPercentage(val: number | undefined | null): string {
  if (val === undefined || val === null) return "0.0%";
  return `${val.toFixed(1)}%`;
}

export function formatPercent(val: number | undefined | null): string {
  if (val === undefined || val === null) return "—";
  return `${(val * 100).toFixed(1)}%`;
}

export function formatPlaytimeHours(
  minutes: number | undefined | null,
): string {
  if (!minutes) return "0h";
  const hours = Math.round(minutes / 60);
  return `${formatNumber(hours)}h`;
}

export function formatOwnerRange(
  lower: number | undefined | null,
  upper: number | undefined | null,
): string {
  if (
    lower === undefined ||
    lower === null ||
    upper === undefined ||
    upper === null
  ) {
    return "—";
  }
  return `${lower.toLocaleString()} – ${upper.toLocaleString()}`;
}

export function formatOwnerMidpoint(
  midpoint: number | undefined | null,
): string {
  if (midpoint === undefined || midpoint === null) return "—";
  return `~${midpoint.toLocaleString()}`;
}

export function formatYearRange(
  firstYear: number | undefined | null,
  latestYear: number | undefined | null,
): string {
  if (!firstYear && !latestYear) return "—";
  if (firstYear && !latestYear) return `${firstYear}`;
  if (!firstYear && latestYear) return `${latestYear}`;
  if (firstYear === latestYear) return `${firstYear}`;
  return `${firstYear} – ${latestYear}`;
}

export function formatEntropy(entropy: number | undefined | null): string {
  if (entropy === undefined || entropy === null) return "—";
  return entropy.toFixed(2);
}
