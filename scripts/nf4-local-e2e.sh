#!/usr/bin/env bash
# Unattended local Anvil run of doc/Local Assistant Quick Start.md.
# Writes reports/local-e2e-<utc>/report.md. No secrets in the report.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

NF4="${ROOT}/scripts/nf4"
ANVIL_KEY0="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
ANVIL_ADDR0="0xf39Fd6e51aad88F6F4ce6ab8827279cffFb92266"
ANVIL_ADDR1="0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
C1_MNEMONIC="spice split denial symbol resemble knock hunt trial make buzz attitude mom slice define clinic kid crawl guilt frozen there cage light secret work"
C2_MNEMONIC="wink shell monkey fiscal exit great friend motor arrange file coffee leg catch drip amateur simple plastic win seat circle couch differ stomach law"
CHILD_PATH="m/44'/60'/0'/0/0"
CERT_DIR="blockchain_assets/test_contracts/X509/_certificates/user"

ID20="0x00"
ID721_C1="0x03e9"
ID721_C2="0x03ea"
ID1155="0x07d1"
ID3525_C1="0x0bb9"
ID3525_C2="0x0bba"

REUSE=0
SOAK_HOURS=0
RUN_DIR=""
STEPS_FILE=""
OVERALL="Pass"
FAIL_REASON=""

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*"; }
die() {
  OVERALL="Fail"
  FAIL_REASON="$*"
  log "FAIL: $*"
  dump_logs || true
  write_report || true
  exit 1
}

record() {
  local name="$1" status="$2" evidence="${3:-}"
  printf '%s\t%s\t%s\n' "$name" "$status" "$evidence" >>"$STEPS_FILE"
  log "[$status] $name"
}

dump_logs() {
  docker logs nf4_indie_client --tail 80 >"${RUN_DIR}/client1.tail.log" 2>&1 || true
  docker logs nf4_indie_client2 --tail 80 >"${RUN_DIR}/client2.tail.log" 2>&1 || true
  docker logs nf4_indie_proposer --tail 80 >"${RUN_DIR}/proposer.tail.log" 2>&1 || true
}

wait_http() {
  local url="$1" want="${2:-Healthy}" timeout="${3:-90}"
  local i body
  for i in $(seq 1 "$timeout"); do
    body="$(curl -fsS --max-time 3 "$url" 2>/dev/null || true)"
    if [[ "$body" == *"$want"* ]]; then
      return 0
    fi
    sleep 1
  done
  return 1
}

json_get() {
  python3 -c 'import json,sys; d=json.load(sys.stdin)
path=sys.argv[1].split(".")
cur=d
for p in path:
    if isinstance(cur, dict):
        cur=cur[p]
    else:
        sys.exit(1)
if isinstance(cur, (dict, list)):
    print(json.dumps(cur))
else:
    print(cur)' "$1"
}

derive_key() {
  local port="$1" mnemonic="$2"
  curl -sS -X POST "http://127.0.0.1:${port}/v1/deriveKey" \
    -H 'Content-Type: application/json' \
    --data "$(python3 -c 'import json,sys; print(json.dumps({"mnemonic":sys.argv[1],"child_path":sys.argv[2]}))' "$mnemonic" "$CHILD_PATH")"
}

post_cert() {
  local port="$1" user="$2" keyfield="$3"
  curl -sS -D "${RUN_DIR}/curl/cert-${port}.hdr" -o "${RUN_DIR}/curl/cert-${port}.body" \
    --request POST "http://127.0.0.1:${port}/v1/certification" \
    --form "certificate=@${CERT_DIR}/${user}.der;type=application/pkix-cert" \
    --form "${keyfield}=@${CERT_DIR}/${user}.priv_key;type=application/octet-stream"
  local code
  code="$(awk 'NR==1{print $2}' "${RUN_DIR}/curl/cert-${port}.hdr")"
  [[ "$code" == 200 || "$code" == 202 ]] || die "certification on :${port} returned HTTP ${code}"
}

# POST JSON; writes headers/body; prints x-request-id (may be empty).
post_json() {
  local name="$1" url="$2" body="$3"
  local hdr="${RUN_DIR}/curl/${name}.hdr"
  local out="${RUN_DIR}/curl/${name}.body"
  curl -sS -D "$hdr" -o "$out" -H 'Content-Type: application/json' -X POST "$url" --data-raw "$body"
  awk -F': ' 'tolower($1)=="x-request-id"{gsub(/\r/,"",$2); print $2}' "$hdr"
}

