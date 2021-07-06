# **Dot Marketplace**

- Status: Open
- Proposer: Wow Labz
- Projects you think this work could be useful for:
[https://polkadot.network](https://polkadot.network/)
[https://kusama.network](https://kusama.network/)

### **Overview**
# 📄

Dot Marketplace is a general purpose decentralised marketplace. It can be used by all decentralised projects to float any kind of tasks in a marketplace and invite their community members to execute them for a reward. Its roadmap includes integration with wallets and a decentralised court for dispute resolution.

The inspiration for Dot Marketplace emerged from our own needs while building Yoda - a protocol that supports decentralised product development leveraging open data. Dot Marketplace would be open sourced and we would be using it extensively to create marketplaces on Yoda protocol.

Dot Marketplace is being built on Substrate. It would include boilerplate code that teams can customize as per their own requirements. We believe this project has the potential to transform community participation, engagement and governance in decentralized projects.

The current focus is to enhance the existing Substrate pallet and allied code base to get a basic yet functional Marketplace up and running:

1. Wallet integration - Polkawallet, Metamask
2. Register/ Signup through PolkaJS
3. Conventional interfaces for Web2 users
  1. Conventional Registration/ Signup
  2. Removing token/ blockchain specific language
  3. Settings for enabling transaction notifications. By default they would be disabled
  4.
4. Publish a task
  1. Define the specifics of the task
  2. Lock money in escrow
  3. Publish the task
5. Bid for a task by staking tokens
  1. Lock tokens in escrow
  2. Submit the proof of the task being completed
  3. Integration with centralised storage (for now)
6. Ratings
  1. For the task publisher by worker
  2. For the worker by task publisher

### **Ecosystem Fit**

Yes there are a few similar projects. But, what differentiates us from them is that our project bridges the gap and provides an entire ecosystem of Services, Data and Tools Marketplace. We can confidently say that, there is something for everyone and there is everything for someone.

List of competitors:

1. [https://data.iota.org/#/](https://data.iota.org/#/)
2. [https://www.snowflake.com/data-marketplace/](https://www.snowflake.com/data-marketplace/)
3. [https://datum.org/](https://datum.org/)
4. [https://market.oceanprotocol.com/](https://market.oceanprotocol.com/)
5. [https://streamr.network/](https://streamr.network/)

## **Team**
# 👥

### **Team members**

- Amit Singh
- Roshit Omanakuttan
- Umashankar Das
- Hamad Jowher
- Varun Gyanchandani
- Loakesh Indiran
- Siddharth Teli

### **Team Website**

- [http://www.wowlabz.com](https://www.yoda.to/)

### **Project Website**

- [https://www.yoda.to/](https://www.yoda.to/)

### **Legal Structure**

Wow Labz

[Address](https://www.google.com/search?rlz=1C1CHZN_enIN936IN936&amp;q=wow+labz+address&amp;stick=H4sIAAAAAAAAAOPgE-LVT9c3NEzOKU7PqbA01pLNTrbSz8lPTizJzM-DM6wSU1KKUouLF7EKlOeXK-QkJlUpQIUAMRiJsEMAAAA&amp;ludocid=5336906714757536502&amp;sa=X&amp;ved=2ahUKEwiJkcvjtrLxAhXa6XMBHfEUA_IQ6BMwJHoECD0QBA): 3rd Floor, Fremont Terraces, #3580, 4th Cross Rd, HAL 2nd Stage, Doopanahalli, Indiranagar, Bengaluru, Karnataka 560008

### **Team&#39;s experience**

Dot Marketplace is being built by the team at Wow Labz.
 Wow Labz is one of India&#39;s leading turnkey product development companies. It has built over 100 products for several funded startups and enterprises. It is currently focused on building Blockchain and AI products.

 Previously, at Wow Labz we have built a decentralised storage protocol called Lake Network - [https://lakenetwork.io/](https://lakenetwork.io/)besides multiple dApps on Ethereum, Stellar, EOS and Hyperledger.

A list of centralised apps published can be found [here](https://www.wowlabz.com/work/).

### **Team Code Repos**

- [https://github.com/WowLabz](https://github.com/WowLabz)
- [https://github.com/orgs/WowLabz/projects](https://github.com/orgs/WowLabz/projects)
- [https://github.com/WowLabz/tasking\_backend](https://github.com/WowLabz/tasking_backend)
- [https://github.com/WowLabz/tasking\_frontend](https://github.com/WowLabz/tasking_frontend)

### **Team LinkedIn Profiles (if available)**

- [https://www.linkedin.com/in/startupamit/](https://www.linkedin.com/in/startupamit/) (Amit Singh)
- [https://www.linkedin.com/in/hamadjowher/](https://www.linkedin.com/in/hamadjowher/) (Hamad Jowher)
- [https://www.linkedin.com/in/varunsays/](https://www.linkedin.com/in/varunsays/) (Varun Gyanchandani)
- [https://www.linkedin.com/in/siddharthteli/](https://www.linkedin.com/in/siddharthteli/) (Siddharth Teli)
- [https://www.linkedin.com/in/loakesh-indiran-8a2282140](https://www.linkedin.com/in/loakesh-indiran-8a2282140) (Loakesh Indiran)

## **Development Roadmap**
# 🔩

![dotmarketplacegif](https://user-images.githubusercontent.com/11945179/124598936-c9f01000-de82-11eb-91d5-b2e37f1791df.gif)

Based on our past experience from building Dot Marketplace for the Polkadot Buildathon India (2021) and the expertise we gained while building this PoC Pallet-Tasking Runtime, we think that there are a number of tasks that will contribute to the success of Dot Marketplace.

All the below mentioned Milestones are going to be an RFP response and this application is going to be fully public.

### **Milestone 1** -

We will be building a substrate based services marketplace, where a user gets registered via a registration form and linking their respective wallets, which will be linked to Polkawallet and Metamask.

- Marketplace Initialization
  - User Registration
  - Forms
  - Wallets
- User Dashboard
  - Custom screens
  - Forgot Password
  - Sign Up/Sign In with Google
  - Illustration for home screen.

### **Milestone 2 -**

In continuation to the previous work, we will be working on the rating system over here, which will help the platform provide the distribution of network rewards based on the user&#39;s performance and credible work submitted in the past. This rating will eventually be the motivating factor for performance in the network to be incentivized for quality work. :

- Ratings
  - Customer Rating Workflow
  - Worker Rating workflow

- User Account
  - Profile
  - Rating
  - Earnings

- Settings
  - Switch to customer/worker view
  - Toggle switch for on/off Blockchain Notifications (Events)
  - Logout

### **Milestone 3 -**

- Multi User scenarios
  - Scaling and testing the usability for multiple users.
- UI upgrades and form tags
  - Registration form fields \&lt;tags\&gt;
  - Align workers to tag based track/domain.
- Project Pagination.
- Storage APIs and DB Connection
  - Integrating Azure storage services
  - Mongo connection \&lt;objectId\&gt;
  - Async file Upload/download.

<img src = "https://user-images.githubusercontent.com/11945179/124599088-e8560b80-de82-11eb-8ece-1f9f8e76a235.png" width = 700 height = 500>


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
  - Rust, Substrate, React, Python, MongoDB, Azure Storage, AMQP, Celery
- Documentation of components, protocols, architecture etc.
  - [https://drive.google.com/drive/folders/173Wup7oxr7IywHFpfrhaTdZtNlkZfFL5?usp=sharing](https://drive.google.com/drive/folders/173Wup7oxr7IywHFpfrhaTdZtNlkZfFL5?usp=sharing)
- PoC/MVP
  - [https://youtu.be/xQNOkXQdDnQ](https://youtu.be/xQNOkXQdDnQ) (Dot Marketplace Video)
  - [http://65.2.26.225:8001/](http://65.2.26.225:8001/) (Dot Marketplace)

###
