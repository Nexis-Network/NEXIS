#!/usr/bin/env bash

set -x
! tmux list-sessions || tmux kill-session
declare sudo=
if sudo true; then
  sudo="sudo -n"
fi

echo "pwd: $(pwd)"
for pid in nexis/*.pid; do
  pgid=$(ps opgid= "$(cat "$pid")" | tr -d '[:space:]')
  if [[ -n $pgid ]]; then
    $sudo kill -- -"$pgid"
  fi
done
if [[ -f nexis/netem.cfg ]]; then
  nexis/scripts/netem.sh delete < nexis/netem.cfg
  rm -f nexis/netem.cfg
fi
nexis/scripts/net-shaper.sh cleanup
for pattern in validator.sh boostrap-leader.shnexis- remote- iftop validator client node; do
  echo "killing $pattern"
  pkill -f $pattern
done
