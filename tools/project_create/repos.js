//
// Usage: Usage: node repos.js <app id> <installation id> <path to PEM file>
//
// Note: The Habitat Dev App Id is 5629
// Note: The Habitat-Sh Installation Id is 56940
//

const createApp = require('github-app');

if (process.argv.length < 5) {
    console.log("Usage: node repos.js <app id> <installation id> <path to PEM file>");
    process.exit(-1);
}

var app_id = process.argv[2];
var installation_id = process.argv[3];
var pem_path = process.argv[4];

const app = createApp({
  id: app_id,
  cert: require('fs').readFileSync(pem_path)
});

app.asInstallation(installation_id).then(github => {
  github.apps.getInstallationRepositories({})
  .then((repos) => {
    var i;
    for (i in repos.data.repositories) {
      console.log(repos.data.repositories[i].id + ':' + repos.data.repositories[i].name);
    }
  })
  .catch((reason) => {console.log('Error:'+reason);});
});
