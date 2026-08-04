/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_EVALUATE_URL: string;
  readonly VITE_APP_VERSION: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

interface Window {
  Prism?: {
    highlightAllUnder(element: Element): void;
  };
}
