#!/usr/bin/env bash

set -e
cd "$(dirname "$0")"
NZT_ROOT="$(cd ../..; pwd)"

logDir="$PWD"/logs
rm -rf "$logDir"
mkdir "$logDir"

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

bootstrapInstall "edge"
exzo-install-init --version
exzo-install-init edge
nexis-gossip --version
nexis-dos --version

killallnexis-gossip || true
nexis-gossip spy --gossip-port 8001 > "$logDir"/gossip.log 2>&1 &
nexisGossipPid=$!
echo "nexis-gossip pid: $nexisGossipPid"
sleep 5
nexis-dos --mode gossip --data-type random --data-size 1232 &
dosPid=$!
echo "nexis-dos pid: $dosPid"

pass=true

SECONDS=
while ((SECONDS < 600)); do
  if ! kill -0 $nexisGossipPid; then
    echo "nexis-gossip is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  if ! kill -0 $dosPid; then
    echo "nexis-dos is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  sleep 1
done

kill $nexisGossipPid || true
kill $dosPid || true
wait || true

$pass && echo Pass
