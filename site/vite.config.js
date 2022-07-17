// vite.config.js
import { sveltekit } from '@sveltejs/kit/vite';
/** @type {import('vite').UserConfig} */
const config = {
  server: {
    fs: {
      allow: [".."]
    }
  },
  plugins: [sveltekit()], 
};
export default config;