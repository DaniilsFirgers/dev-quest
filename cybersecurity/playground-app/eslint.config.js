// eslint.config.js
import tseslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import importPlugin from "eslint-plugin-import";
import unusedImports from "eslint-plugin-unused-imports";
import security from "eslint-plugin-security";
import eslintPluginPrettier from "eslint-plugin-prettier";
import nodePlugin from "eslint-plugin-node";

export default [
  {
    files: ["**/*.ts"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: "./tsconfig.json",
      },
    },
    plugins: {
      "@typescript-eslint": tseslint,
      import: importPlugin,
      "unused-imports": unusedImports,
      security,
      prettier: eslintPluginPrettier,
      node: nodePlugin,
    },
    settings: {
      "import/resolver": {
        typescript: {
          project: "./tsconfig.json",
        },
      },
    },
    rules: {
      // 🔒 Safety
      eqeqeq: "error",
      "no-eval": "error",
      "no-implied-eval": "error",

      // 🧹 Unused imports
      "unused-imports/no-unused-imports": "error",

      // 🧠 TypeScript rules
      "@typescript-eslint/no-explicit-any": "warn",
      "@typescript-eslint/consistent-type-imports": "error",
      "@typescript-eslint/no-floating-promises": "error",
      "@typescript-eslint/await-thenable": "error",
      "@typescript-eslint/no-misused-promises": "error",
      "@typescript-eslint/no-unnecessary-type-assertion": "warn",

      // 📦 Import hygiene
      "import/no-unresolved": "error",

      // 🔐 Security plugin
      "security/detect-object-injection": "warn",

      // 🎨 Prettier as lint rule
      "prettier/prettier": [
        "error",
        {
          singleQuote: false,
          semi: true,
          printWidth: 100,
          trailingComma: "all",
          tabWidth: 2,
          useTabs: false,
        },
      ],
    },
  },
];
