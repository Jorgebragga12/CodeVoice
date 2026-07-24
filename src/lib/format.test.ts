import { describe, expect, it } from "vitest";
import { formatDuration } from "./format";

describe("formatDuration", () => {
  it("formats sub-minute durations with a padded seconds field", () => {
    expect(formatDuration(0)).toBe("0:00");
    expect(formatDuration(5_000)).toBe("0:05");
    expect(formatDuration(9_000)).toBe("0:09");
  });

  it("formats durations past a minute", () => {
    expect(formatDuration(65_000)).toBe("1:05");
    expect(formatDuration(600_000)).toBe("10:00");
  });

  it("floors partial seconds instead of rounding up", () => {
    expect(formatDuration(1_999)).toBe("0:01");
  });

  it("treats negative input as zero", () => {
    expect(formatDuration(-1_000)).toBe("0:00");
  });
});
