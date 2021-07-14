# **Dot Marketplace**

- Status: Open
- Proposer: Wow Labz
- Projects you think this work could be useful for: 
[Polkadot](https://polkadot.network/)
[Kusama](https://kusama.network/)
[Moonbeam](https://moonbeam.network/)
and all Polkadot parachains/ parathreads 

### **Overview** 📄

Dot Marketplace is a general purpose decentralised marketplace. 
Here is a user workflow diagram linked to the pallet tasking of the same. 

Customer                   
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125618145-b1be2302-12d7-48e5-bfef-36e6b47bfeb2.png" widht=350 height=400>

Worker
:-------------------------:
<img src = "https://user-images.githubusercontent.com/11945179/125619653-f2255fd5-7c2a-4d55-b65c-47d929a58c10.png" widht=350 height=400>


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

The current focus is to enhance the existing Substrate pallet and allied code base to get a basic yet functional Marketplace up and running:


### **Ecosystem Fit**

Dot Marketplace can be used by any decentralised project to float tasks and invite their community members to execute them for a reward. Its MVP was developed during the Polkadot India buildathon (2021).  

The inspiration for Dot Marketplace emerged from our own needs while building Yoda - a protocol that facilitates decentralised app development leveraging open data. Dot Marketplace would be used to create data, services and app marketplaces on Yoda, which would motivate us to maintain this project in the long run. 

![dotmarketplacegif](https://user-images.githubusercontent.com/11945179/124598936-c9f01000-de82-11eb-91d5-b2e37f1791df.gif)

List of competitors:

Any product or services marketplace would qualify, here are some examples from outside the Polkadot ecosystem. 
1. [Human Protocol](https://data.iota.org/#/)
2. [Effect Network](https://www.snowflake.com/data-marketplace/)
3. [Ocean Protocol Market](https://market.oceanprotocol.com/)


## **Team** 👥

### **Team members**

- Amit Singh
- Varun Gyanchandani
- Loakesh Indiran
- Siddharth Teli

### **Team Website**

- [http://www.wowlabz.com](https://www.yoda.to/) // Needs to be converted into a Notion Page

### **Project Website**
- Dot marketplace web
- [https://www.yoda.to/](https://www.yoda.to/) // We can start with a quick Notion Page 

### **Legal Structure** <Needs to be updated>

Wow Labz

[Address](https://g.page/2gethr-ORR): Wow Labz, 2Gethr Cowork, Tower B, Mantri Commercio, Outer Ring Rd, near Sakra World Hospital, Kariyammana Agrahara, Bellandur, Bengaluru, Karnataka 560103

### **Team&#39;s experience**

Dot Marketplace is being built by the team at Wow Labz.
Wow Labz is one of India&#39;s leading turnkey product development companies.
Wow Labz is the parent company behind Yoda Protocol. The team has previously built a decentralised storage protocol called Lake Network - [https://lakenetwork.io/](https://lakenetwork.io/) besides multiple dApps on Ethereum, Stellar, EOS and Hyperledger.

A list of centralised apps published can be found [here](https://www.wowlabz.com/work/).
A list of awards won by the organisation can be found [here](https://www.wowlabz.com/awards/).

### **Team Code Repos**

- [https://github.com/WowLabz](https://github.com/WowLabz) // Do we need this? 
- [https://github.com/orgs/WowLabz/projects](https://github.com/orgs/WowLabz/projects) // Create a sub folder Blockchain and put the tasking pallet there
- [https://github.com/WowLabz/tasking\_backend](https://github.com/WowLabz/tasking_backend)
- [https://github.com/WowLabz/tasking\_frontend](https://github.com/WowLabz/tasking_frontend)

### **Team LinkedIn Profiles (if available)**

Profiles of the people working actively on Dot Marketplace
- [https://www.linkedin.com/in/startupamit/](https://www.linkedin.com/in/startupamit/) (Amit Singh)
- [https://www.linkedin.com/in/varunsays/](https://www.linkedin.com/in/varunsays/) (Varun Gyanchandani)
- [https://www.linkedin.com/in/siddharthteli/](https://www.linkedin.com/in/siddharthteli/) (Siddharth Teli) 
- [https://www.linkedin.com/in/loakesh-indiran-8a2282140](https://www.linkedin.com/in/loakesh-indiran-8a2282140) (Loakesh Indiran)

## **Development Roadmap**🔩

Based on our past experience from building Dot Marketplace for the Polkadot Buildathon India (2021) and the expertise we gained while building this PoC Pallet-Tasking Runtime, we think that there are a number of tasks that will contribute to the success of Dot Marketplace.

For our custom pallet (tasking) we used various substrate provided traits like - Currency, ExistenceRequirement, LockIdentifier, LockableCurrency, ReservableCurrency and few more. Used the pre-existing pallets like assets, balances and staking. Implemented our custom struct like Task Details, Transfer Details. These in return are used for various functionalities like publish task, bid for task, complete task, approve task. A special transfer money function is only initiated once the task cycle gets completed and the escrow funds are released to the worker. 

All the below mentioned Milestones are going to be an RFP response and this application is going to be fully public.

### **Milestone 1** -

The main deliverable for this milestone is that we will be building a substrate based services marketplace, where a user gets registered via a registration form and linking their respective wallets, which will be linked to Polkawallet.


| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 1      | [Documentation](https://github.com/WowLabz/tasking_backend) | We will provide both inline documentation of the code and a tutorial that explains how a user can use DOT Marketplace and understand the flow of tasking pallet.         |
| 2      | User Registeration | Form based user registeration and linking their respective wallets.         |
| 3      | Wallet Linking | Support for most Substrate/Polkadot based wallet applications. Smart Contract transfer function allows for the directly wallet-signed transfer of assets from one application/user address to the other.         |
| 4      | Profile based Screens | Support for role based screens to ease the usability for users  |

### **Milestone 2 -**

In continuation to the previous work, we will be working on the rating system over here, which will help the platform provide the distribution of network rewards based on the user&#39;s performance and credible work submitted in the past. This rating will eventually be the motivating factor for performance in the network to be incentivized for quality work. :


| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 1      | User Rating Workflows | Support for profile based rating using substrate balances, treasury and staking pallets to be integrated with our custom tasking pallet to weigh the user's performance and rewards based rating system.          |
| 2      | Programmatic Wallet Transfer | Substrate based Smart Contract transfer function allows for the programmatic/automated transfer of tokens from one application/user via smart contract to the other.         |
| 3      | Asset Restrictions | Support for the locking of assets by time or by issuer permission, support for expirations and potentially invalidations.         |
| 4      | Settings View | UI enhancements for event notifications to be customized as per the user profile.         |

### **Milestone 3 -**
The deliverable for this milestone is that we will be providing a multi user scenario to test the functionality and integrate with storage and backend APIs for multipart data to be uploaded and downloaded.

| Number        | Deliverable   | Specification  |
| :-------------|:-------------:| :--------------|
| 1      | Scalability | Support for multiple Substrate seed users to test the functionality and make the task based transactions as per the Status mentioned. Substrate based Lockable currency for reserving user funds and allowing the escrow unlock after the approved status.         |
| 2      | Profile Tagging | Support for smart tags with the user profiles for programmatic track/domain alignment         |
| 3      | Storage APIs  | API connections to cloud storage and backend database for async upload/download of multipart data using actix web         |
| 4      | Testing | Repositories including the deployment and test sections for instructions and scripts to help contributors to package, deploy, run, test.        |

### **Architecture Overview:**

<img src = "https://user-images.githubusercontent.com/11945179/124599088-e8560b80-de82-11eb-8ece-1f9f8e76a235.png" width = 1000 height = 600>


### **Development team:**

1 backend developer

1 frontend developer

1 Designer

1 Devops + Technical Writer

Total man-hours: 488

Total project length: 3 months

Every milestone will be documented and dockerized.

### **Additional Project Details**

- Mockups/designs
  - [http://yoda.to/](http://yoda.to/) (Yoda UI)
- Technology stack to be used
  - Rust, Substrate, React, Python, MongoDB, Azure Storage, AMQP, Celery, Actix web
- Documentation of components, protocols, architecture etc.
  - [https://drive.google.com/drive/folders/173Wup7oxr7IywHFpfrhaTdZtNlkZfFL5?usp=sharing](https://drive.google.com/drive/folders/173Wup7oxr7IywHFpfrhaTdZtNlkZfFL5?usp=sharing)
- PoC/MVP
  - [https://youtu.be/xQNOkXQdDnQ](https://youtu.be/xQNOkXQdDnQ) (Dot Marketplace Video)
  - [http://65.2.26.225:8001/](http://65.2.26.225:8001/) (Dot Marketplace)


### **Future Plans** 
Future releases of the Dot Marketplace include:

| Version 2 | Project Milestones |  |  |
| --- | --- | --- | --- |
||| Only 3 milestones ||
|||| Tasks view for milestones |
||| % Completion ||
||| Approval per milestone ||
||| Payment per milestone ||
|| Chat |||
||| Built with a 3rd party tool ||
|||||
| Version 3 | Contest Submissions |||
|| Decentralised Court |||
||| Conflict Management ||
|||||
| Version 4 | Integration with EPNS (Ethereum Push Notification Service) |||
||| Decentralised Chat using Push Notification Services ||
|||||
|| Integration with IPFS |||
||| Decentralised file storage ||
|||||

###
