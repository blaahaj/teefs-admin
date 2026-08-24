import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  base: "/admin",
  plugins: [solid()],

  server: {
    proxy: {
      "/admin/api": {
        target: "http://localhost:8300",
      },
    },
  },
});
