import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import reactPlugin from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";

export default [
  { files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"] },
  { languageOptions: { parserOptions: { ecmaFeatures: { jsx: true } } } },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.all,
  ...tseslint.configs.recommendedTypeChecked,
  reactPlugin.configs.flat.recommended,
  reactHooks.configs.flat.recommended,
  {
    rules: {
      "no-magic-numbers": "off",
      "no-plusplus": "off",
      "no-void": ["error", { allowAsStatement: true }],
      "@typescript-eslint/no-misused-promises": [
        "error",
        { checksVoidReturn: { attributes: false } },
      ],
      "@typescript-eslint/restrict-template-expressions": [
        "error",
        { allowArray: true },
      ],
      "id-length": "off",
      "no-ternary": "off",
      "func-style": "off",
      "sort-keys": "off",
      "no-undefined": "off",
      "max-lines-per-function": "off",
      "default-case": "off",
      "max-statements": "off",
      "@typescript-eslint/switch-exhaustiveness-check": [
        "error",
        {
          allowDefaultCaseForExhaustiveSwitch: false,
          requireDefaultForNonUnion: true,
        },
      ],
      "no-duplicate-imports": [
        "error",
        { allowSeparateTypeImports: true, includeExports: true },
      ],
      "capitalized-comments": [
        "error",
        "always",
        { ignoreConsecutiveComments: true },
      ],
      "one-var": "off",
      "init-declarations": "off",
      "no-console": "off",
      "no-nested-ternary": "off",
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
    },
    settings: {
      react: {
        version: "detect",
      },
    },
    languageOptions: {
      parserOptions: {
        projectService: true,
      },
    },
  },
];
