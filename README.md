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
- The Customer rates the Service Provider and visa versa 

NOTE: If the Customer doesn't accept the work, a dispute is raised and it gets resolved in a decentralised court (out of current scope) which will be implemented in the next phase. 

The following diagrams highlight the workflow:


Customer                   
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125753620-e1b094dc-552e-4a4f-9826-23cbefe3a677.png" widht=350 height=400>

Worker
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125753635-1cc3170e-7a19-410e-a350-93f75a10e93f.png" widht=350 height=400>


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

- Amit Singh (product manager)
- Roshit Omanakuttan (technical architect)
- Varun Gyanchandani (backend lead)
- Loakesh Indiran (full stack dev)
- Siddharth Teli (backend dev)
- Bharath Kumar (tester)
- Smita Ashok (tech content writer)


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
A list of awards won by the organisation can be found [here](https://www.wowlabz.com/awards/).

### **Team Code Repos**

- [https://github.com/orgs/WowLabz/projects](https://github.com/orgs/WowLabz/projects) 
- [https://github.com/WowLabz/tasking\_backend](https://github.com/WowLabz/tasking_backend)
- [https://github.com/WowLabz/tasking\_frontend](https://github.com/WowLabz/tasking_frontend)

### **Team LinkedIn Profiles (if available)**

Profiles of the people working actively on Dot Marketplace
- [Amit Singh](https://www.linkedin.com/in/startupamit/)
- [Roshit Omanakuttan](https://www.linkedin.com/in/roshit/)
- [Varun Gyanchandani](https://www.linkedin.com/in/varunsays/)
- [Siddharth Teli](https://www.linkedin.com/in/siddharthteli/) 
- [Loakesh Indiran](https://www.linkedin.com/in/loakesh-indiran-8a2282140)
- [Bharath Kumar](https://www.linkedin.com/in/bharath-kumar-h-13a572126/)
- [Smita Ashok](https://www.linkedin.com/in/smita-ashok-823631175/)

## **Development Roadmap**🔩

The development of Dot Marketplace is already underway. 
For the custom pallet (tasking) we have: 
1. Used various Substrate provided traits like - `Currency`, `ExistenceRequirement`, `LockIdentifier`, `LockableCurrency`, `ReservableCurrency` and few more;
2. Used the pre-existing pallets like `assets`, `balances` and `staking`;
3. Implemented custom structs like `TaskDetails` and `TransferDetails`. These in return are used for various functionalities like `create_task`, `bid_task`, `complete_task` and `approve_task`. A special transfer money function is only initiated once the task cycle gets completed and the escrow funds are released to the worker. 

All the below mentioned milestones are going to be an RFP response and this application is going to be fully public.
  
NOTE: A barebones UI would also be provided as a part of this submission to help the developer experience the functionality

### **Milestone 1**

Duration: 2 weeks  
FTEs: 1  
PTEs: 4  
Costs: 3,220 USD // rest is funded by Yoda  


The main deliverable for this milestone will be to allow a user to register via a registration form and link her Polkawallet account along with role based switching from Service Provider view to Customer view and visa versa.


| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 0a      | License | Apache 2.0         |
| 0b      | [Documentation](https://github.com/WowLabz/tasking_backend) | We will provide both inline documentation of the code and a tutorial that explains how a user can use DOT Marketplace and understand the flow of tasking pallet.         |
| Oc      | Testing Guide | 	Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests.
| 0d      | Docker Image | Docker image of build with a script to run unit tests | 
| 1      | Registration Module | Form based user registration         |
| 2      | Wallet Linking | Support for user to link their Polkawallet with the account.     |
| 3      | Profile Module | Support for role based screens to ease the usability for users  |


### **Milestone 2**
  
Duration: 3 weeks  
FTEs: 2  
PTEs: 4  
Costs: 7,440 USD // rest is funded by Yoda  
 
In continuation to the previous work, we will be working on a rating system for both Customer and Service Provider. This rating will eventually be the motivating factor for performance in the network to be incentivized for quality work. :


| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 0a      | License | Apache 2.0         |
| 0b      | [Documentation](https://github.com/WowLabz/tasking_backend) | We will provide both inline documentation of the code and a tutorial that explains how a user can use DOT Marketplace and understand the flow of tasking pallet.         |
| Oc      | Testing Guide | 	Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests.
| 0d      | Docker Image | Docker image of build with a script to run unit tests | 
| 1      | Rating Module | Support for profile based rating using substrate balances, treasury and staking pallets to be integrated with our custom tasking pallet to weigh the user's performance and rewards based rating system.          |
| 2      | Programmatic Wallet Transfer | Substrate based Smart Contract transfer function for programmatic/automated transfer of tokens from one application/user to the other.         |
| 3      | Asset Restrictions | Support for the locking of assets by time         |

  
### **Milestone 3**
  
Duration: 3 weeks  
FTEs: 2  
PTEs: 5  
Costs: 8,260 USD // rest is funded by Yoda  

The deliverable for this milestone is that we will be providing a multi user scenario to test the functionality and integrate with storage and backend APIs for multipart data to be uploaded and downloaded.

| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 0a      | License | Apache 2.0         |
| 0b      | [Documentation](https://github.com/WowLabz/tasking_backend) | Documentation of the entire pallet, a guide for developers forking the project including FAQ 
| Oc      | Testing Guide | 	Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests.
| 0d      | Docker Image | Docker image of build with a script to run unit tests | 
| 1      | Multiuser Module | Support for multiple Substrate seed users to test the functionality and make the task based transactions as per the Status mentioned. Substrate based Lockable currency for reserving user funds and allowing the escrow unlock after the approved status.         |
| 2      | Tagging Module | Support for smart tags with the user profiles for programmatic track/domain alignment in the future        |
| 3      | Async Upload Module  | API connections to cloud storage async upload/download of small files via Rocket      |
| 4      | Testing | Repositories including the deployment and test sections for instructions and scripts to help contributors to package, deploy, run and test.       |


### **Summary:**

Team count: 7  
Total duration: 2 months  
Total cost: 18,920 USD  


### **Additional Project Details**

- Technology stack to be used
  - Rust, Substrate, React, Python, centralised cloud storage
- Documentation of workflows, architecture etc.
  - [User Workflows](https://drive.google.com/drive/folders/1tLV5q5iRt7Rz-F89UBKalfQ_C-JzbUe4?usp=sharing)



### **Future Plans** 
Future releases of the Dot Marketplace include:

| Phase        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 2      | Decentralised Court | A fully decentralised dispute resolution mechanism along with configurible rules for slashing and reputation.          |
| 3      | Milestone based submissions | Making provisions to breakdown a project into multiple configurable milestones to allow parallel or sequential execution        |
| 4     | Decentralised Storage | Integration with IPFS or another decentralised storage platform        |



###
