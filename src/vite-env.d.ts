/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_MLAD_PREVIEW_LABEL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
