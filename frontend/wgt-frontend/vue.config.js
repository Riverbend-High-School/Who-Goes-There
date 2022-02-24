module.exports = {
    pages: {
        index: {
            entry: "./src/pages/Home/main.js",
            template: "public/index.html",
            title: "asdasda",
            chunks: ["chunk-vendors", "chunk-common", "index"],
        },
        checkin: {
            entry: "./src/pages/CheckIn/main.js",
            template: "public/index.html",
            title: "Check In",
            chunks: ["chunk-vendors", "chunk-common", "checkin"],
        },
        checkout: {
            entry: "./src/pages/CheckOut/main.js",
            template: "public/index.html",
            title: "Check Out",
            chunks: ["chunk-vendors", "chunk-common", "checkout"],
        },
        visits: {
            entry: "./src/pages/ActiveVisits/main.js",
            template: "public/index.html",
            title: "Active Visits",
            chunks: ["chunk-vendors", "chunk-common", "visits"],
        },
        public: {
            entry: "./src/pages/PublicVisits/main.js",
            template: "public/index.html",
            title: "Public Visits",
            chunks: ["chunk-vendors", "chunk-common", "public"],
        },
        error: {
            entry: "./src/pages/NotFound/main.js",
            template: "public/index.html",
            title: "Error",
            chunks: ["chunk-vendors", "chunk-common", "error"],
        },
    },
};
