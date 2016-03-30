// Expose chai.expect globally for tests
declare var expect: Function;
expect = chai.expect;

// Ensure config global exists
window["Habitat"] = { config: {} };

// Load all tests
let testContext = (<{ context?: Function }>require).context("./", true, /\.test\.ts/);
testContext.keys().forEach(testContext);
