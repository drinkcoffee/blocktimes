## Block times 
This is a vibe coded Rust CLI that calls an Ethereum JSON-RPC endpoint and prints block numbers, block timestamps, UTC date/time, and the delta from the previous block in CSV format.


Usage:

```
cargo run -- --rpc-url https://rpc.immutable.com --start-block 19000000 --block-count 5 --block-interval 10000
```

Resulting output:

```
19000000,1740269298,2025-02-23 00:08:18 UTC,
19010000,1740279306,2025-02-23 03:15:06 UTC,10008
19020000,1740289314,2025-02-23 06:21:54 UTC,10008
19030000,1740299322,2025-02-23 09:28:42 UTC,10008
19040000,1740309330,2025-02-23 12:35:30 UTC,10008
```

Notes:
- `--rpc-url` can also be provided via the `ETH_RPC_URL` environment variable.
- If no args are supplied, the CLI prints the built-in help.
