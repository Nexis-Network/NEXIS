#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

if grep -q nexis/etc/passwd ; then
  echo "User nexisalready exists"
else
  adduser nexis--gecos "" --disabled-password --quiet
  adduser nexissudo
  adduser nexisadm
  echo "nexisALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
  id nexis

  [[ -r /nexis-scratch/id_ecdsa ]] || exit 1
  [[ -r /nexis-scratch/id_ecdsa.pub ]] || exit 1

  sudo -u nexisbash -c "
    echo 'PATH=\"/home/nexis/.cargo/bin:$PATH\"' > /home/nexis/.profile
    mkdir -p /home/nexis/.ssh/
    cd /home/nexis/.ssh/
    cp /nexis-scratch/id_ecdsa.pub authorized_keys
    umask 377
    cp /nexis-scratch/id_ecdsa id_ecdsa
    echo \"
      Host *
      BatchMode yes
      IdentityFile ~/.ssh/id_ecdsa
      StrictHostKeyChecking no
    \" > config
  "
fi
