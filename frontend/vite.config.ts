import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait()
    ],
    server: {
        fs: {
            // rust build 成果物 (../backend/pkg) へのアクセスを許可する
            allow: [".."]
        }
    }
});
