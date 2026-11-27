#!/usr/bin/env bash
# Validates scripts/nf4-sepolia-e2e stays on this machine.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ORCH="${ROOT}/scripts/nf4-sepolia-e2e"
CLIENT_KEY="0x1111111111111111111111111111111111111111111111111111111111111111"
DEPLOYER_KEY="0x2222222222222222222222222222222222222222222222222222222222222222"
ANVIL_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
C2="0xB4c4000eeA0cff47a27f8Ee15AFD87400Ef86004"

fail() {
  echo "FAIL $*" >&2
  exit 1
}

need() {
  grep -q -- "$2" "$1" || fail "$3"
}

need_not() {
  if grep -q -- "$2" "$1" 2>/dev/null; then
    fail "$3"
  fi
}

if grep -E -n 'ssh|bastion|az network' "$ORCH"; then
  fail "orchestrator still mentions a remote login"
fi

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
WORK="$SANDBOX/work"
mkdir -p "$WORK/scripts" "$WORK/temp" "$SANDBOX/bin"
cp "$ORCH" "$WORK/scripts/nf4-sepolia-e2e"
chmod +x "$WORK/scripts/nf4-sepolia-e2e"

cat > "$SANDBOX/bin/cast" <<'EOF'
#!/usr/bin/env bash
echo 11155111
EOF
cat > "$SANDBOX/bin/curl" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *http_code* ]]; then
  echo 200
  exit 0
