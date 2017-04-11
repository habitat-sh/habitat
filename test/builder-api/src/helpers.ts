import supertest = require("supertest");
import chai = require("chai");
import "mocha";

const globalAny: any = global;

// Users we can authenticate as
globalAny.bobo_bearer = "Bearer bobo";
globalAny.logan_bearer = "Bearer logan";
