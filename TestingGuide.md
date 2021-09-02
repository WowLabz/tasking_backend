## Setup ##
First, complete the basic Rust setup instructions.

Use Rust's native cargo command to build and launch the node:

`cargo build --release`

`./target/release/node-template \
    --base-path /tmp/node \
    --port 30333 \
    --ws-port 9944 \
    --rpc-port 9933 \
    --rpc-cors all \
    --validator \
    --ws-external \
    --rpc-external \
    --rpc-methods=Unsafe \
    --prometheus-external \
    --name pallet_tasking_backend \
    --dev \`

Run in Docker

# Install Docker-Compose

First, install Docker and Docker Compose.
For a Ubuntu/Linux Machine

`sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose`

`sudo chmod +x /usr/local/bin/docker-compose`

To check the compose version

`docker-compose --version`

To run the docker-compose build

`sudo docker-compose up -d` Or to rebuild any image `sudo docker-compose up --build -d` 

To stop the service

`sudo docker-compose down`

Then run the following command to start the server.

# Run UI

To setup the UI

Clone the WowLabz [Dot_Marketplace_Frontend](https://github.com/WowLabz/tasking_frontend.git)

to intitate the docker build for UI

Run `sudo docker-compose up -d`

Once the build is complete, access the port `9001` from your localhost

# Build the Authentication Service

To setup the Auth Service

Clone the WowLabz [Dot_Marketplace_Authentication_Service](https://github.com/WowLabz/authentication_service.git)

to initiate the service run the following commands

Run `sudo docker-compose up -d`

To check the running container run `sudo docker ps`

# Run Substrate node without re-compiling



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

