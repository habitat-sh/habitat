#!/bin/bash

unset NODE_ENV

if [ ! -f "$HOME/.nvm/nvm.sh" ]; then
    curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.1/install.sh | bash
fi

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

cd components/builder-web

if [ ! -f habitat.conf.js ]; then
    cp habitat.conf.sample.js habitat.conf.js
fi

nvm install
npm install
npm run build

echo '{
  "port": 3000,
  "open": false,
  "files": false,
  "server": {
    "baseDir": "./"
  }
}' > /tmp/bs-config.json

./node_modules/.bin/lite-server -c /tmp/bs-config.json
