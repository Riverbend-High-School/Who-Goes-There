import { createApp } from "vue";
import axios from "axios";
import VueAxios from "vue-axios";
import * as Sentry from "@sentry/vue";
import { BrowserTracing } from "@sentry/tracing";
import Configuration from "@/assets/configuration";

import AccessDenied from "./AccessDenied.vue";

Sentry.init({
    app,
    dsn: Configuration.value("sentryDSN"),
    integrations: [
        new BrowserTracing({
            tracingOrigins: ["localhost", "wgt.sitt.gabrielhogan.com", /^\//],
        }),
    ],
    // Set tracesSampleRate to 1.0 to capture 100%
    // of transactions for performance monitoring.
    // We recommend adjusting this value in production
    tracesSampleRate: 1.0,
});

const app = createApp(AccessDenied);
app.use(VueAxios, axios);
app.mount("#app");

app.mixin(Sentry.createTracingMixins({ trackComponents: true }));
Sentry.attachErrorHandler(app, { logErrors: true });
