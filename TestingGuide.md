## Setup ##
First, complete the basic Rust setup instructions. If you want to play at the code level.

If No, then plz follow this simple guide to make your life easy :)

Run in Docker

# Install Docker-Compose

First, install Docker and Docker Compose. Follow the basic installation guide [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/)

For a Windows Machine

[Follow the guide mentioned here](https://docs.docker.com/desktop/windows/install/)

Installation from the terminal on a Ubuntu/Linux Machine 

`curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose`

`chmod +x /usr/local/bin/docker-compose`

# Docker quick guide

To check the compose version

`docker-compose --version`

To run the build

`docker-compose up --build -d` 

To stop the service

`docker-compose down`

To view the installed images locally

`docker images`

To delete the images

`docker rmi <IMAGE ID>`

Then run the following command to start the server.

To get more idea about the project and the build please refer the link <To be added>

# Run backend node

To setup the Backend Node

Clone the WowLabz [Dot_Marketplace_backend](https://github.com/WowLabz/tasking_backend.git)

Run `docker-compose up --build -d`

In order to check for the status of the running node
Run `docker-compose logs <CONTAINER ID>`

# Build the Authentication Service

To setup the Auth Service

Clone the WowLabz [Dot_Marketplace_Authentication_Service](https://github.com/WowLabz/authentication_service.git)

to initiate the service run the following commands

Run `docker-compose up --build -d`

To check the running container run `docker ps`

# Run UI

To setup the UI

Clone the WowLabz [Dot_Marketplace_Frontend](https://github.com/WowLabz/tasking_frontend.git)

to intitate the docker build for UI

Run `docker-compose up --build -d`

Once the build is complete, access the port `9001` from your localhost

![Screenshot_15](https://user-images.githubusercontent.com/11945179/131972401-6a700ce1-d938-45e2-931d-a50986daac12.png)

# Launch Interactive Bash Session with the Node
    
`docker exec -it <CONTAINER ID> bash` |OR| `docker exec -it <CONTAINER ID> /bin/sh`
    
# Run Tests
    
`cargo test` (This will run all the test cases)
 
 `cargo test <test_name>` (For checking any specific test case)

# Interacting with Polkadot Js Apps Explorer
    
![BlocksFinality](https://user-images.githubusercontent.com/11945179/131971129-d166e10f-5efe-4d1a-8fab-082ba8a13a07.png)
    
To check the working of palletTasking go to Developers -> Extrinsics -> Submit the following Extrinsic -> palletTasking -> createTask (Starting Point, explore further we have worked on the full tasking lifecycle) 

![ChainExtrinsics](https://user-images.githubusercontent.com/11945179/131971070-580769be-7827-429e-8d9b-7216997813ca.png)
    
    
To read about the working of the pallet please refer the guide [Description](https://github.com/WowLabz/tasking_backend/tree/dev#readme)

