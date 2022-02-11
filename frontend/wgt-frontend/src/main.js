import * as Vue from "vue"; // in Vue 3
import axios from "axios";
import VueAxios from "vue-axios";
import * as VueRouter from "vue-router";
import * as Sentry from "@sentry/vue";
import { BrowserTracing } from "@sentry/tracing";
import Configuration from '@/assets/configuration'

import App from "./App.vue";

import checkTemplate from "./components/checkTemplate.vue";

const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes: [
        {
            path: "/",
            name: "home",
            component: () => import("./pages/Home.vue"),
        },
        {
            path: "/checkin",
            name: "checkin",
            component: checkTemplate,
            props: { title: "Library Sign In", endpoint: "checkin" },
        },
        {
            path: "/checkout",
            name: "checkout",
            component: checkTemplate,
            props: { title: "Library Sign Out", endpoint: "checkout" },
        },
        {
            path: "/visits",
            name: "visits",
            component: () => import("./pages/ActiveVisits.vue"),
            props: { endpoint: "visits" },
        },
        {
            path: "/public",
            name: "public",
            component: () => import("./pages/PublicVisits.vue"),
            props: { endpoint: "visits/public" },
        },
        {
            path: "/test",
            name: "test",
            component: () => import("./components/errorTemplate.vue"),
            props: (route) => ({
                title: "Not Found",
                message: `The requested URL <span>${route.path}</span> was not found on this server. `,
            }),
        },
        {
            path: "/:pathMatch(.*)*",
            name: "not-found",
            component: () => import("./components/errorTemplate.vue"),
            props: (route) => ({
                title: "Not Found",
                message: `The requested URL <span>${route.path}</span> was not found on this server. `,
            }),
        },
    ],
});

export default router;

Sentry.init({
    app,
    dsn: Configuration.value("sentryDSN"),
    integrations: [
        new BrowserTracing({
            routingInstrumentation: Sentry.vueRouterInstrumentation(router),
            tracingOrigins: ["localhost", "my-site-url.com", /^\//],
        }),
    ],
    // Set tracesSampleRate to 1.0 to capture 100%
    // of transactions for performance monitoring.
    // We recommend adjusting this value in production
    tracesSampleRate: 1.0,
});

const app = Vue.createApp(App);
app.use(router);
app.use(VueAxios, axios);
app.mount("#app");

app.mixin(Sentry.createTracingMixins({ trackComponents: true }));
Sentry.attachErrorHandler(app, { logErrors: true });
