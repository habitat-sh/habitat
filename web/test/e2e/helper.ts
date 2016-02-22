import * as chai from "chai";
import * as chaiAsPromised from "chai-as-promised";
chai.use(chaiAsPromised);

export const expect = chai.expect;

browser.manage().window().setSize(1024, 768);