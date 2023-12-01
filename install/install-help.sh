#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"/..
cargo="$(readlink -f "./cargo")"

"$cargo" build --packagenexis-install
export PATH=$PWD/target/debug:$PATH

echo "\`\`\`manpage"
nexis-install --help
echo "\`\`\`"
echo ""

commands=(init info deploy update run)

for x in "${commands[@]}"; do
    echo "\`\`\`manpage"
   nexis-install "${x}" --help
    echo "\`\`\`"
    echo ""
done
