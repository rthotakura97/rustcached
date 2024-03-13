## Tenets
* ACID
    * Atomicity - Every transaction on the database is atomic in nature.
    * Consistency - The database should be strongly consistent.
    * Isolation - Every transaction is isolated and can run independently, thus supporting concurrency >= 1
    * Durability - Transactions are persisted to a durable endpoint to provide safety during power failures, crashes, network partitions, etc.
* Highly Available
    * Should be able to withstand node failures and network partitions
* Scalability
    * Easily scalable horizontally and/or vertically
* Fast and Performant
    * Uses volatile storage model of database-in-memory
    * Written with a highly performant language

## Characteristics
* Replication Strategy
    * Implementation of Raft (https://raft.github.io/) for consensus and replication
* Sharding/Partitioning Strategy
    * Data will be partitioned by a keyspace using a identifying partition key
    * Consistent hashing to map nodes to a keyspace


## Using consistent hashing
* We should use consistent hashing to map keys to servers. We need to find a discoverability algorithm, or we can have a config file to start that stores node information.

## Replication
* We need to replicate data from leaders (who server writes and reads) (reads because of strong consistency) to followers

## Leader election
* Probably some sort of heartbeating mechanism

## Memcached
Memcached takes away a lot of the single node complexity of a in-memory KV store. I wonder if it makes sense to use Memcached on the host and then build out additional functionality: consistent hashing for sharding data and then also replication to help with single node failures

Memcached protocol: https://github.com/memcached/memcached/blob/master/doc/protocol.txt
Memcached protocol wiki: https://github.com/memcached/memcached/wiki/Protocols 

### Internal data store
This will be a HashMap. Need to think more about how the data is structured. Memcached seeems to support unstructured data? not sure yet.

### Async framework
* I prefer to use tokio async framework. its really for IO bound applications
* We can probably spawn a tokio task per request. Tasks are lightweight and a lot can be spawned. We can also probably rate limit concurrency since concurrent tokio tasks still succumb to memory and CPU pressure
* Also consider actor model -> we have a single data source in memory, and rather than use locks,mutex,Arc,etc why not just have a single long living task own that data? We can probably put a queue in front of it to process events serially
    * We need to really create a small scope for this task because we want it to spend the LEAST amount of time possible since it is a blocking task. So really make it a wrapper around the internal HashMap
* The opposite of the actor model is for all the tokio tasks (outstanding requests) to access a shared HashMap. It will be likely wrapped by a Mutex
    * theoretically higher throughput and lower latency, so better performance
    * this is simpler than an actor model
    * however, we need to manage concurrency issues on the shared HashMap which is always painful
    * we need to also think about data access patterns to think about how much contention we can expect. obviously each customer has different access patterns. 
* I prefer to go with the actor model -> trading off some performance for strong isolation and minimal contention

### Encryption
Memcached does not natively support encryption. I am pretty sure it's just over a plain TCP connection. Likewise across hosts. I don't think it makes sense to support here. 

### Networking
Will use TcpListener to listen/send messages on a port.