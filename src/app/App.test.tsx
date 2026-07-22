import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { App } from "./App";

describe("App", () => {
  it("renders the CodeVoice home screen", () => {
    render(<App />);
    expect(screen.getByText("CodeVoice")).toBeInTheDocument();
  });
});
