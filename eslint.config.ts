import js from "@eslint/js";
import tseslint from "typescript-eslint";
import globals from "globals";

export default [
  { ignores: ["dist/**", "src-tauri/**", "node_modules/**"] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    languageOptions: {
      globals: globals.browser,
    },
    rules: {
      "no-restricted-imports": [
        "error",
        {
          paths: ["@tauri-apps/api", "@tauri-apps/api/core"],
          patterns: ["@tauri-apps/api/*"],
        },
      ],
    },
  },
  {
    files: ["src/api/**"],
    rules: {
      "no-restricted-imports": "off",
    },
  },
];