wait_request() {
  local port="$1" id="$2" timeout="${3:-180}"
  local i st status
  [[ -n "$id" ]] || die "missing request id"
  for i in $(seq 1 "$timeout"); do
    st="$(curl -fsS --max-time 5 "http://127.0.0.1:${port}/v1/request/${id}" 2>/dev/null || true)"
    status="$(printf '%s' "$st" | json_get status 2>/dev/null || true)"
    case "$status" in
      Submitted|Confirmed|Complete|Succeeded|Success) echo "$status"; return 0 ;;
      Failed|Error|Rejected)
        echo "$st" >"${RUN_DIR}/curl/request-${id}.json"
        die "request ${id} failed: $st"
        ;;
    esac
    sleep 1
  done
  echo "$st" >"${RUN_DIR}/curl/request-${id}.json"
  die "request ${id} still ${status:-unknown} after ${timeout}s"
}

wait_not_404() {
  local url="$1" timeout="${2:-180}"
  local i body
  for i in $(seq 1 "$timeout"); do
    body="$(curl -sS --max-time 5 "$url" 2>/dev/null || true)"
    if [[ "$body" != *"No such token"* && -n "$body" ]]; then
      printf '%s' "$body"
      return 0
    fi
    sleep 2
  done
  die "still No such token: $url"
}

hex_to_dec() {
  python3 -c 'import sys; s=sys.argv[1].strip().lower().removeprefix("0x"); print(int(s,16) if s else 0)' "$1"
}

assert_dec() {
  local name="$1" got_hex="$2" want="$3"
  local got
  got="$(hex_to_dec "$got_hex")"
  [[ "$got" == "$want" ]] || die "$name: expected $want, got $got ($got_hex)"
}

sync_phase() {
  local port="$1"
  curl -sS "http://127.0.0.1:${port}/v1/synchronisation" | json_get phase
}

max_l2() {
  local port="$1"
  curl -sS "http://127.0.0.1:${port}/v1/commitments" | python3 -c '
import json,sys
d=json.load(sys.stdin)
if not isinstance(d, list):
    print(0); raise SystemExit(0)
nums=[c.get("layer_2_block_number") for c in d if isinstance(c.get("layer_2_block_number"), int)]
print(max(nums) if nums else 0)
'
}

wait_sync_at_least() {
  local port="$1" min_block="$2" timeout="${3:-120}"
  local i phase cur
  for i in $(seq 1 "$timeout"); do
    phase="$(sync_phase "$port" 2>/dev/null || echo "")"
    cur="$(max_l2 "$port" 2>/dev/null || echo 0)"
    if [[ "$phase" == "Synchronized" ]] && [[ "$cur" =~ ^[0-9]+$ ]] && (( cur >= min_block )); then
      echo "$cur"
      return 0
    fi
    sleep 2
  done
  die "client :${port} not synced (phase=${phase:-none} l2=${cur:-none} want>=${min_block})"
}

on_chain_proposer_url() {
  python3 - <<'PY'
section = None
want = "[local.nightfall_deployer]"
with open("nightfall.toml", encoding="utf-8") as fh:
    for raw in fh:
        line = raw.strip()
        if line.startswith("[") and line.endswith("]"):
            section = line
            continue
        if section == want and line.startswith("default_proposer_url"):
            _, _, rest = line.partition("=")
            print(rest.strip().strip('"').strip("'"))
            raise SystemExit(0)
raise SystemExit("default_proposer_url not found in [local.nightfall_deployer]")
PY
}

write_report() {
  local report="${RUN_DIR}/report.md"
  local sha branch
  sha="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
  branch="$(git branch --show-current 2>/dev/null || echo unknown)"
  {
    echo "# Local Anvil Nightfall e2e report"
    echo
    echo "| Field | Value |"
    echo "|---|---|"
    echo "| Test date (UTC) | $(date -u '+%Y-%m-%d %H:%M') |"
    echo "| Environment | Local Anvil |"
    echo "| Repository / branch / commit | nightfall_4_CE / ${branch} / ${sha} |"
    echo "| Run mode | local |"
    echo "| Prover | mock |"
    echo "| Result | ${OVERALL} |"
    if [[ -n "$FAIL_REASON" ]]; then
      echo "| Failure | ${FAIL_REASON} |"
    fi
    echo
    echo "No private keys, mnemonics, or RPC secrets are included."
    echo
    echo "## Steps"
    echo
    echo "| Step | Status | Evidence |"
    echo "|---|---|---|"
    if [[ -s "$STEPS_FILE" ]]; then
      awk -F'\t' '{printf "| %s | %s | %s |\n", $1, $2, $3}' "$STEPS_FILE"
    fi
    echo
    echo "## Captured artifacts"
    echo
    echo "Directory: \`${RUN_DIR}\`"
  } >"$report"
  log "report: $report"
}

