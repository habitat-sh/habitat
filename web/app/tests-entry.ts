let testContext = (<{ context?: Function }>require).context("./", true, /\.test\.ts/);
testContext.keys().forEach(testContext);
