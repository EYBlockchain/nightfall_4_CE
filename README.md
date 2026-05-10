# nightfall_4_CE
Community edition of Nightfall_4

_This code is not owned by EY and EY provides no warranty and disclaims any and all liability for use of this code. Users must conduct their own diligence with respect to use for their purposes and any and all usage is on an as-is basis and at your own risk._

Nightfall_4 is a ZK rollup build around the ZK Privacy of Nightfall. It enables one to transfer ERC20, ERC721, ERC1155 and ERC3525 tokens in privacy. Full details can be found in the /doc folder of this repository.

Please note that this software should be treated as experimental. It should not be used to make significant value transactions.

## Proposer MongoDB

For Docker-based development and sync testing, proposer MongoDB runs as a single-node replica set so the proposer can later use MongoDB transactions safely.

- `db_proposer` uses replica set `nf4-rs-proposer`
- `db_proposer2` uses replica set `nf4-rs-proposer2`
- proposer MongoDB URIs should include both `replicaSet=...` and `directConnection=true`

Examples:

```text
mongodb://nf4_db_proposer:27017/?replicaSet=nf4-rs-proposer&directConnection=true
mongodb://nf4_db_proposer2:27017/?replicaSet=nf4-rs-proposer2&directConnection=true
```

Quick verification:

```bash
docker exec nf4_db_proposer mongosh --quiet --eval 'rs.status()'
docker exec nf4_db_proposer2 mongosh --quiet --eval 'rs.status()'
```

This change is proposer-only. `db_client` and `db_client2` remain standalone for now and client-side MongoDB replica set conversion is intentionally deferred to a separate task.
