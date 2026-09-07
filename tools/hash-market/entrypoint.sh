#!/bin/sh
# hash-market-server entrypoint
#
# Modes:
#   1. Direct: config already mounted at HM_CONFIG → start immediately
#   2. Akash/o-line: SSH_PUBKEY set → start sshd, wait for SFTP-delivered config
#
# NEVER pass signing_key / mnemonics as env vars. Secrets arrive only via
# config.toml delivered over SFTP after lease (see o-line hashmerchant upload).

set -e

CONFIG="${HM_CONFIG:-/etc/hash-market/config.toml}"
DATA="${HM_DATA:-/var/lib/hash-market}"
SSH_PORT="${SSH_P:-22}"

mkdir -p "$(dirname "$CONFIG")" "$DATA" /var/run/sshd /etc/ssh

# ── Optional SSH bootstrap (post-deploy secret delivery) ─────────────────────
if [ -n "${SSH_PUBKEY:-}" ]; then
  echo "=== hash-market: SSH secret-delivery mode ==="

  if ! command -v sshd >/dev/null 2>&1; then
    echo "ERROR: openssh-server not installed in image (needed for SSH_PUBKEY mode)"
    exit 1
  fi

  # Host keys (persist under data volume when possible)
  if [ ! -f /etc/ssh/ssh_host_ed25519_key ]; then
    if [ -f "$DATA/ssh_host_ed25519_key" ]; then
      cp "$DATA/ssh_host_ed25519_key" /etc/ssh/ssh_host_ed25519_key
      cp "$DATA/ssh_host_ed25519_key.pub" /etc/ssh/ssh_host_ed25519_key.pub 2>/dev/null || true
    else
      ssh-keygen -t ed25519 -f /etc/ssh/ssh_host_ed25519_key -N ""
      cp /etc/ssh/ssh_host_ed25519_key "$DATA/ssh_host_ed25519_key" 2>/dev/null || true
      cp /etc/ssh/ssh_host_ed25519_key.pub "$DATA/ssh_host_ed25519_key.pub" 2>/dev/null || true
    fi
  fi

  mkdir -p /root/.ssh
  printf '%s\n' "$SSH_PUBKEY" > /root/.ssh/authorized_keys
  chmod 700 /root/.ssh
  chmod 600 /root/.ssh/authorized_keys

  cat > /etc/ssh/sshd_config <<EOF
Port ${SSH_PORT}
ListenAddress 0.0.0.0
Protocol 2
HostKey /etc/ssh/ssh_host_ed25519_key
PermitRootLogin prohibit-password
PasswordAuthentication no
PubkeyAuthentication yes
ChallengeResponseAuthentication no
UsePAM no
X11Forwarding no
AllowTcpForwarding no
Subsystem sftp internal-sftp
EOF

  /usr/sbin/sshd
  echo "sshd listening on :${SSH_PORT} — waiting for config at ${CONFIG}"
  echo "Deliver with: oline hashmerchant upload --config ./config.toml ..."

  # Wait forever until operator pushes config.toml via SFTP
  i=0
  while [ ! -s "$CONFIG" ]; do
    i=$((i + 1))
    if [ $((i % 15)) -eq 0 ]; then
      echo "still waiting for ${CONFIG} (${i}s)..."
    fi
    sleep 2
  done
  echo "=== config present — starting hash-market-server ==="
fi

if [ ! -s "$CONFIG" ]; then
  echo "ERROR: config not found at ${CONFIG}"
  echo "Mount a config.toml or set SSH_PUBKEY and SFTP the file post-deploy."
  exit 1
fi

# Ensure data dir writable
mkdir -p "$DATA"
chown -R hashmarket:hashmarket "$DATA" 2>/dev/null || true

# Prefer dropping privileges when possible
if command -v su-exec >/dev/null 2>&1; then
  exec su-exec hashmarket hash-market-server --config "$CONFIG"
elif command -v setpriv >/dev/null 2>&1; then
  exec setpriv --reuid hashmarket --regid hashmarket --clear-groups \
    hash-market-server --config "$CONFIG"
else
  exec hash-market-server --config "$CONFIG"
fi
