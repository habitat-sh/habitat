//
// Usage: Usage: node installations.js <app id> <path to PEM file>
//
// Note: The Habitat Dev App Id is 5629
//

const createApp = require('github-app');

if (process.argv.length < 4) {
    console.log("Usage: node installations.js <app id> <path to PEM file>");
    process.exit(-1);
}

var app_id = process.argv[2];
var pem_path = process.argv[3];

const app = createApp({
  id: app_id,
  cert: require('fs').readFileSync(pem_path)
});

app.asApp().then(github => {
  github.apps.getInstallations({}).then(installations => {
    var i;
    for (i in installations.data) {
      console.log(installations.data[i].id + ':' + installations.data[i].account.login);
    }
  });
});
