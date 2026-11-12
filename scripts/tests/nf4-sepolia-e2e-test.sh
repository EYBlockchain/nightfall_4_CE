#!/usr/bin/env bash
# Validates scripts/nf4-sepolia-e2e without SSH, Sepolia, or a private key on the wire.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ORCH="${ROOT}/scripts/nf4-sepolia-e2e"
CLIENT_KEY="0x1111111111111111111111111111111111111111111111111111111111111111"
DEPLOYER_KEY="0x2222222222222222222222222222222222222222222222222222222222222222"
ANVIL_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
C2="0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004"
C1="0x2021c2Ca1791E5b50D3e882f979b03610db3B8A3"
ZKP="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
PASS=0

fail() {
  echo "FAIL $*" >&2
  exit 1
}

need() {
  local file="$1" pattern="$2"
  grep -q -- "$pattern" "$file" || fail "$3"
}

need_not() {
  local file="$1" pattern="$2"
  if grep -q -- "$pattern" "$file" 2>/dev/null; then
    fail "$3"
  fi
}

write_fakes() {
  local bin="$1"
  mkdir -p "$bin"
  cat > "$bin/ssh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
mkdir -p "${NF4_SSH_LOG}"
{
  printf 'ARGV'
  printf ' %q' "$@"
  printf '\n'
} >> "${NF4_SSH_LOG}/argv"
stdin_file="$(mktemp)"
if [[ -t 0 ]]; then
  : > "$stdin_file"
else
  cat > "$stdin_file"
fi
cat "$stdin_file" >> "${NF4_SSH_LOG}/stdin"
args=("$@")
i=0
while [[ "$i" -lt ${#args[@]} ]]; do
  case "${args[$i]}" in
    -N)
      touch "${NF4_SANDBOX}/forwards-up"
      rm -f "$stdin_file"
      exit 0
      ;;
    -i|-p|-o|-L)
      i=$((i + 2))
      ;;
    -*)
      i=$((i + 1))
      ;;
    *@*)
      i=$((i + 1))
      break
      ;;
    *)
      break
      ;;
  esac
done
rest=("${args[@]:$i}")
if [[ ${#rest[@]} -eq 0 ]]; then
  rm -f "$stdin_file"
  exit 0
fi
if [[ "${rest[0]}" == bash && "${rest[1]:-}" == "-s" ]]; then
  bash -s -- "${rest[@]:3}" < "$stdin_file"
  rc=$?
  rm -f "$stdin_file"
  exit "$rc"
fi
bash --noprofile --norc -c "${rest[*]}" < "$stdin_file"
rc=$?
rm -f "$stdin_file"
exit "$rc"
EOF
  cat > "$bin/tmux" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cwd="$PWD"
args=("$@")
i=0
while [[ "$i" -lt ${#args[@]} ]]; do
  if [[ "${args[$i]}" == "-c" ]]; then
    cwd="${args[$((i + 1))]}"
  fi
  i=$((i + 1))
done
if [[ "${1:-}" == has-session || "${1:-}" == list-windows ]]; then
  exit 1
fi
if [[ "${1:-}" == new-session ]]; then
  exit 0
fi
if [[ "${1:-}" == new-window ]]; then
  i=0
  while [[ "$i" -lt ${#args[@]} ]]; do
    if [[ "${args[$i]}" == "-lc" ]]; then
      cd "$cwd"
      bash --noprofile --norc -c "${args[$((i + 1))]}"
      exit $?
    fi
    i=$((i + 1))
  done
fi
exit 0
EOF
  cat > "$bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  info) exit 0 ;;
  ps)
    if [[ -f "$PWD/.client-up" ]]; then
      echo nf4_indie_client
    fi
    exit 0
    ;;
  logs)
    if [[ -f "$PWD/.docker-log" ]]; then
      cat "$PWD/.docker-log"
    fi
    exit 0
    ;;
  inspect) exit 1 ;;
  *) exit 0 ;;
esac
EOF
  cat > "$bin/cast" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == chain-id ]]; then
  echo 11155111
  exit 0
fi
if [[ "${1:-}" == call ]]; then
  addr=""
  for arg in "$@"; do
    if [[ "$arg" =~ ^0x[0-9a-fA-F]{40}$ ]]; then
      addr="$arg"
    fi
  done
  file="${NF4_SANDBOX}/l1-${addr}"
  if [[ -f "$file" ]]; then
    cat "$file"
  else
    echo 1
  fi
  exit 0
fi
echo 11155111
EOF
  cat > "$bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *--short\ HEAD* ]]; then
  echo abc1234
  exit 0
fi
if [[ "$*" == *abbrev-ref* ]]; then
  echo sepolia-lab
  exit 0
fi
exit 0
EOF
  cat > "$bin/az" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == account && "${2:-}" == show ]]; then
  [[ -f "${NF4_SANDBOX}/az-logged-in" ]]
  exit $?
fi
touch "${NF4_SANDBOX}/bastion-up"
exit 0
EOF
  cat > "$bin/lsof" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
port=""
for arg in "$@"; do
  case "$arg" in
    -iTCP:*) port="${arg#-iTCP:}" ;;
  esac
