#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

[[ -d /home/nexis/.ssh ]] || exit 1

if [[ ${#NZT_PUBKEYS[@]} -eq 0 ]]; then
  echo "Warning: sourcenexis-user-authorized_keys.sh first"
fi

#nexis-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL testnets
for key in "${NZT_PUBKEYS[@]}"; do
  echo "$key" >> /nexis-scratch/authorized_keys
done

sudo -u nexisbash -c "
  cat /nexis-scratch/authorized_keys >> /home/nexis/.ssh/authorized_keys
"