fi
echo Healthy
EOF
cat > "$SANDBOX/bin/docker" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
cat > "$SANDBOX/bin/forge" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
cat > "$SANDBOX/bin/cargo" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
chmod +x "$SANDBOX/bin"/*
mkdir -p "$SANDBOX/home/.foundry/bin" "$SANDBOX/home/.cargo/bin"
for cmd in "$SANDBOX/bin"/*; do
  ln -sf "$cmd" "$SANDBOX/home/.foundry/bin/$(basename "$cmd")"
done
export HOME="$SANDBOX/home"
export PATH="$SANDBOX/bin:$PATH"

cat > "$WORK/scripts/nf4-deploy" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/deploy.log"
if [[ "$*" == *--fresh-start* ]]; then
  echo "Fresh start. Keeping deploy.env"
  exit 0
fi
if [[ "$*" == *--check* ]]; then
  echo "  deployer, proposer, and Client 1 are the same existing wallet."
  echo "Wallet check OK."
  exit 0
fi
if [[ "$*" == *--late-join* ]]; then
  mkdir -p temp
  echo "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" > temp/zkp.public
  echo "Certified user-4"
  exit 0
fi
mkdir -p configuration/toml
echo "nightfall = \"0xb769327ed0D36216967319B023ec9C7f9dAa5576\"" > configuration/toml/addresses.toml
echo "C1_ZKP='bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'" > configuration/toml/public.env
chmod 644 configuration/toml/public.env
echo "Certified user-2"
echo "Certified user-3"
echo "Published C1_ZKP"
exit 0
EOF
cat > "$WORK/scripts/nf4-test" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${NF4_SANDBOX}/test.log"
step=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --step) step="$2"; shift 2 ;;
    *) shift ;;
  esac
done
case "$step" in
  tokens) mkdir -p temp/sepolia-test; touch temp/sepolia-test/minted ;;
  baseline) ;;
  deposit)
    mkdir -p temp/sepolia-step10
    printf '%s\n' READY > temp/sepolia-step10/record.txt
    ;;
  transfer)
    mkdir -p temp/sepolia-step12
    touch temp/sepolia-step12/confirmed
    ;;
  stop-client1|restart-client1|l1-balances|restart-proposer|balances) ;;
  withdraw) mkdir -p temp/sepolia-step18-c1 temp/sepolia-step18-c2 ;;
  deposit-again)
    mkdir -p temp/sepolia-step20
    echo queued > temp/sepolia-step20/ERC20.request-id
    ;;
  late-deposit)
    mkdir -p temp/sepolia-step11
    for name in ERC721 ERC1155 ERC3525; do echo queued > "temp/sepolia-step11/${name}.request-id"; done
    ;;
  return-721)
    mkdir -p temp/sepolia-step17
    echo queued > temp/sepolia-step17/ERC721.request-id
    ;;
  soak-check) ;;
  soak-deposit)
    mkdir -p temp/sepolia-step21
    echo queued > temp/sepolia-step21/ERC20.request-id
    ;;
  accept-zkp) mkdir -p configuration/toml; echo "C2_ZKP='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'" >> configuration/toml/public.env ;;
  *) echo "unexpected step $step" >&2; exit 1 ;;
esac
EOF
chmod +x "$WORK/scripts/nf4-deploy" "$WORK/scripts/nf4-test"

write_env() {
  local role="$1" keyname="$2" key="$3"
  cat > "$WORK/deploy.env" <<EOF
NF4_ROLE=${role}
NF4_NETWORK=testnet
NF4_CHAIN_ID=11155111
NF4_RPC_WSS=wss://sepolia.example/ws
NF4_RPC_HTTP=https://sepolia.example/http
${keyname}=${key}
NF4_CLIENT2_ADDRESS=${C2}
CANARY_LINE=keep-me
EOF
  chmod 600 "$WORK/deploy.env"
}

run_orch() {
  local out="$SANDBOX/out.txt"
  set +e
  (
    cd "$WORK"
    export NF4_SANDBOX="$SANDBOX"
    "$WORK/scripts/nf4-sepolia-e2e" --file deploy.env "$@"
  ) >"$out" 2>&1
  local rc=$?
  set -e
  printf '%s\n' "$rc"
}

: > "$SANDBOX/deploy.log"
: > "$SANDBOX/test.log"

"$WORK/scripts/nf4-sepolia-e2e" --help >"$SANDBOX/help.txt"
need "$SANDBOX/help.txt" "Does not log in" "help still implies a remote login"
need "$SANDBOX/help.txt" "soak-deposit" "help missing client stages"
need "$SANDBOX/help.txt" "--network" "help missing network status"
echo "ok help"

write_env operator NF4_DEPLOYER_KEY "$ANVIL_KEY"
rc="$(run_orch --through preflight)"
[[ "$rc" != 0 ]] || fail "anvil key was accepted"
need "$SANDBOX/out.txt" "Anvil key" "anvil refusal missing"
need_not "$SANDBOX/out.txt" "$ANVIL_KEY" "anvil key was printed"
echo "ok anvil key refused"

rm -rf "$WORK/temp"
write_env client NF4_CLIENT_KEY "$CLIENT_KEY"
printf '%s\n' "NF4_JOIN_URL=http://127.0.0.1:8080/configuration/toml/join.toml" >> "$WORK/deploy.env"
rc="$(run_orch --through check)"
[[ "$rc" != 0 ]] || fail "loopback join URL was accepted"
need "$SANDBOX/out.txt" "Refusing NF4_JOIN_URL" "join refusal missing"
echo "ok loopback join URL refused"

rm -rf "$WORK/temp"
: > "$SANDBOX/deploy.log"
write_env client NF4_CLIENT_KEY "$CLIENT_KEY"
printf '%s\n' "NF4_JOIN_URL=http://10.0.0.6:8080/configuration/toml/join.toml" >> "$WORK/deploy.env"
rc="$(run_orch --through late-join)"
[[ "$rc" != 0 ]] || fail "Client 2 started without READY"
need "$SANDBOX/out.txt" "READY" "missing READY refusal"
need_not "$SANDBOX/deploy.log" "--late-join" "late-join ran before READY"
echo "ok Client 2 stayed down without READY"

rm -rf "$WORK/temp"
: > "$SANDBOX/deploy.log"
: > "$SANDBOX/test.log"
write_env operator NF4_DEPLOYER_KEY "$DEPLOYER_KEY"
rc="$(run_orch --through deposit)"
[[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "operator deposit exited $rc"; }
need "$SANDBOX/out.txt" "deposit READY" "deposit gate missing"
need_not "$WORK/deploy.env" "NF4_JOIN_URL=" "join URL was written into the operator file"
need "$WORK/deploy.env" "CANARY_LINE=keep-me" "operator run rewrote an unrelated line"
rc="$(run_orch --through transfer)"
[[ "$rc" != 0 ]] || fail "transfer ran without zkp.public"
need "$SANDBOX/out.txt" "temp/zkp.public" "missing copy instruction"
need_not "$SANDBOX/test.log" "--step transfer" "transfer step ran without the public key"
echo "ok operator stops for the copied public key"

mkdir -p "$WORK/temp"
printf '%s\n' aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa > "$WORK/temp/zkp.public"
rc="$(run_orch --through transfer)"
[[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "transfer after copy exited $rc"; }
need "$SANDBOX/test.log" "--step transfer" "transfer did not run after the copy"
echo "ok transfer after the user copies zkp.public"

rm -rf "$WORK/temp"
mkdir -p "$WORK/temp/sepolia-e2e" "$WORK/temp/sepolia-step11"
echo late-join > "$WORK/temp/sepolia-e2e/state"
echo queued > "$WORK/temp/sepolia-step11/ERC721.request-id"
write_env client NF4_CLIENT_KEY "$CLIENT_KEY"
: > "$SANDBOX/test.log"
rc="$(run_orch --through late-deposit)"
[[ "$rc" != 0 ]] || fail "torn directory was posted over"
need "$SANDBOX/out.txt" "torn" "torn refusal missing"
[[ -f "$WORK/temp/sepolia-step11/ERC721.request-id" ]] || fail "torn directory was deleted"
echo "ok torn directory was preserved"

rm -rf "$WORK/temp"
write_env operator NF4_DEPLOYER_KEY "$DEPLOYER_KEY"
echo keep-me >> "$WORK/deploy.env"
rc="$(run_orch --fresh-start)"
[[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "fresh-start exited $rc"; }
need "$SANDBOX/deploy.log" "--fresh-start" "fresh-start was not local"
need "$WORK/deploy.env" "$DEPLOYER_KEY" "fresh-start rewrote the key"
need "$WORK/deploy.env" "CANARY_LINE=keep-me" "fresh-start rewrote an unrelated line"
need_not "$SANDBOX/deploy.log" "--late-join" "fresh-start started Client 2"
need "$SANDBOX/out.txt" "does not log in" "fresh-start implied a remote wipe"
echo "ok fresh-start stays local"

rm -rf "$WORK/temp"
mkdir -p "$WORK/temp/sepolia-e2e"
echo check > "$WORK/temp/sepolia-e2e/state"
: > "$SANDBOX/deploy.log"
rc="$(run_orch --status)"
[[ "$rc" == 0 ]] || fail "status exited $rc"
need "$SANDBOX/out.txt" "state: check" "status missing state"
need "$SANDBOX/out.txt" "orchestrator: idle" "status missing idle line"
[[ ! -s "$SANDBOX/deploy.log" ]] || fail "status deployed"
echo "ok status"

: > "$SANDBOX/deploy.log"
rc="$(run_orch --network)"
[[ "$rc" != 0 ]] || fail "stub network was reported up"
need "$SANDBOX/out.txt" "chain_id: 11155111" "network status missing chain id"
need "$SANDBOX/out.txt" "network: down" "network status missing verdict"
[[ ! -s "$SANDBOX/deploy.log" ]] || fail "network status deployed"
echo "ok network status"

rm -rf "$WORK/temp"
mkdir -p "$WORK/temp/sepolia-e2e"
echo deposit-again > "$WORK/temp/sepolia-e2e/state"
export NF4_E2E_NOW=1000
rc="$(run_orch --through soak)"
[[ "$rc" == 0 ]] || { cat "$SANDBOX/out.txt" >&2; fail "soak start exited $rc"; }
need "$SANDBOX/out.txt" "soak incomplete" "short soak was called a pass"
need "$SANDBOX/out.txt" "not a 48 hour pass" "short soak did not say it was not a pass"
unset NF4_E2E_NOW
echo "ok soak wording"

echo "all nf4-sepolia-e2e tests passed"
