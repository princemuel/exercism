// @ts-check

import config from "@exercism/eslint-config-typescript";
import maintainersConfig from "@exercism/eslint-config-typescript/maintainers.mjs";
import tsEslint from "typescript-eslint";

export default [
    ...tsEslint.config(...config, {
        files: [".meta/proof.ci.ts", ".meta/exemplar.ts", "*.test.ts"],
        extends: maintainersConfig,
    }),
    {
        ignores: [
            // # Protected or generated
            ".git/**/*",
            ".vscode/**/*",

            //# When using npm
            "node_modules/**/*",

            // # Configuration files
            "babel.config.cjs",
            "jest.config.cjs",
        ],
    },
];