done
case "$port" in
  50023) [[ -f "${NF4_SANDBOX}/bastion-up" ]] ;;
  18080|13001) [[ -f "${NF4_SANDBOX}/forwards-up" ]] ;;
  *) exit 1 ;;
esac
EOF
  cat > "$bin/curl" <<'EOF'
#!/usr/bin/env python3
import os, sys
from pathlib import Path
sandbox = Path(os.environ["NF4_SANDBOX"])
args = sys.argv[1:]
url = ""
want_code = False
i = 0
while i < len(args):
    arg = args[i]
    if arg in {"-fsS", "-f", "-s", "-S", "-fsS"}:
        i += 1
        continue
    if arg in {"--max-time", "--connect-timeout", "--retry", "--retry-delay", "-o", "-w"}:
        if arg == "-w":
            want_code = True
        i += 2
        continue
    if arg.startswith("http"):
        url = arg
    i += 1

def emit(body, code="200"):
    if want_code:
        sys.stdout.write(code)
        return
    sys.stdout.write(body)

if url.endswith("/v1/health"):
    emit("Healthy\n")
elif url.endswith("/v1/blockdata"):
    emit(sandbox.joinpath("l2").read_text().strip() + "\n")
elif url.endswith("/v1/commitments"):
    emit(sandbox.joinpath("commitments.json").read_text())
elif "/v1/balance/" in url:
    token_id = url.rstrip("/").split("/")[-1]
    bal = sandbox / "balances" / token_id
    emit(bal.read_text() if bal.is_file() else "00")
elif url.endswith("public.env"):
    emit(sandbox.joinpath("public.env").read_text())
elif url.endswith("join.toml"):
    emit("join\n", "200")
else:
    emit("ok\n")
