# Market Place

# **Dot Marketplace**

- Status: Open
- Proposer: Wow Labz
- Projects you think this work could be useful for: [Polkadot](https://polkadot.network/), [Kusama](https://kusama.network/), [Moonbeam](https://moonbeam.network/) and all Polkadot parachains/ parathreads 

### **Overview** 📄

Dot Marketplace is a general purpose decentralised marketplace created as a Substrate pallet.  

The current scope of work involves two user types: **Customer** and **Service Provider (or Worker)**

- The Customer can post a task and invite bids from Service Providers to fulfill it. 
- The Customer needs to deposit the budgeted amount in an escrow for the task to be published. 
- The Service Provider needs to deposit some token to participate in a bid. If not shortlisted, this bid amount is returned. 
- The Service Provider completes the task and submits it. 
- The Customer accepts the work and the escrowed amount is credited to the Service Providers wallet.
- The Customer rates the Service Provider and visa versa.

NOTE: If the Customer doesn't accept the work, a dispute is raised and it gets resolved in a decentralised court (out of current scope) which will be implemented in the next phase. 

The following diagrams highlight the workflow:


Customer                   
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125753620-e1b094dc-552e-4a4f-9826-23cbefe3a677.png" widht=600>

Worker
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125753635-1cc3170e-7a19-410e-a350-93f75a10e93f.png" widht=600>


Dot Marketplace is being built as a Substrate pallet. It would include boilerplate code that parachain teams can customize as per their own requirements. We believe this project has the potential to transform community participation, engagement and governance in decentralized projects.


### **Repository Hierarchy**
```
├── Dot Marketplace Network Node [link](https://github.com/WowLabz/tasking_backend)
│   ├── ./node ["Chainspecs for Node"]
│   ├── ./scripts [Packaging & Deployment Scripts]
│   ├── ./pallets/pallet-tasking [Pallets]
│   │	    └── ./pallet-tasking 
│   │    	        └── ./src/lib.rs [Tasking Pallet (being implemented)]
│   └── ./runtime [Runtime Module]
│    	    └── Included custom Tasking Pallet

```

The current focus is to enhance the existing Substrate pallet and allied code base to get a basic yet functional marketplace up and running:


### **Ecosystem Fit**

Dot Marketplace can be used by any decentralised project to float tasks and invite their community members to execute them for a reward. Its MVP was developed during the Polkadot India buildathon (2021).  

The inspiration for Dot Marketplace emerged from our own needs while building Yoda - a protocol that facilitates decentralised app development leveraging open data. Dot Marketplace would be used to create data, services and app marketplaces on Yoda, which would motivate us to maintain this project in the long run. 

![dotmarketplacegif](https://user-images.githubusercontent.com/11945179/124598936-c9f01000-de82-11eb-91d5-b2e37f1791df.gif)

### **List of Competitors**

Any product or services marketplace would qualify, here are some examples from outside the Polkadot ecosystem. 
1. [Human Protocol](https://data.iota.org/#/)
2. [Effect Network](https://www.snowflake.com/data-marketplace/)
3. [Ocean Protocol Market](https://market.oceanprotocol.com/)


## **Team** 👥

### **Team members**

* Amit Singh (product manager)
* Roshit Omanakuttan (technical architect)
* Varun Gyanchandani (backend lead)
* Loakesh Indiran (full stack dev)
* Siddharth Teli (backend dev)
* Ritiek Malhotra (backend dev)
* Bharath Kumar (tester)


### **Team Website**

- [http://www.wowlabz.com](https://www.wowlabz.com/) 

### **Project Website**
- Dot marketplace website is under construction

### **Legal Structure** 
- Indian, Private Limited Company 

Wow Labz

[Address](https://g.page/2gethr-ORR): Wow Labz, 2Gethr Cowork, Tower B, Mantri Commercio, Outer Ring Rd, near Sakra World Hospital, Kariyammana Agrahara, Bellandur, Bengaluru, Karnataka 560103

### **Team&#39;s experience**

Dot Marketplace is being built by the team at Wow Labz.
Wow Labz is one of India&#39;s leading turnkey product development companies.
Yoda Protocol has been conceptualised and is being built by the team at Wow Labz. The team has previously built a decentralised storage protocol called Lake Network - [https://lakenetwork.io/](https://lakenetwork.io/) in addition to multiple dApps on Ethereum, Stellar, EOS and Hyperledger.

A list of centralised apps published can be found [here](https://www.wowlabz.com/work/).


### **Team Code Repos**

* [https://github.com/orgs/WowLabz/projects](https://github.com/orgs/WowLabz/projects) 
* [https://github.com/WowLabz/tasking\_backend](https://github.com/WowLabz/tasking_backend)
* [https://github.com/WowLabz/tasking\_frontend](https://github.com/WowLabz/tasking_frontend)

### **Team LinkedIn Profiles (if available)**

Profiles of the people working actively on Dot Marketplace
* [Amit Singh](https://www.linkedin.com/in/startupamit/)
* [Roshit Omanakuttan](https://www.linkedin.com/in/roshit/)
* [Varun Gyanchandani](https://www.linkedin.com/in/varunsays/)
* [Loakesh Indiran](https://www.linkedin.com/in/loakesh-indiran-8a2282140)
* [Siddharth Teli](https://www.linkedin.com/in/siddharthteli/)
* [Ritiek Malhotra](https://www.linkedin.com/in/ritiek/)
* [Bharath Kumar](https://www.linkedin.com/in/bharath-kumar-h-13a572126/)

## **Development Roadmap**🔩

The development of Dot Marketplace is already underway. 
For the custom pallet (tasking) we have: 
1. Used various Substrate provided traits like - `Currency`, `ExistenceRequirement`, `LockIdentifier`, `LockableCurrency`, `ReservableCurrency` and few more;
2. Used the pre-existing pallets like `assets`, `balances` and `staking`;
3. Implemented custom structs like `TaskDetails` and `TransferDetails`. These in return are used for various functionalities like `create_task`, `bid_task`, `complete_task` and `approve_task`. A special transfer money function is only initiated once the task cycle gets completed and the escrow funds are released to the worker. 



### **Future Plans** 
Future releases of the Dot Marketplace include:

| Phase        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 2      | Decentralised Court | A fully decentralised dispute resolution mechanism along with configurible rules for slashing and reputation.          |
| 3      | Milestone based submissions | Making provisions to breakdown a project into multiple configurable milestones to allow parallel or sequential execution        |
| 4     | Decentralised Storage | Integration with IPFS or another decentralised storage platform        |



###


# Substrate Node Template

A fresh FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket:

## Getting Started

Follow these steps to get started with the Node Template :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

-   [`chain_spec.rs`](./node/src/chain_spec.rs): A
    [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
    source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
    are useful for development and testing, and critical when architecting the launch of a
    production chain. Take note of the `development_config` and `testnet_genesis` functions, which
    are used to define the genesis state for the local development chain configuration. These
    functions identify some
    [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
    and use them to configure the blockchain's initial state.
-   [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
    the libraries that this file imports and the names of the functions it invokes. In particular,
    there are references to consensus-related topics, such as the
    [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
    the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
    mechanism and the
    [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
    gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the terms
"[runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime)" and
"[state transition function](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

-   This file configures several pallets to include in the runtime. Each pallet configuration is
    defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
-   The pallets are composed into a single runtime by way of the
    [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
    macro, which is part of the core
    [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
    library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

-   Storage: FRAME defines a rich set of powerful
    [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes
    it easy to use Substrate's efficient key-value database to manage the evolving state of a
    blockchain.
-   Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
    from outside of the runtime in order to update its state.
-   Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to
    notify users of important changes in the runtime.
-   Errors: When a dispatchable fails, it returns an error.
-   Config: The `Config` configuration interface is used to define the types and parameters upon
    which a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
