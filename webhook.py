from flask import Flask, request
from datetime import datetime
app = Flask(__name__)
@app.route("/webhook", methods=["POST"])
def webhook():
    data = request.get_json(silent=True)
    # Heuristics to classify payload
    kind = "TransactionEvent" if isinstance(data, dict) and "uuid" in data else \
           "BlockchainEvent"  if isinstance(data, dict) and "l1_txn_hash" in data else \
           "Unknown"
    print(f"[{datetime.utcnow().isoformat()}Z] {kind}: {data}")
    return "", 200
if __name__ == "__main__":
    # Bind to 0.0.0.0 so Docker containers can reach it
    app.run(host="0.0.0.0", port=8080)
