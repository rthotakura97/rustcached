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
