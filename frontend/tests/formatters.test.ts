import { describe, expect, it } from "vitest";
import {
  formatEntropy,
  formatNumber,
  formatOwnerMidpoint,
  formatOwnerRange,
  formatPercent,
  formatYearRange,
} from "../src/utils/formatters";

describe("Formatters", () => {
  it("formats numbers with comma separators", () => {
    expect(formatNumber(0)).toBe("0");
    expect(formatNumber(1250)).toBe("1,250");
    expect(formatNumber(124146)).toBe("124,146");
    expect(formatNumber(undefined)).toBe("—");
  });

  it("formats owner ranges properly", () => {
    expect(formatOwnerRange(0, 0)).toBe("0 – 0");
    expect(formatOwnerRange(100000, 200000)).toBe("100,000 – 200,000");
    expect(formatOwnerRange(undefined, 50000)).toBe("—");
  });

  it("formats owner midpoints with tildes", () => {
    expect(formatOwnerMidpoint(150000)).toBe("~150,000");
    expect(formatOwnerMidpoint(0)).toBe("~0");
    expect(formatOwnerMidpoint(undefined)).toBe("—");
  });

  it("formats year ranges", () => {
    expect(formatYearRange(2010, 2024)).toBe("2010 – 2024");
    expect(formatYearRange(2020, 2020)).toBe("2020");
    expect(formatYearRange(undefined, 2024)).toBe("2024");
    expect(formatYearRange(undefined, undefined)).toBe("—");
  });

  it("formats percentages and entropy values", () => {
    expect(formatPercent(0.854)).toBe("85.4%");
    expect(formatEntropy(2.351)).toBe("2.35");
    expect(formatEntropy(undefined)).toBe("—");
  });
});
