import { createApp } from "vue";
import App from "./App.vue";

import { router } from "./vue_router.ts";

createApp(App).use(router).mount("#app");
