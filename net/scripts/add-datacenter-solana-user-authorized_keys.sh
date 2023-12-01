#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

# shellcheck source=net/scripts/nexis-user-authorized_keys.sh
sourcenexis-user-authorized_keys.sh

#nexis-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL datacenter nodes.
for i in "${!NZT_USERS[@]}"; do
  echo "environment=\"NZT_USER=${NZT_USERS[i]}\" ${NZT_PUBKEYS[i]}"
done

