# rustcached
A Rust Client for Memcached that expands capabilities of Memcached to a distributed system that provides fault tolerance and sharding.

## Building, Deploying, and Running Rustcached in a Container
The rustcached server can be packaged into a Docker image. To build and run the Docker image, run the following steps:

1. Install Docker if not installed: https://www.docker.com/
1. Build container image: `docker build -t rustcached_container .`
1. Run container instance: `docker run -p <your host port>:7878 rustcached_container`. The server runs on port 7878. You can map the container port to the port of your choosing on your own host.
1. To talk to the server, you can compile the client using `cargo build --bin client --release` and run `cargo run --bin client`. Alternatively you can build your own client. The server is listening over TCP on the port you map it to in the previous step.