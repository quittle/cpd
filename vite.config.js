import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import process from "process";
import path from "path";

const outDir = process.env.OUT_DIR;
if (!outDir) {
    throw new Error(
        "OUT_DIR environment variable is not set. Likely run as part of a non-Cargo driven build."
    );
}

const isDev = process.env.NODE_ENV === "development";
const isProd = !isDev;

export default defineConfig({
    plugins: [
        react({
            minify: isProd,
        }),
    ],
    build: {
        minify: isProd ? "esbuild" : false,
        sourcemap: true,
        emptyOutDir: true,
        outDir: path.join(outDir, "static"),
    },
});
