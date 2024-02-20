# distkv
Implementation of a distributed key-value store, similar to AWS DynamoDB, Redis, Apache Cassandra, etc.

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

