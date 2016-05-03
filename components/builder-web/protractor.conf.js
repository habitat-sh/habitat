
exports.config = {
    baseUrl: process.env.URL || "http://localhost:3000",
    framework: "mocha",
    specs: ["test/e2e**/*.test.ts"],

    chromeOptions: {
        extensions: [],
    },


    mochaOpts: {
        reporter: "spec",
        timeout: 2000,
    },

    /**
     * ng2 related configuration
     *
     * useAllAngular2AppRoots: tells Protractor to wait for any angular2 apps on the page instead of just the one matching
     * `rootEl`
     *
     */
    useAllAngular2AppRoots: true
};
