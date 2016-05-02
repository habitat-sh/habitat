// Take the CSS_SHA and JS_SHA environment variables and replace the index.html
// references to the scripts with them.

"use strict";

const jsdom = require("jsdom")
const path = require("path")
const readFileSync = require("fs").readFileSync;
let doc = jsdom.jsdom(readFileSync(path.join(__dirname, "..", "index.html")));

let linkTag = doc.getElementById("hab-css");
linkTag.href = `/assets/app-${process.env.CSS_SHA}.css`;

let scriptTag = doc.getElementById("hab-js");
scriptTag.src = `/assets/app-${process.env.JS_SHA}.js`;

console.log(jsdom.serializeDocument(doc));
