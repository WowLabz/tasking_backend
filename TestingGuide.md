## Setup ##
First, complete the basic Rust setup instructions.

Use Rust's native cargo command to build and launch the node:

`cargo run --release -- --dev --tmp`
or use make alias

`make run`

Run in Docker
First, install Docker and Docker Compose.

`sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose`

`sudo chmod +x /usr/local/bin/docker-compose`

To check the compose version

`docker-compose --version`

To run the docker-compose build

`sudo docker-compose up --build -d` 

Then run the following command to start the server.

../scripts/docker_run.sh
This command will firstly compile your code, and then start a local development network. You can also replace the default command (cargo build --release && ./target/release/node-template --dev --ws-external) by appending your own. A few useful ones are as follow.

# Run Substrate node without re-compiling
../scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
../scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
../scripts/docker_run.sh cargo check
Automation Testing
The make tests command will launch comprehensive test suite.

To launch only pallet tests run

`cargo test <test-name>`
Manual testing
Launch node

make run
Use this link to open the Polkadot JS Apps UI and automatically configure the UI to connect to the local node.

