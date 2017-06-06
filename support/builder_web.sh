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

# When the Habitat source tree is shared with a guest OS, the builder-web/node_modules
# directories get shared as well -- including their bundled/compiled native binaries,
# which is rarely what you want.  This keeps the guest-built stuff on the guest
# and the host-built stuff on the host.
mkdir -p "$HOME/.builder_web_node_modules" ./node_modules
sudo mount --bind "$HOME/.builder_web_node_modules" ./node_modules

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
