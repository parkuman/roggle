// vite.config.js
import { sveltekit } from '@sveltejs/kit/vite';
/** @type {import('vite').UserConfig} */
const config = {
  server: {
    port: "3000",
    fs: {
      // allow import from /static/ folder
      allow: [".."]
    }
  },
  plugins: [sveltekit()], 
};
export default config;