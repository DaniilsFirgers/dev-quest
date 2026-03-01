## ES Linter

It is a static code analysis tool for Javascript/Typescript that helps to find problems in the code before it runs.

### Why is it useful?

1. Catches bugs early (missing await, duplicate imports, using undefined variables, etc)
2. Enforces code consistency (in the teams, everyone has different coding style, ESLint ensures consitency among team members)
3. Improves code quality (with advanced ESLint features, you can enforce best practices + use prettier alongside)
4. Helps team scale (no more debates over code style, let the linter decide + code reviews become easier)
5. ESLint works with CI/CD pipelines, VsCode (real time) and Git hooks (prevent bad code)

### What to install as dev dependencies?

1. `eslint`

Core linter. Everything else depends on this.

2. `@typescript-eslint/parser`

Tells ESLint how to read TypeScript code. Without this, ESLint can’t parse .ts or .tsx files.

3. `@typescript-eslint/eslint-plugin`

Provides TypeScript-specific rules, like enforcing types, consistent type imports, or avoiding any.

3. `eslint-config-prettier`

Disables ESLint rules that conflict with Prettier formatting. Ensures linting doesn’t fight automatic formatting.

4. `eslint-plugin-import`

Helps manage ES module imports: checks for missing files, incorrect paths, or misordered imports.

5. `eslint-import-resolver-typescript`

Works with eslint-plugin-import so it can understand TypeScript paths, including tsconfig.json paths.

6. `eslint-plugin-security`

Warns about potential security risks in your code (like eval, insecure random numbers, or unsafe regex).

7. `eslint-plugin-unused-imports`

Automatically flags or removes unused imports, helping keep code clean.

8. `prettier`

An automatic code formatter. Ensures consistent indentation, quotes, semicolons, and line length across the codebase.

9. `eslint-config-prettier`

Disables ESLint rules that conflict with Prettier formatting. Ensures linting doesn’t fight automatic formatting.

10. `eslint-plugin-prettier`

Runs Prettier as an ESLint rule. Linting errors are shown if code is not properly formatted.

**Example of a solid typescript/node.js ESLint config**:

```
// eslint.config.js
import tseslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import importPlugin from "eslint-plugin-import";
import unusedImports from "eslint-plugin-unused-imports";
import security from "eslint-plugin-security";
import prettierPlugin from "eslint-plugin-prettier";

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
      prettier: prettierPlugin,
    },
    extends: [
      "plugin:prettier/recommended", // Enables prettier plugin and config
    ],
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

      // 🚫 Node specific
      "node/no-process-exit": "error",

      // 🎨 Prettier as lint rule
      "prettier/prettier": [
        "error",
        {
          singleQuote: true,
          semi: true,
          printWidth: 100,
          trailingComma: "all",
        },
      ],
    },
  },
];
```
