(async () => {
try {
  const e = process.env;
  const want = [
    ["ERC20", e.REQ20, e.TOKEN20, e.ID20, 0n, 100n],
    ["ERC721", e.REQ721, e.TOKEN721, e.ID721_C1, 1001n, 0n],
    ["ERC1155", e.REQ1155, e.TOKEN1155, e.ID1155, 2001n, 20n],
    ["ERC3525", e.REQ3525, e.TOKEN3525, e.ID3525_C1, 3001n, 100n]
  ];
  const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  function hex(v) {
    const s = String(v ?? "").replace(/^0x/i, "");
    if (!/^[0-9a-f]{1,64}$/i.test(s)) throw Error("Invalid stored hex field");
    return s.toLowerCase().padStart(64, "0");
  }
  for (const [name, request, address, id, nativeId] of want) {
    if (!uuid.test(request || "") || !/^0x[0-9a-f]{40}$/i.test(address || "") ||
        !/^0x(?:[0-9a-f]{2}){1,32}$/i.test(id || "") || BigInt(id) !== nativeId) {
      throw Error(name + ": invalid request ID/address or wrong fixed token ID");
    }
  }
  if (new Set(want.map(w => w[1].toLowerCase())).size !== 4) throw Error("Request IDs must be distinct");
  const nf = db.getSiblingDB("nightfall");
  const blocks = new Set();
  let ready = true;
  for (const [name, rawRequest, address, id, nativeId, expected] of want) {
    const request = rawRequest.toLowerCase();
    const req = await nf.requests.findOne({uuid: request});
    print(name + " request_id=" + request + " request_status=" + (req ? req.status : "absent (possibly restarted)"));
    if (req && !["Queued", "Processing", "Submitted", "Confirmed"].includes(req.status)) {
      throw Error(name + ": request " + req.status + "; inspect client logs before any retry");
    }
    const mappings = await nf.request_commitment_mappings.find({request_id: request}).toArray();
    if (!mappings.length) {
      if (!req || !["Queued", "Processing"].includes(req.status)) throw Error(name + ": no commitment mapping; check request ID/client logs");
      print(name + " waiting for commitment mapping");
      ready = false;
      continue;
    }
    const keys = [...new Set(mappings.map(m => m.commitment_hash))];
    const rows = await nf.commitments.find({_id: {$in: keys}}).toArray();
    if (rows.length !== keys.length) throw Error(name + ": mapped commitment missing from database");
    const digest = require("crypto").createHash("sha256")
      .update(Buffer.from(hex(address) + hex(id), "hex")).digest("hex");
    const nfTokenId = hex((BigInt("0x" + digest) >> 4n).toString(16));
    if (rows.length !== 1) throw Error(name + ": expected one mapped commitment with deposit_fee=0");
    const r = rows[0];
    if (r.token_type !== name || hex(r.preimage.nf_token_id) !== nfTokenId ||
        r.preimage.salt.type !== "Deposit" || BigInt("0x" + hex(r.preimage.value)) !== expected) {
      throw Error(name + ": mapped commitment does not match this deposit's asset/value");
    }
    let l2 = null;
    if (r.layer_2_block_number !== null && r.layer_2_block_number !== undefined) {
      const raw = String(r.layer_2_block_number);
      l2 = Number(raw);
      if (!/^(0|[1-9][0-9]*)$/.test(raw) || !Number.isSafeInteger(l2)) throw Error(name + ": invalid L2 block number");
    }
    let blockTx = null;
    if (r.layer_1_transaction_hash !== null && r.layer_1_transaction_hash !== undefined) {
      const h = r.layer_1_transaction_hash;
      const s = typeof h === "string" ? h.replace(/^0x/i, "") : h.toString("hex");
      if (!/^[0-9a-f]{64}$/i.test(s)) throw Error(name + ": invalid block transaction hash");
      blockTx = "0x" + s.toLowerCase();
    }
    print(name + " token_id=" + nativeId + " commitment=" + r._id + " status=" + r.status +
      " value=" + expected + " l2=" + l2 + " block_tx=" + blockTx);
    if (r.status === "PendingCreation") ready = false;
    else if (r.status !== "Unspent" || l2 === null || blockTx === null) throw Error(name + ": inconsistent or already spent commitment; do not advance");
    if (r.status === "Unspent") blocks.add(l2);
  }
  print("L2 blocks for these deposits: " + ([...blocks].join(" ") || "none"));
  print(ready ? "READY" : "WAITING");
  quit(ready ? 0 : 2);
} catch (err) {
  console.error("CHECK FAILED: " + err.message);
  quit(1);
}
})();
