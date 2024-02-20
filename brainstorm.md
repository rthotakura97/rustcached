## Using consistent hashing
* We should use consistent hashing to map keys to servers. We need to find a discoverability algorithm, or we can have a config file to start that stores node information.

## Replication
* We need to replicate data from leaders (who server writes and reads) (reads because of strong consistency) to followers

## Leader election
* Probably some sort of heartbeating mechanism

## Use memcached?
Memcached takes away a lot of the single node complexity of a in-memory KV store. I wonder if it makes sense to use Memcached on the host and then build out additional functionality: consistent hashing for sharding data and then also replication to help with single node failures
