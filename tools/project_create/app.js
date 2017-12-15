//
// Usage: Usage: node app.js <app id> <path to PEM file>
//
// Note: The Habitat Dev App Id is 5629
//

const createApp = require('github-app');

if (process.argv.length < 4) {
    console.log("Usage: node app.js <app id> <path to PEM file>");
    process.exit(-1);
}

var app_id = process.argv[2];
var pem_path = process.argv[3];

const app = createApp({
  id: app_id,
  cert: require('fs').readFileSync(pem_path)
});


app.asApp().then(github => {
  github.apps.get({}).then(result => {
    console.log(result.data.id + ':' + result.data.name + ' (' + result.data.owner.login + ')');
  });
});
