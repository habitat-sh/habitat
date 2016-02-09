// Expose chai.expect globally for tests
declare var expect: Function;
expect = chai.expect;

// Load all tests
let testContext = (<{ context?: Function }>require).context("./", true, /\.test\.ts/);
testContext.keys().forEach(testContext);
