import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import path from "path";
import Components from "unplugin-vue-components/vite";
import AutoImport from "unplugin-auto-import/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    tailwindcss(),
    
    // 自动导入组件
    Components({
      dirs: ["src/components"],
      extensions: ["vue"],
      deep: true,
      dts: "src/components.d.ts",
      directoryAsNamespace: false,
      resolvers: [
        // 自定义解析器，支持 UI 组件
        (componentName) => {
          // 自动导入 src/components/ui 下的组件
          if (componentName.match(/^(Button|Card|CardHeader|CardTitle|CardContent|Input|Label|Select|Textarea|Dialog|AlertDialog|Toast|Dropdown|Popover|Tabs|Accordion|Avatar|Badge|Checkbox|Radio|Switch|Slider|Progress|Separator|Sheet|Skeleton|Table|Tooltip)$/)) {
            return {
              name: componentName,
              from: `@/components/ui/${componentName.toLowerCase().replace(/([A-Z])/g, '-$1').slice(1)}`,
            };
          }
        },
      ],
    }),
    
    // 自动导入 API
    AutoImport({
      imports: [
        "vue",
        "@vueuse/core",
      ],
      dts: "src/auto-imports.d.ts",
      dirs: [
        "src/composables",
        "src/stores",
      ],
      vueTemplate: true,
    }),
  ],

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
