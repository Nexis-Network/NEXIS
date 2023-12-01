#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$NZT_CONFIG_DIR"/latest-testnet-snapshot
mkdir -p "$NZT_CONFIG_DIR"/latest-testnet-snapshot
(
  cd "$NZT_CONFIG_DIR"/latest-testnet-snapshot || exit 1
  set -x
  wget http://api.testnet.velas.com/genesis.tar.bz2
  wget --trust-server-names http://testnet.velas.com/snapshot.tar.bz2
)

snapshot=$(ls "$NZT_CONFIG_DIR"/latest-testnet-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$NZT_CONFIG_DIR"/bootstrap-validator
mkdir -p "$NZT_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$NZT_CONFIG_DIR"/faucet.json
else
  $exzo_keygen new --no-passphrase -fso "$NZT_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$NZT_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $exzo_keygen new --no-passphrase -so "$NZT_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$exzo_keygen new --no-passphrase -so "$NZT_CONFIG_DIR"/bootstrap-validator/vote-account.json
$exzo_keygen new --no-passphrase -so "$NZT_CONFIG_DIR"/bootstrap-validator/stake-account.json

$exzo_ledger_tool create-snapshot \
  --ledger "$NZT_CONFIG_DIR"/latest-testnet-snapshot \
  --faucet-pubkey "$NZT_CONFIG_DIR"/faucet.json \
  --faucet-lamports 500000000000000000 \
  --bootstrap-validator "$NZT_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$NZT_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$NZT_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$NZT_CONFIG_DIR"/bootstrap-validator

$exzo_ledger_tool modify-genesis \
  --ledger "$NZT_CONFIG_DIR"/latest-testnet-snapshot \
  --hashes-per-tick sleep \
  "$NZT_CONFIG_DIR"/bootstrap-validator