full_reset() {
  log "full reset"
  docker compose --profile anvil --profile configuration --profile indie-deployer \
    --profile indie-proposer --profile indie-client --profile indie-client2 --profile no_test \
    down --remove-orphans || true
  docker network rm nightfall_4_ce_nightfall_network 2>/dev/null || true
  rm -f configuration/toml/addresses.toml configuration/toml/contract_hashes.toml
}

start_anvil() {
  docker compose --profile anvil down 2>/dev/null || true
  docker network rm nightfall_4_ce_nightfall_network 2>/dev/null || true
  docker compose --profile anvil up -d
  local i
  for i in $(seq 1 30); do
    if cast chain-id --rpc-url ws://127.0.0.1:8545 >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done
  local cid
  cid="$(cast chain-id --rpc-url ws://127.0.0.1:8545)"
  [[ "$cid" == "31337" ]] || die "chain id $cid, expected 31337"
}

parse_mock_addr() {
  local label="$1" logf="$2"
  grep -E "Deployed ${label} to:" "$logf" | tail -1 | awk '{print $NF}'
}

main() {
for arg in "$@"; do
  case "$arg" in
    --reuse) REUSE=1 ;;
    --soak-hours=*) SOAK_HOURS="${arg#*=}" ;;
    --help|-h)
      echo "Usage: $0 [--reuse] [--soak-hours=N]"
      exit 0
      ;;
    *)
      echo "Unknown argument: $arg" >&2
      exit 2
      ;;
  esac
done

STAMP="$(date -u +%Y%m%d-%H%M%S)"
RUN_DIR="${ROOT}/reports/local-e2e-${STAMP}"
mkdir -p "$RUN_DIR/curl" "$RUN_DIR/mongo"
STEPS_FILE="${RUN_DIR}/steps.tsv"
: >"$STEPS_FILE"
OVERALL="Pass"
FAIL_REASON=""

trap 'if [[ "$OVERALL" == Pass ]]; then write_report; fi' EXIT

log "run dir $RUN_DIR"

if [[ "$REUSE" == 1 ]]; then
  log "reuse mode"
  wait_http http://127.0.0.1:3001/v1/health Healthy 30 || die "--reuse: proposer not healthy"
  url="$(on_chain_proposer_url)"
  if [[ "$url" != *indie-proposer* ]]; then
    die "--reuse refused: on-chain proposer URL is not docker DNS ($url)"
  fi
  record "reuse-preflight" "Pass" "$url"
else
  full_reset
  start_anvil
  record "anvil" "Pass" "chain-id 31337"
  log "wizard deploy --yes (this builds keys/images; can take several minutes)"
  "$NF4" wizard deploy --network local --yes
  record "wizard-deploy" "Pass" "indie-proposer on-chain URL"
  url="$(on_chain_proposer_url)"
  [[ "$url" == *indie-proposer* ]] || die "expected docker proposer URL, got $url"
fi

"$NF4" webhook start 8081 >/dev/null || true
"$NF4" wizard proposer --yes
wait_http http://127.0.0.1:3001/v1/health Healthy 90 || die "proposer health"
post_cert 3001 user-2 priv_key
record "proposer" "Pass" "health + user-2 cert"

"$NF4" wizard client --yes
wait_http http://127.0.0.1:3000/v1/health Healthy 90 || die "client1 health"
post_cert 3000 user-3 certificate_private_key
C1_ZKP="$(derive_key 3000 "$C1_MNEMONIC" | json_get zkp_public_key)"
[[ "${#C1_ZKP}" -eq 64 ]] || die "C1_ZKP length ${#C1_ZKP}"
record "client1-keys" "Pass" "C1_ZKP derived"

MOCK_LOG="${RUN_DIR}/mock-tokens.log"
"$NF4" client deploy-mock-tokens | tee "$MOCK_LOG"
TOKEN20="$(parse_mock_addr ERC20Mock "$MOCK_LOG")"
TOKEN721="$(parse_mock_addr ERC721Mock "$MOCK_LOG")"
TOKEN1155="$(parse_mock_addr ERC1155Mock "$MOCK_LOG")"
TOKEN3525="$(parse_mock_addr ERC3525Mock "$MOCK_LOG")"
NF="$(awk -F'"' '/^nightfall *=/{print $2}' configuration/toml/addresses.toml)"
[[ -n "$TOKEN20" && -n "$TOKEN721" && -n "$TOKEN1155" && -n "$TOKEN3525" && -n "$NF" ]] \
  || die "failed to parse mock token addresses"
