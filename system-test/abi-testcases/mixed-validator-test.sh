#!/usr/bin/env bash
#
# Basic empirical ABI system test - can validators on all supported versions of
# Nexis talk to each other?
#

set -e
cd "$(dirname "$0")"
NZT_ROOT="$(cd ../..; pwd)"

logDir="$PWD"/logs
ledgerDir="$PWD"/config
rm -rf "$ledgerDir" "$logDir"
mkdir -p "$logDir"

baselineVersion=1.1.18  # <-- oldest version we remain compatible with
otherVersions=(
  beta
  edge
)

nexisInstallDataDir=$PWD/releases
nexisInstallGlobalOpts=(
  --data-dir "$nexisInstallDataDir"
  --config "$nexisInstallDataDir"/config.yml
  --no-modify-path
)

# Install all the nexisversions
bootstrapInstall() {
  declare v=$1
  if [[ ! -h $nexisInstallDataDir/active_release ]]; then
    sh "$NZT_ROOT"/install/exzo-install-init.sh "$v" "${nexisInstallGlobalOpts[@]}"
  fi
  export PATH="$nexisInstallDataDir/active_release/bin/:$PATH"
}

bootstrapInstall "$baselineVersion"
for v in "${otherVersions[@]}"; do
  exzo-install-init "${nexisInstallGlobalOpts[@]}" "$v"
  nexis-V
done


ORIGINAL_PATH=$PATH
nexisInstallUse() {
  declare version=$1
  echo "--- Now using nexis$version"
  NZT_BIN="$nexisInstallDataDir/releases/$version/nexis-release/bin"
  export PATH="$NZT_BIN:$ORIGINAL_PATH"
}

killSession() {
  tmux kill-session -t abi || true
}

export RUST_BACKTRACE=1

# Start up the bootstrap validator using the baseline version
nexisInstallUse "$baselineVersion"
echo "--- Starting $baselineVersion bootstrap validator"
trap 'killSession' INT TERM ERR EXIT
killSession
(
  set -x
  if [[ ! -x baseline-run.sh ]]; then
    curl https://raw.githubusercontent.com/nexis-labs/nexis/v"$baselineVersion"/run.sh -o baseline-run.sh
    chmod +x baseline-run.sh
  fi
  tmux new -s abi -d " \
    ./baseline-run.sh 2>&1 | tee $logDir/$baselineVersion.log \
  "

  SECONDS=
  while [[ ! -f config/baseline-run/init-completed ]]; do
    sleep 5
    if [[ $SECONDS -gt 60 ]]; then
      echo "Error: validator failed to start"
      exit 1
    fi
  done

  nexis--url http://127.0.0.1:8899 show-validators
)

# Ensure all versions can see the bootstrap validator
for v in "${otherVersions[@]}"; do
  nexisInstallUse "$v"
  echo "--- Looking for bootstrap validator on gossip"
  (
    set -x
    "$NZT_BIN"/nexis-gossip spy \
      --entrypoint 127.0.0.1:8001 \
      --num-nodes-exactly 1 \
      --timeout 30
  )
  echo Ok
done

# Start a validator for each version and look for it
#
# Once https://github.com/nexis-labs/nexis/issues/7738 is resolved, remove
# `--no-snapshot-fetch` when starting the validators
#
nodeCount=1
for v in "${otherVersions[@]}"; do
  nodeCount=$((nodeCount + 1))
  nexisInstallUse "$v"
  # start another validator
  ledger="$ledgerDir"/ledger-"$v"
  rm -rf "$ledger"
  echo "--- Looking for $nodeCount validators on gossip"
  (
    set -x
    tmux new-window -t abi -n "$v" " \
      $NZT_BIN/nexis-validator \
      --ledger $ledger \
      --no-snapshot-fetch \
      --entrypoint 127.0.0.1:8001 \
      -o - 2>&1 | tee $logDir/$v.log \
    "
    "$NZT_BIN"/nexis-gossip spy \
      --entrypoint 127.0.0.1:8001 \
      --num-nodes-exactly $nodeCount \
      --timeout 30

    # Wait for it to make a snapshot root
    SECONDS=
    while [[ ! -d $ledger/snapshot ]]; do
      sleep 5
      if [[ $SECONDS -gt 60 ]]; then
        echo "Error: validator failed to create a snapshot"
        exit 1
      fi
    done
  )
  echo Ok
done

# Terminate all the validators
killSession

echo
echo Pass
exit 0
