import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import eslintReact from "@eslint-react/eslint-plugin";
import reactHooks from "eslint-plugin-react-hooks";
import pluginPrettier from "eslint-config-prettier/flat";

export default [
  {
    files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"],
    languageOptions: {
      globals: globals.browser,
      parserOptions: {
        projectService: true,
        ecmaFeatures: { jsx: true },
      },
    },
  },
  pluginJs.configs.all,
  ...tseslint.configs.recommendedTypeChecked,
  eslintReact.configs["all"],
  reactHooks.configs.flat.recommended,
  pluginPrettier,
  {
    rules: {
      "no-magic-numbers": "off",
      "no-plusplus": "off",
      "no-void": [
        "error",
        {
          allowAsStatement: true,
        },
      ],
      "id-length": "off",
      "no-ternary": "off",
      "func-style": "off",
      "sort-keys": "off",
      "no-undefined": "off",
      "max-lines-per-function": "off",
      "default-case": "off",
      "max-statements": "off",
      "no-duplicate-imports": [
        "error",
        {
          allowSeparateTypeImports: true,
          includeExports: true,
        },
      ],
      "capitalized-comments": [
        "error",
        "always",
        {
          ignoreConsecutiveComments: true,
        },
      ],
      "one-var": "off",
      "init-declarations": "off",
      "no-console": "off",
      "no-nested-ternary": "off",
    },
  },
  {
    rules: {
      "@typescript-eslint/no-misused-promises": [
        "error",
        {
          checksVoidReturn: { attributes: false },
        },
      ],
      "@typescript-eslint/restrict-template-expressions": [
        "error",
        {
          allowArray: true,
        },
      ],
      "@typescript-eslint/switch-exhaustiveness-check": [
        "error",
        {
          allowDefaultCaseForExhaustiveSwitch: false,
          requireDefaultForNonUnion: true,
        },
      ],
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
    },
  },
  {
    settings: {
      "react-x": {
        version: "detect",
      },
    },
  },
];