RPC=http://127.0.0.1:8545
cast send "$TOKEN721" "mint(address,address,uint256)" "$ANVIL_ADDR0" "$NF" 1001 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
cast send "$TOKEN721" "mint(address,address,uint256)" "$ANVIL_ADDR1" "$NF" 1002 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$ANVIL_ADDR0" "$NF" 2001 20 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
cast send "$TOKEN1155" "mint(address,address,uint256,uint256)" "$ANVIL_ADDR1" "$NF" 2001 10 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$ANVIL_ADDR0" 3001 1 100 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
cast send "$TOKEN3525" "mint(address,address,uint256,uint256,uint256)" "$NF" "$ANVIL_ADDR1" 3002 1 100 --private-key "$ANVIL_KEY0" --rpc-url "$RPC" >/dev/null
record "mock-tokens" "Pass" "ERC20=$TOKEN20"

deposit() {
  local port="$1" name="$2" body="$3"
  local id
  id="$(post_json "$name" "http://127.0.0.1:${port}/v1/deposit" "$body")"
  wait_request "$port" "$id" 180 >/dev/null
}

deposit 3000 c1-erc20 "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
deposit 3000 c1-erc721 "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
deposit 3000 c1-erc1155 "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x14\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
deposit 3000 c1-erc3525 "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
assert_dec "C1 ERC20" "$(wait_not_404 "http://127.0.0.1:3000/v1/balance/${TOKEN20}/${ID20}")" 100
wait_not_404 "http://127.0.0.1:3000/v1/balance/${TOKEN721}/${ID721_C1}" >/dev/null
assert_dec "C1 ERC1155" "$(wait_not_404 "http://127.0.0.1:3000/v1/balance/${TOKEN1155}/${ID1155}")" 20
assert_dec "C1 ERC3525" "$(wait_not_404 "http://127.0.0.1:3000/v1/balance/${TOKEN3525}/${ID3525_C1}")" 100
record "client1-deposits" "Pass" "ERC20=100 ERC1155=20 ERC3525=100"

C1_BLOCK="$(max_l2 3000)"
"$NF4" up client2
wait_http http://127.0.0.1:3002/v1/health Healthy 90 || die "client2 health"
post_cert 3002 user-4 certificate_private_key
C2_ZKP="$(derive_key 3002 "$C2_MNEMONIC" | json_get zkp_public_key)"
[[ "${#C2_ZKP}" -eq 64 ]] || die "C2_ZKP length ${#C2_ZKP}"
# C2 will not hold C1 commitments. Sync means phase=Synchronized, not matching C1's max L2.
local i phase
for i in $(seq 1 60); do
  phase="$(sync_phase 3002 2>/dev/null || echo "")"
  [[ "$phase" == "Synchronized" ]] && break
  sleep 2
done
[[ "$phase" == "Synchronized" ]] || die "client2 not Synchronized (phase=${phase:-none}); C1 max L2=${C1_BLOCK}"
deposit 3002 c2-erc721 "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
deposit 3002 c2-erc1155 "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
deposit 3002 c2-erc3525 "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x64\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN721}/${ID721_C2}" >/dev/null
assert_dec "C2 ERC1155" "$(wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN1155}/${ID1155}")" 10
assert_dec "C2 ERC3525" "$(wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN3525}/${ID3525_C2}")" 100
record "client2-join" "Pass" "C2_ZKP derived"

transfer() {
  local name="$1" body="$2"
  local id
  id="$(post_json "$name" "http://127.0.0.1:3000/v1/transfer" "$body")"
  wait_request 3000 "$id" 180 >/dev/null
}

transfer t-erc20 "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
transfer t-erc721 "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
transfer t-erc1155 "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"recipientData\":{\"values\":[\"0x08\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
transfer t-erc3525 "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"recipientData\":{\"values\":[\"0x28\"],\"recipientCompressedZkpPublicKeys\":[\"${C2_ZKP}\"]},\"fee\":\"0x00\"}"
record "transfers-submitted" "Pass" "all four Submitted"

docker stop nf4_indie_client >/dev/null
record "client1-stopped" "Pass" "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

