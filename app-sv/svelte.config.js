// import adapter from '@sveltejs/adapter-auto';
import adapter from "svelte-adapter-bun-next";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    csrf: {
      trustedOrigins: ["http://localhost:5170", "http://localhost:5170/*"],
    },
    // adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
    // If your environment is not supported, or you settled on a specific environment, switch out the adapter.
    // See https://svelte.dev/docs/kit/adapters for more information about adapters.
    adapter: adapter(),
    alias: {
      $shadui: "./src/lib/shadcn/components/ui",
      $shadcn: "./src/lib/shadcn",
    },
  },
};

export default config;
