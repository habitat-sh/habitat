#!/bin/bash

export APP_HOSTNAME=localhost:3000
export GITHUB_API_URL=https://api.github.com
export GITHUB_WEB_URL=https://github.com
export GITHUB_CLIENT_ID=Iv1.732260b62f84db15
export GITHUB_CLIENT_SECRET=fc7654ed8c65ccfe014cd339a55e3538f935027a
export WORKER_AUTH_TOKEN=fc7654ed8c65ccfe014cd339a55e3538f935027a
export GITHUB_ADMIN_TEAM=1995301
export GITHUB_WORKER_TEAM=2555389
export GITHUB_WEBHOOK_SECRET=58d4afaf5e5617ab0f8c39e505605e78a054d003
export GITHUB_APP_ID=5629
pushd /src
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_latest.sh
sh support/linux/install_dev_9_linux.sh
# shellcheck disable=SC2016
echo 'eval "$(direnv hook bash)"' | sudo tee --append /root/.bashrc > /dev/null
echo "source /src/support/linux/helpers.sh" | sudo tee --append /root/.bashrc > /dev/null
. ~/.profile
. ~/.bashrc
. ./.envrc

sudo hab pkg install core/postgresql core/shadow -c stable
sudo hab pkg install core/hab-sup core/hab-launcher
sudo hab pkg install core/builder-datastore
sudo hab pkg install core/docker
sudo hab pkg install core/hab-pkg-export-docker

sudo hab pkg exec core/shadow groupadd --force krangschnak
if ! id -u krangschnak > /dev/null 2>&1; then
	sudo hab pkg exec core/shadow useradd --groups=tty --create-home -g krangschnak krangschnak
fi

sudo mkdir -p /hab/svc/builder-jobsrv/data
sudo mkdir -p /hab/svc/builder-datastore
sudo cp -f support/builder/datastore.toml /hab/svc/builder-datastore/user.toml
support/builder/init-datastore.sh
sudo -E support/builder/dev-config.sh
support/linux/setup_keys.sh
# Make sure setup_keys is the last thing so that if it has errors the user
# can just rerun it without needing to redo the rest of this script
popd
