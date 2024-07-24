import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import wasm from "vite-plugin-wasm";
import cssJsonVar from "vite-plugin-css-json-var"

export default defineConfig({
  plugins: [solid(), wasm(), cssJsonVar({
    file: './assets/style_vars.json',
    lang: 'css',
    style: 'css'
  })],
  resolve: {
    alias: {
      src: "/src",
      assets: "/assets",
    },
  },
  build: {
    // Below is to support top-level-await required for our wasm stuff
    target: 'es2022',
    rollupOptions: {
      input: {
        main: 'index.html',
      },
      output: {
        preserveModules: false
      }
    }
  },
  optimizeDeps: {
    // This is from solid-codemirror package docs:
    // https://github.com/riccardoperra/solid-codemirror/blob/afb0a56bf4bff0c9b1544a57b4dd9b9170cb5d39/README.md#codemirror-packages-error-fix
    include: ['@codemirror/state', '@codemirror/view'],
  }

})
