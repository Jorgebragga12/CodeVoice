import "@testing-library/jest-dom/vitest";

import { afterEach } from "vitest";
import { cleanup } from "@testing-library/react";

// `vitest.config.ts` não usa `test.globals: true`, então o `afterEach` global que o
// `@testing-library/react` procura pra registrar sua limpeza automática entre testes não
// existe — sem isso, o DOM de um `render()` vaza pro próximo teste do mesmo arquivo. Só não
// dava pra notar antes porque cada arquivo de teste tinha um único `it()`.
afterEach(() => {
  cleanup();
});