assert_dec "C2 ERC20 recv" "$(wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN20}/${ID20}" 180)" 40
wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN721}/${ID721_C1}" 180 >/dev/null
assert_dec "C2 ERC1155" "$(wait_not_404 "http://127.0.0.1:3002/v1/balance/${TOKEN1155}/${ID1155}" 60)" 18
record "client2-received" "Pass" "ERC20=40 ERC721=1001 ERC1155=18"

docker start nf4_indie_client >/dev/null
wait_http http://127.0.0.1:3000/v1/health Healthy 90 || die "client1 restart health"
C1_ZKP="$(derive_key 3000 "$C1_MNEMONIC" | json_get zkp_public_key)"
record "client1-restart" "Pass" "deriveKey after restart"

id="$(post_json ret-721 "http://127.0.0.1:3002/v1/transfer" "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"recipientData\":{\"values\":[\"0x00\"],\"recipientCompressedZkpPublicKeys\":[\"${C1_ZKP}\"]},\"fee\":\"0x00\"}")"
wait_request 3002 "$id" 180 >/dev/null
wait_not_404 "http://127.0.0.1:3000/v1/balance/${TOKEN721}/${ID721_C1}" 180 >/dev/null
record "erc721-return" "Pass" "C1 holds 1001"

withdraw() {
  local port="$1" name="$2" body="$3"
  local id
  id="$(post_json "$name" "http://127.0.0.1:${port}/v1/withdraw" "$body")"
  wait_request "$port" "$id" 180 >/dev/null
}
withdraw 3000 w1-20 "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x3c\",\"recipientAddress\":\"${ANVIL_ADDR0}\",\"fee\":\"0x00\"}"
withdraw 3000 w1-721 "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C1}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${ANVIL_ADDR0}\",\"fee\":\"0x00\"}"
withdraw 3000 w1-1155 "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x0c\",\"recipientAddress\":\"${ANVIL_ADDR0}\",\"fee\":\"0x00\"}"
withdraw 3000 w1-3525 "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C1}\",\"tokenType\":\"3\",\"value\":\"0x3c\",\"recipientAddress\":\"${ANVIL_ADDR0}\",\"fee\":\"0x00\"}"
withdraw 3002 w2-20 "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x28\",\"recipientAddress\":\"${ANVIL_ADDR1}\",\"fee\":\"0x00\"}"
withdraw 3002 w2-721 "{\"ercAddress\":\"${TOKEN721}\",\"tokenId\":\"${ID721_C2}\",\"tokenType\":\"2\",\"value\":\"0x00\",\"recipientAddress\":\"${ANVIL_ADDR1}\",\"fee\":\"0x00\"}"
withdraw 3002 w2-1155 "{\"ercAddress\":\"${TOKEN1155}\",\"tokenId\":\"${ID1155}\",\"tokenType\":\"1\",\"value\":\"0x12\",\"recipientAddress\":\"${ANVIL_ADDR1}\",\"fee\":\"0x00\"}"
withdraw 3002 w2-3525 "{\"ercAddress\":\"${TOKEN3525}\",\"tokenId\":\"${ID3525_C2}\",\"tokenType\":\"3\",\"value\":\"0x28\",\"recipientAddress\":\"${ANVIL_ADDR1}\",\"fee\":\"0x00\"}"
record "withdrawals" "Pass" "L2 submitted (L1 descrow may still settle)"

docker stop nf4_indie_proposer >/dev/null
sleep 2
docker start nf4_indie_proposer >/dev/null
wait_http http://127.0.0.1:3001/v1/health Healthy 90 || die "proposer restart health"
record "proposer-restart" "Pass" "Healthy"

deposit 3000 c1-post "{\"ercAddress\":\"${TOKEN20}\",\"tokenId\":\"${ID20}\",\"tokenType\":\"0\",\"value\":\"0x0a\",\"fee\":\"0x00\",\"deposit_fee\":\"0x00\"}"
record "post-recovery-deposit" "Pass" "ERC20 10"

if (( SOAK_HOURS > 0 )); then
  log "soak ${SOAK_HOURS}h (health only)"
  sleep $((SOAK_HOURS * 3600))
  wait_http http://127.0.0.1:3001/v1/health Healthy 10 || die "proposer unhealthy after soak"
  record "soak" "Pass" "${SOAK_HOURS}h"
else
  record "soak" "Pass" "skipped"
fi

write_report
log "done ${OVERALL}: ${RUN_DIR}/report.md"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
