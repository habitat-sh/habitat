#/bin/bash

if [ ! -f "$HOME/.nvm/nvm.sh" ]; then
    curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.1/install.sh | bash
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
fi

cd components/builder-web

if [ ! -f habitat.conf.js ]; then
    cp habitat.conf.sample.js habitat.conf.js
fi

if [ ! nvm use ]; then
    nvm install $(cat .nvmrc)
fi

npm install
npm start