EOF
  for extra in forge cargo; do
    printf '%s\n' '#!/usr/bin/env bash' 'exit 0' > "$bin/$extra"
  done
  chmod +x "$bin"/*
}

write_leaf_stubs() {
  local laptop="$1" remote="$2"
  mkdir -p "$laptop/scripts" "$remote/scripts" "$remote/configuration/toml" "$remote/temp"
  cp "$ORCH" "$laptop/scripts/nf4-sepolia-e2e"
  chmod +x "$laptop/scripts/nf4-sepolia-e2e"
  cat > "$laptop/scripts/nf4-laptop-tunnel" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
  cat > "$laptop/scripts/nf4-deploy" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/local-deploy.log"
if [[ "$*" == *--fresh-start* ]]; then
  echo "Fresh start on client. Keeping deploy.env"
  exit 0
fi
if [[ "$*" == *--check* ]]; then
  echo "NF4_CLIENT2_ADDRESS=0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004"
  echo "client 2 balance: 1000000000000000000"
  echo "Wallet check OK. Client 2 was not started."
  if [[ -f "${NF4_SANDBOX}/deployed" ]]; then
    echo "join HTTP 200"
    exit 0
  fi
  echo "curl: (7) Failed to connect"
  echo "join HTTP 000"
  exit 7
fi
if [[ "$*" == *--late-join* ]]; then
  mkdir -p temp
  printf "C2_ZKP='%s'\n" "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" > temp/zkp.public
  echo "Certified user-4"
  exit 0
fi
echo "unexpected local deploy: $*" >&2
exit 1
EOF
  cat > "$laptop/scripts/nf4-test" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/local-test.log"
step=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --step) step="$2"; shift 2 ;;
    --step=*) step="${1#--step=}"; shift ;;
    *) shift ;;
  esac
done
mkdir -p temp/sepolia-test
l2="$(cat "${NF4_SANDBOX}/l2")"
case "$step" in
  late-deposit)
    mkdir -p temp/sepolia-step11
    for name in ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step11/${name}.request-id"; done
    echo $((l2 + 1)) > "${NF4_SANDBOX}/l2"
    ;;
  sync)
    printf '{"phase":"Synchronized","layer_2_block_number":%s}\n' "$(cat "${NF4_SANDBOX}/l2")" > temp/sepolia-test/client-sync.json
    ;;
  balances)
    echo 12 > "${NF4_SANDBOX}/balances/0x07d1"
    echo "ERC20 28"
    ;;
  return-721)
    mkdir -p temp/sepolia-step17
    echo queued > temp/sepolia-step17/ERC721.request-id
    echo $((l2 + 1)) > "${NF4_SANDBOX}/l2"
    ;;
  withdraw)
    mkdir -p temp/sepolia-step18-c2
    for name in ERC20 ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step18-c2/${name}.request-id"; done
    echo "descrow 0xf3b85fc2" > temp/sepolia-step18-c2/client.log
    echo 2 > "${NF4_SANDBOX}/l1-0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004"
    ;;
  l1-balances) echo "C2 2" ;;
  soak-check) echo soak >> temp/sepolia-test/soak.txt ;;
  soak-deposit)
    mkdir -p temp/sepolia-step21
    echo queued > temp/sepolia-step21/ERC20.request-id
    echo $((l2 + 1)) > "${NF4_SANDBOX}/l2"
    ;;
  *) echo "unexpected local step $step" >&2; exit 1 ;;
esac
EOF
  cat > "$remote/scripts/nf4-deploy" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/remote-deploy.log"
host="${NF4_FAKE_HOST:-10.0.0.6}"
if [[ "$*" == *--fresh-start* ]]; then
  echo "Fresh start on operator. Keeping deploy.env"
  exit 0
fi
if [[ "$*" == *--check* && "$*" != *--file* ]]; then
  echo "Operator host: ${host}"
  echo "Join URL: http://${host}:8080/configuration/toml/join.toml"
  echo "NF4_JOIN_URL=http://${host}:8080/configuration/toml/join.toml"
  exit 0
fi
if [[ "$*" == *--check* ]]; then
  echo "NF4_JOIN_URL=http://${host}:8080/configuration/toml/join.toml"
  echo "  deployer, proposer, and Client 1 are the same existing wallet."
  echo "deployer balance: 1000000000000000000"
  echo "client 2 balance: 1000000000000000000"
  echo "Wallet check OK. No contracts were deployed."
  exit 0
fi
mkdir -p configuration/toml temp/sepolia-e2e
cat > configuration/toml/addresses.toml <<'ADDR'
nightfall = "0xb769327ed0D36216967319B023ec9C7f9dAa5576"
round_robin = "0x98b67fD7B25B0b9E099F950560D4B47693D0a6F0"
x509 = "0x589Fe721B1251ef47914F6871f8c2E1B63E7CAd5"
verifier = "0xA98EA5690b9b3227bB69a95dA210e958cE678dFb"
ADDR
cat > configuration/toml/public.env <<'PUB'
C1='0x2021c2Ca1791E5b50D3e882f979b03610db3B8A3'
C2='0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004'
TOKEN20='0x1111111111111111111111111111111111111111'
TOKEN721='0x2222222222222222222222222222222222222222'
TOKEN1155='0x3333333333333333333333333333333333333333'
TOKEN3525='0x4444444444444444444444444444444444444444'
ID20='0x00'
ID721_C1='0x03e9'
ID721_C2='0x03ea'
ID1155='0x07d1'
ID3525_C1='0x0bb9'
ID3525_C2='0x0bba'
C1_ZKP='bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
PUB
chmod 644 configuration/toml/public.env
cp configuration/toml/public.env "${NF4_SANDBOX}/public.env"
echo "Certified user-2"
echo "Certified user-3"
echo "Published C1_ZKP to configuration/toml/public.env"
echo "public.env mode 644"
touch "${NF4_SANDBOX}/deployed"
if [[ -f deploy.env ]]; then
  grep -v '^NF4_REDEPLOY=' deploy.env > deploy.env.tmp || true
  mv deploy.env.tmp deploy.env
fi
exit 0
EOF
  cat > "$remote/scripts/nf4-test" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/remote-test.log"
step=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --step) step="$2"; shift 2 ;;
    --step=*) step="${1#--step=}"; shift ;;
    *) shift ;;
  esac
done
mkdir -p temp/sepolia-test configuration/toml
l2="$(cat "${NF4_SANDBOX}/l2")"
case "$step" in
  tokens)
    touch temp/sepolia-test/minted
    grep -q '^TOKEN20=' configuration/toml/public.env || echo "TOKEN20='0x1111111111111111111111111111111111111111'" >> configuration/toml/public.env
    ;;
  baseline)
    printf '{"phase":"Synchronized","layer_2_block_number":%s}\n' "$l2" > temp/sepolia-test/baseline-sync.json
    ;;
  deposit)
    if [[ -f "${NF4_SANDBOX}/no-ready" ]]; then
      echo "not ready" > temp/sepolia-step10/record.txt
      exit 0
    fi
    mkdir -p temp/sepolia-step10
    for name in ERC20 ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step10/${name}.request-id"; done
    printf 'READY\n' > temp/sepolia-step10/record.txt
    ;;
  sync)
    printf '{"phase":"Synchronized","layer_2_block_number":%s}\n' "$(cat "${NF4_SANDBOX}/l2")" > temp/sepolia-test/operator-sync.json
    ;;
  accept-zkp)
    echo "C2_ZKP='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'" >> configuration/toml/public.env
    ;;
  transfer)
    mkdir -p temp/sepolia-step12
    for name in ERC20 ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step12/${name}.request-id"; done
    date -u > temp/sepolia-step12/confirmed
    ;;
  transfer-evidence)
    echo "abbreviated" > temp/sepolia-step12/proposer.log
    ;;
  stop-client1)
    rm -f .client-up
    ;;
  restart-client1)
    touch .client-up
    ;;
  withdraw)
    mkdir -p temp/sepolia-step18-c1
    for name in ERC20 ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step18-c1/${name}.request-id"; done
    echo "descrow 0xf3b85fc2" > temp/sepolia-step18-c1/client.log
    echo 2 > "${NF4_SANDBOX}/l1-0x2021c2Ca1791E5b50D3e882f979b03610db3B8A3"
    ;;
  l1-balances) echo "C1 2" ;;
  restart-proposer)
    printf '%s\n' '[{"layer2_block_number": 4}]' > temp/sepolia-test/proposer-blocks-before.json
    printf '%s\n' '[{"layer2_block_number": 4}]' > temp/sepolia-test/proposer-blocks-after.json
    date -u > temp/sepolia-test/proposer-stopped-at.txt
    ;;
  deposit-again)
    mkdir -p temp/sepolia-step20
    echo queued > temp/sepolia-step20/ERC20.request-id
    echo 0a > "${NF4_SANDBOX}/balances/0x00"
    echo $((l2 + 1)) > "${NF4_SANDBOX}/l2"
    ;;
  soak-check)
    echo "soak https://secret.example/v1" >> temp/sepolia-test/soak.txt
    echo "soak https://secret.example/v1" >> temp/sepolia-test/report.md
    ;;
  *) echo "unexpected remote step $step" >&2; exit 1 ;;
esac
EOF
  chmod +x "$laptop/scripts/"* "$remote/scripts/"*
}

write_env() {
  local laptop="$1" remote="$2" key="$3"
  cat > "$laptop/deploy.env" <<EOF
NF4_ROLE=client
NF4_NETWORK=testnet
NF4_CHAIN_ID=11155111
NF4_RPC_WSS=wss://sepolia.example/ws
NF4_RPC_HTTP=https://sepolia.example/rpc
NF4_CONFIRM=apply
NF4_CLIENT_KEY=${key}
NF4_LATE_JOIN=false
EOF
  chmod 600 "$laptop/deploy.env"
  cat > "$remote/deploy.env" <<EOF
NF4_ROLE=operator
NF4_NETWORK=testnet
NF4_CHAIN_ID=11155111
NF4_RPC_WSS=wss://sepolia.example/ws
NF4_RPC_HTTP=https://sepolia.example/rpc
NF4_CONFIRM=apply
NF4_DEPLOYER_KEY=${DEPLOYER_KEY}
NF4_CLIENT2_ADDRESS=0x0000000000000000000000000000000000000001
NF4_ZKP_MNEMONIC='spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work'
CANARY_LINE=keep-me
EOF
  chmod 600 "$remote/deploy.env"
  touch "$remote/.client-up"
}

prepare() {
  SANDBOX="$(mktemp -d "${TMPDIR:-/tmp}/nf4-e2e.XXXXXX")"
  LAPTOP="${SANDBOX}/laptop"
  REMOTE="${SANDBOX}/remote"
  mkdir -p "$SANDBOX/bin" "$SANDBOX/balances" "$SANDBOX/ssh"
  write_fakes "$SANDBOX/bin"
  write_leaf_stubs "$LAPTOP" "$REMOTE"
  write_env "$LAPTOP" "$REMOTE" "$CLIENT_KEY"
  echo 2 > "$SANDBOX/l2"
  echo 1 > "$SANDBOX/l1-${C1}"
  echo 1 > "$SANDBOX/l1-${C2}"
  echo 28 > "$SANDBOX/balances/0x00"
  echo 0a > "$SANDBOX/balances/0x07d1"
  echo 12 > "$SANDBOX/balances/0x07d1.wait"
  printf '%s\n' 12 > "$SANDBOX/balances/0x07d1"
  # late-deposit wants 10 (0a). balances stage wants 18 (12). Start at 0a and
  # switch after late-deposit by letting the balances stub rewrite it.
  echo 0a > "$SANDBOX/balances/0x07d1"
  cat > "$SANDBOX/commitments.json" <<'EOF'
[
  {"native_token_id": "1002", "status": "Unspent", "value": "0x00", "layer_2_block_number": 3},
  {"native_token_id": "1001", "status": "Unspent", "value": "0x00", "layer_2_block_number": 4},
  {"native_token_id": "3002", "status": "Unspent", "value": "0x64", "layer_2_block_number": 3}
]
EOF
  cat > "$SANDBOX/public.env" <<'EOF'
C1='0x2021c2Ca1791E5b50D3e882f979b03610db3B8A3'
C2='0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004'
TOKEN20='0x1111111111111111111111111111111111111111'
TOKEN721='0x2222222222222222222222222222222222222222'
TOKEN1155='0x3333333333333333333333333333333333333333'
TOKEN3525='0x4444444444444444444444444444444444444444'
ID20='0x00'
ID1155='0x07d1'
EOF
  touch "$SANDBOX/bastion-up" "$SANDBOX/forwards-up" "$SANDBOX/az-logged-in"
  mkdir -p "$SANDBOX/home/.foundry/bin" "$SANDBOX/home/.cargo/bin"
  for cmd in "$SANDBOX/bin"/*; do
    ln -sf "$cmd" "$SANDBOX/home/.foundry/bin/$(basename "$cmd")"
  done
  export HOME="$SANDBOX/home"
  : > "$SANDBOX/local-deploy.log"
  : > "$SANDBOX/remote-deploy.log"
  : > "$SANDBOX/local-test.log"
  : > "$SANDBOX/remote-test.log"
  export NF4_SANDBOX="$SANDBOX"
  export NF4_SSH_LOG="${SANDBOX}/ssh"
  export NF4_E2E_SSH_KEY="${SANDBOX}/id_ecdsa"
  export NF4_E2E_POLL_INTERVAL=0
  export NF4_E2E_POLL_MAX=4
  export NF4_E2E_REMOTE_DIR="$REMOTE"
  export PATH="${SANDBOX}/bin:${PATH}"
  : > "$NF4_E2E_SSH_KEY"
  unset NF4_FAKE_HOST || true
}

run_orch() {
  local out="${SANDBOX}/out.txt"
  set +e
  "$LAPTOP/scripts/nf4-sepolia-e2e" --remote-dir "$REMOTE" --file deploy.env "$@" >"$out" 2>&1
  local rc=$?
  set -e
  printf '%s\n' "$rc"
}

assert_secret_boundary() {
  need_not "${NF4_SSH_LOG}/argv" "$CLIENT_KEY" "client key was passed to ssh"
  need_not "${NF4_SSH_LOG}/argv" "$DEPLOYER_KEY" "deployer key was passed to ssh"
  need_not "${NF4_SSH_LOG}/argv" "local.env" "local.env was named on the ssh command"
  need_not "${NF4_SSH_LOG}/argv" "cat deploy.env" "deploy.env was printed over ssh"
  need_not "${NF4_SSH_LOG}/stdin" "$CLIENT_KEY" "client key was on ssh stdin"
  need_not "${NF4_SSH_LOG}/stdin" "$DEPLOYER_KEY" "deployer key was on ssh stdin"
  need_not "${SANDBOX}/out.txt" "$CLIENT_KEY" "client key was printed"
  need_not "${SANDBOX}/out.txt" "$DEPLOYER_KEY" "deployer key was printed"
  need_not "${SANDBOX}/out.txt" "sepolia.example" "RPC URL was printed"
  if [[ -f "$LAPTOP/temp/sepolia-e2e/report.md" ]]; then
    need_not "$LAPTOP/temp/sepolia-e2e/report.md" "$CLIENT_KEY" "client key in the report"
    need_not "$LAPTOP/temp/sepolia-e2e/report.md" "sepolia.example" "RPC URL in the report"
    need_not "$LAPTOP/temp/sepolia-e2e/report.md" "https://" "https URL survived the report scrub"
  fi
}

case_help() {
  prepare
  "$LAPTOP/scripts/nf4-sepolia-e2e" --help >"$SANDBOX/out.txt"
  need "$SANDBOX/out.txt" "soak-deposit" "help missing stages"
  [[ ! -s "${NF4_SSH_LOG}/argv" ]] || fail "help invoked ssh"
  echo "ok help"
}

case_anvil_key() {
  prepare
  write_env "$LAPTOP" "$REMOTE" "$ANVIL_KEY"
  rc="$(run_orch --through preflight)"
  [[ "$rc" != 0 ]] || fail "anvil key was accepted"
  need "$SANDBOX/out.txt" "Anvil key" "anvil refusal missing"
  need_not "$SANDBOX/out.txt" "$ANVIL_KEY" "anvil key was printed"
  echo "ok anvil key refused"
}

case_bad_join() {
  prepare
  export NF4_FAKE_HOST=127.0.0.1
  rc="$(run_orch --through check)"
  [[ "$rc" != 0 ]] || fail "loopback join URL was accepted"
  need "$SANDBOX/out.txt" "Refusing NF4_JOIN_URL" "join refusal missing"
  need_not "$LAPTOP/deploy.env" "NF4_JOIN_URL=http://127.0.0.1" "loopback join URL was written"
  need_not "${NF4_SSH_LOG}/argv" "$CLIENT_KEY" "client key leaked while refusing the join URL"
  echo "ok loopback join URL refused"
}

case_client_before_ready() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e"
  echo deposit > "$LAPTOP/temp/sepolia-e2e/state"
  printf '%s\n' "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" >> "$LAPTOP/deploy.env"
  rc="$(run_orch --through late-join)"
  [[ "$rc" != 0 ]] || fail "Client 2 started without READY"
  need "$SANDBOX/out.txt" "READY" "missing READY refusal"
  need_not "$SANDBOX/local-deploy.log" "--late-join" "late-join ran before READY"
  echo "ok Client 2 stayed down without READY"
}

case_resume_no_repost() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e" "$REMOTE/temp/sepolia-step12" "$REMOTE/configuration/toml"
  echo late-deposit > "$LAPTOP/temp/sepolia-e2e/state"
  printf '%s\n' "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" >> "$LAPTOP/deploy.env"
  printf '%s\n' "NF4_TUNNEL_HTTP=http://127.0.0.1:18080" >> "$LAPTOP/deploy.env"
  printf '%s\n' "NF4_TUNNEL_PROPOSER=http://127.0.0.1:13001" >> "$LAPTOP/deploy.env"
  for name in ERC20 ERC721 ERC1155 ERC3525; do
    echo queued > "$REMOTE/temp/sepolia-step12/${name}.request-id"
  done
  date -u > "$REMOTE/temp/sepolia-step12/confirmed"
  echo "C2_ZKP='${ZKP}'" > "$REMOTE/configuration/toml/public.env"
  rc="$(run_orch --through transfer)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "resume exited $rc"; }
  need "$SANDBOX/out.txt" "transfers already confirmed" "resume did not skip the transfer POST"
  if grep -E -- '--step[= ]transfer($| )' "$SANDBOX/remote-test.log"; then
    fail "transfer step ran on resume"
  fi
  need_not "$SANDBOX/remote-deploy.log" "--file deploy.env" "resume redeployed"
  assert_secret_boundary
  echo "ok resume did not redeploy or repost"
}

case_torn() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e" "$LAPTOP/temp/sepolia-step11"
  echo late-join > "$LAPTOP/temp/sepolia-e2e/state"
  printf '%s\n' "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" >> "$LAPTOP/deploy.env"
  echo queued > "$LAPTOP/temp/sepolia-step11/ERC721.request-id"
  rc="$(run_orch --through late-deposit)"
  [[ "$rc" != 0 ]] || fail "torn directory was posted over"
  need "$SANDBOX/out.txt" "torn" "torn refusal missing"
  [[ -f "$LAPTOP/temp/sepolia-step11/ERC721.request-id" ]] || fail "torn directory was deleted"
  [[ ! -f "$LAPTOP/temp/sepolia-step11/ERC1155.request-id" ]] || fail "torn directory was completed by a new POST"
  echo "ok torn directory was preserved"
}

case_fresh() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e"
  echo deposit > "$LAPTOP/temp/sepolia-e2e/state"
  rc="$(run_orch --fresh-start)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "fresh-start exited $rc"; }
  need "$SANDBOX/remote-deploy.log" "--fresh-start" "VM-03 fresh-start was not called"
  need "$SANDBOX/local-deploy.log" "--fresh-start" "laptop fresh-start was not called"
  [[ -f "$LAPTOP/deploy.env" && -f "$REMOTE/deploy.env" ]] || fail "fresh-start deleted deploy.env"
  need "$REMOTE/deploy.env" "$DEPLOYER_KEY" "fresh-start rewrote the remote key"
  need "$REMOTE/deploy.env" "CANARY_LINE=keep-me" "fresh-start rewrote an unrelated line"
  need "$LAPTOP/temp/sepolia-e2e/state" "tunnels" "fresh-start did not restart at check"
  need_not "$SANDBOX/local-deploy.log" "--late-join" "fresh-start started Client 2"
  assert_secret_boundary
  echo "ok fresh-start"
}

case_status() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e"
  echo check > "$LAPTOP/temp/sepolia-e2e/state"
  rc="$(run_orch --status)"
  [[ "$rc" == 0 ]] || fail "status exited $rc"
  need "$SANDBOX/out.txt" "state: check" "status missing state"
  [[ ! -s "$SANDBOX/remote-deploy.log" ]] || fail "status deployed"
  echo "ok status"
}

case_soak_words() {
  prepare
  mkdir -p "$LAPTOP/temp/sepolia-e2e"
  echo deposit-again > "$LAPTOP/temp/sepolia-e2e/state"
  printf '%s\n' "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" >> "$LAPTOP/deploy.env"
  printf '%s\n' "NF4_TUNNEL_HTTP=http://127.0.0.1:18080" >> "$LAPTOP/deploy.env"
  printf '%s\n' "NF4_TUNNEL_PROPOSER=http://127.0.0.1:13001" >> "$LAPTOP/deploy.env"
  export NF4_E2E_NOW=1000
  rc="$(run_orch --through soak)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "soak start exited $rc"; }
  need "$SANDBOX/out.txt" "soak incomplete" "short soak was called a pass"
  need "$SANDBOX/out.txt" "not a 48 hour pass" "short soak did not say it was not a pass"
  rc="$(run_orch --through soak-deposit)"
  [[ "$rc" != 0 ]] || fail "soak-deposit ran without an end snapshot"
  need "$SANDBOX/out.txt" "not a 48 hour pass" "missing rehearsal warning"
  rc="$(run_orch --through soak-deposit --allow-short-soak)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "rehearsal exited $rc"; }
  need "$SANDBOX/out.txt" "soak skipped" "rehearsal did not say the soak was skipped"
  need_not "$SANDBOX/out.txt" "soak end snapshot recorded" "rehearsal recorded a fake end snapshot"
  echo "ok soak wording"
}

case_happy() {
  prepare
  rc="$(run_orch --through soak-deposit --allow-short-soak)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "happy path exited $rc"; }
  need "$SANDBOX/out.txt" "late-join OK" "Client 2 did not join"
  need "$SANDBOX/out.txt" "deposit READY" "deposit gate missing"
  need "$SANDBOX/out.txt" "withdraw OK" "withdraw gate missing"
  need "$SANDBOX/out.txt" "soak skipped" "short soak was not labelled"
  need "$REMOTE/deploy.env" "NF4_CLIENT2_ADDRESS=${C2}" "Client 2 address was not pinned"
  need "$REMOTE/deploy.env" "CANARY_LINE=keep-me" "remote edit rewrote an unrelated line"
  need "$REMOTE/deploy.env" "$DEPLOYER_KEY" "remote edit disturbed the deployer key"
  need_not "$REMOTE/deploy.env" "NF4_JOIN_URL=" "join URL was written on VM-03"
  need "$LAPTOP/deploy.env" "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" "join URL was not the printed host"
  need "$LAPTOP/deploy.env" "NF4_TUNNEL_HTTP=http://127.0.0.1:18080" "tunnel line missing"
  need_not "$LAPTOP/deploy.env" "NF4_JOIN_URL=http://127.0.0.1" "join URL was rewritten to the forward"
  [[ -f "$REMOTE/temp/zkp.public" ]] || fail "zkp.public was not copied"
  need "$REMOTE/temp/zkp.public" "$ZKP" "copied zkp did not match"
  need "$SANDBOX/local-deploy.log" "--late-join" "Client 2 deploy did not run"
  grep -qx -- '--file deploy.env' "$SANDBOX/remote-deploy.log" || fail "operator deploy did not run"
  assert_secret_boundary
  echo "ok happy path"
}

# The late-join ordering assertion above is loose. Check the logs directly.
case_join_after_ready() {
  prepare
  rc="$(run_orch --through late-join)"
  [[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "late-join path exited $rc"; }
  python3 - "$SANDBOX/remote-test.log" "$SANDBOX/local-deploy.log" <<'PY'
import pathlib, sys
remote, local = sys.argv[1:]
remote_text = pathlib.Path(remote).read_text()
local_text = pathlib.Path(local).read_text()
if "--step deposit" not in remote_text:
    raise SystemExit("deposit step missing")
if "--late-join" not in local_text:
    raise SystemExit("late-join missing")
if local_text.index("--late-join") >= 0 and "--step deposit" not in remote_text:
    raise SystemExit("late-join without deposit")
PY
  echo "ok late-join followed READY"
}

[[ -x "$ORCH" || -f "$ORCH" ]] || fail "missing $ORCH"
chmod +x "$ORCH"

case_help
case_anvil_key
case_bad_join
case_client_before_ready
case_resume_no_repost
case_torn
case_fresh
case_status
case_soak_words
case_join_after_ready
case_happy

echo "all nf4-sepolia-e2e tests passed"
