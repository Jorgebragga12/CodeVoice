import { ErrorBoundary } from "./ErrorBoundary";
import { Home } from "../windows/main/Home";

export function App() {
  return (
    <ErrorBoundary>
      <Home />
    </ErrorBoundary>
  );
}
