#!/bin/bash

set -e

if [ -z ${HAB_AUTH_TOKEN+x} ]; then
  echo "HAB_AUTH_TOKEN var is unset"
  exit 1
fi
if [ -z ${BINTRAY_USER+x} ]; then
  echo "BINTRAY_USER var is unset"
  exit 1
fi
if [ -z ${BINTRAY_KEY+x} ]; then
  echo "BINTRAY_KEY var is unset"
  exit 1
fi

HAB_VERSION=$(cat VERSION)
PROMOTE_CHANNEL=stable
RC_CHANNEL=rc-$HAB_VERSION
RC_PKGS_JSON=$(curl https://bldr.habitat.sh/v1/depot/channels/core/$RC_CHANNEL/pkgs)
RC_PKGS_JSON_DATA=$(echo $RC_PKGS_JSON | jq -c -r '.data | .[]')
TOTAL_COUNT=$(echo $RC_PKGS_JSON | jq -r '.total_count')
END=$(echo $RC_PKGS_JSON | jq -r '.range_end')

# We limit results to 50 items. While it is unlikely, it is possible
# after building several releases that we may exceed 50 packages in
# the rc channel. We could get all fancy and complicated here to
# aggregate multiple pages but I'd rather tear my eyes out of their
# sockets. Instead, we suggest a workaround in the failure message.
if [ $(($END+1)) !=  $TOTAL_COUNT ]; then
  echo "There are multipe pages of releases. Consider deleting the channel and then rebuild."
  exit 1
fi

# lets build a list of releases keyed by name-platform to track
# the most recent release for an individual package/target
declare -A pkg_releases
for pkg in $RC_PKGS_JSON_DATA
do
    ORIGIN=$(echo $pkg | jq -r ".origin")
    NAME=$(echo $pkg | jq -r ".name")
    VERSION=$(echo $pkg | jq -r ".version")
    RELEASE=$(echo $pkg | jq -r ".release")
    TARGET=$(echo $pkg | jq -r ".platforms[0]")
    IDENT=$ORIGIN/$NAME/$VERSION/$RELEASE
    if [[ -z ${pkg_releases[$NAME-$TARGET]+x} || "${pkg_releases[$NAME-$TARGET]}" < "$IDENT" ]]; then
        pkg_releases[$NAME-$TARGET]=$IDENT

        if [ "$NAME" == "hab" ]; then
            case $TARGET in
                *-linux)
                LINUX_RELEASE=$RELEASE
                ;;
                *-windows)
                WINDOWS_RELEASE=$RELEASE
                ;;
            esac
        fi
    fi
done

for rel_target in ${!pkg_releases[@]}
do
    rel=${pkg_releases[$rel_target]}
    echo "Promoting $rel_target-$rel to $PROMOTE_CHANNEL"
    hab pkg promote $rel $PROMOTE_CHANNEL
done

if [ -z ${LINUX_RELEASE+x} ]; then
  echo "Could not find hab package for linux platform in $RC_CHANNEL channel"
  exit 1
fi
if [ -z ${WINDOWS_RELEASE+x} ]; then
  echo "Could not find hab package for windows platform in $RC_CHANNEL channel"
  exit 1
fi

echo "Publishing Linux CLI $HAB_VERSION-$LINUX_RELEASE to bintray"
curl -u $BINTRAY_USER:$BINTRAY_KEY -X POST https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-linux/$HAB_VERSION-$LINUX_RELEASE/publish
echo "Publishing Windows CLI $HAB_VERSION-$WINDOWS_RELEASE to bintray"
curl -u $BINTRAY_USER:$BINTRAY_KEY -X POST https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-windows/$HAB_VERSION-$WINDOWS_RELEASE/publish

# We do not store darwin packages on builder so we will get
# the latest version on bintray
DARWIN_RELEASE=$(curl -u $BINTRAY_USER:$BINTRAY_KEY https://api.bintray.com/packages/habitat/stable/hab-x86_64-darwin | jq -r '.versions[0]')
IFS='-' read -r -a version_release <<< $DARWIN_RELEASE
if [ "${version_release[0]}" != "$HAB_VERSION" ]; then
  echo "Could not find a darwin release for $HAB_VERSION on bintray"
  exit 1
fi
echo "Publishing Mac CLI $DARWIN_RELEASE to bintray"
curl -u $BINTRAY_USER:$BINTRAY_KEY -X POST https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-darwin/$DARWIN_RELEASE/publish
