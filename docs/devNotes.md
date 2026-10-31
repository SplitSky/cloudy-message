# Concepts needed
- TCP
- Sockets
- Binding
- Listening
- Client-Server model

## How does it work under the hood?
1. Socker is made
2. Socker is bound to a specific IP address and port
3. Listening - the server starts to listen for incoming connections
4. Accepting connections - when a client connects it accepts it
5. Reads the data from the client, processes it and sends a response
6. Connections are closed when communication is complete

## Common pitfalls
1. Concurrency -> Handling multiple clients requires proper threading or async/await
2. Resource management -> Close sockets to avoid leaks
3. Robust error handling -> Lean on Rust
4. Implement proper authentication and encryption
5. Optimise for high throughput and low latency
6. Use Rust's modules to organise code

## Crates to consider
### Basic implementation
std::net::{TcpListener, Tcp:Stream}
std::io::{Read, Write}

### Threading
also use std::thread

### Async/await
tokio::net and tokio::io
and also futures::Future

## Testing
use telnet or netcat

Write tests using #[cfg(test)]

# Milestones
1. Single partition log
    - append and read by offset
    - record format, length, payload
    - unit test for read, write, read+write all
2. Segments and index
    - roll to a new file at a size limit
    - name files by base offset
    - add sparse index (every N bytes store offset)
    - use binary search for reads
    - on startup scan the last segment and clean up
3. Partitions - A topic holds N partitions, each with its own log. Route by key hash
    - Use stable hash like CRC32
    - Give each partition one writer task fed by a channel, so writes to a paritition are searilised without a lock
    - Done when 100 concurrent producers write to 4 partitions and per-partition order holds.
4. Networking - Tokio TCP server with a length-prefixed binary protocol and two commands 
    - Produce (topic, key, value) -> Returning an offset
    - Fetch (topic, partition, offset, max_bytes)
    - Add a small CLI client
    - Done when a message is produced from one terminal and fetched from another
    - Make integration tests
5. Consumer offests
    - Store commited offests in an internal log (Kafka implementation)
    - Add Commit and FetchOffset
    - Done when a consumer restarts and resumes where it left off
    - Testing to do:
        - Crash test, property tests, benchmark throughput and p99 latency under a few fsync policies
