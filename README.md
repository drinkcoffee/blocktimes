## Block times 
This is a vibe coded rust project to call an RPC and print out the block numbers, block timestamps, and associated block date and time in CSV format.

Example parameters:

```
cargo run -- --rpc-url https://rpc.immutable.com --start-block 19000000 --block-count 5
```

Resulting output:

```
19000000,1740269298,2025-02-23 00:08:18 UTC
19000001,1740269300,2025-02-23 00:08:20 UTC
19000002,1740269302,2025-02-23 00:08:22 UTC
19000003,1740269304,2025-02-23 00:08:24 UTC
19000004,1740269306,2025-02-23 00:08:26 UTC
```
