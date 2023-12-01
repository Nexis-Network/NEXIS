#!/usr/bin/env bash
#
# Starts an instance ofnexis-faucet
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$NZT_CONFIG_DIR"/faucet.json ]] || {
    echo "$NZT_CONFIG_DIR/faucet.json not found, create it by running:"
    echo
    echo "  ${here}/setup.sh"
    exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $nexis_faucet
exec $exzo_faucet --keypair "$NZT_CONFIG_DIR"/faucet.json "$@"
