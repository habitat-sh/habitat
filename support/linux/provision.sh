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

pushd /src
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_latest.sh
sh support/linux/install_dev_9_linux.sh
echo 'eval "$(direnv hook bash)"' >> /root/.bashrc
echo "source /src/support/linux/helpers.sh" >> /root/.bashrc
. ~/.profile
. ~/.bashrc
. ./.envrc

hab pkg install core/postgresql core/shadow -c stable
hab pkg install core/builder-datastore -c unstable
hab pkg install core/docker
hab pkg install core/hab-pkg-export-docker

hab pkg exec core/shadow groupadd --force krangschnak
hab pkg exec core/shadow useradd --groups=tty --create-home -g krangschnak krangschnak || echo "User 'krangschnak' already exists"

mkdir -p /hab/svc/builder-jobsrv/data
mkdir -p /hab/svc/builder-datastore
cp -f support/builder/datastore.toml /hab/svc/builder-datastore/user.toml
support/builder/init-datastore.sh
support/builder/config.sh
support/linux/setup_keys.sh
popd
