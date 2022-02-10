License: Unlicense

Court Summons:
1. If a customer feels that the task is not upto the mark - Summon the court in provide worker rating

Case A: Worker wins 
Worker gets back their money from escrow 
Worker gets the money for the task 

2. If a worker feels the rating provided is unsatisfactory - Summon the court in provide customer rating
3. If a customer feels the rating provided is unsatisfactory - Sumon the court in a new function

<!-- -> Ratings is provided more importance than tokens because ratings affect other oppotunities as well in the long terms
For increase in rating -> (% above mjority * 0.5) + actual rating received
No deductions in rating for the losing side as of now -->

If a customer gets unjust ratings from the worker:
1. Chances of becoming a juror is affected
2. Chances of geeting workers for future tasks is affected

If a worker gets unjust ratings from the customer:
1. Chances of becoming a juror is affected
2. Chances fo getting picked for future tasks is affected


Points of contention:
1. How to affect the ratings in Court Summons 2&3 - solved
   Take ratings from jurors and average, if rating alredy exists, average that with with juror rating
   Issue -  Floating point for vec u8

2. How to affect/access balances when the whole transaction involves more tokens than what is present in the escrow - solved
   Both sides have to pay court fee, take tokens from escrow only
   

3. How to make sure jurors are domian experts/capable 
   users ave tags already so not to worry about that

Escrow should be released once customer ratings has been provided by the worker

Primary Escrow  - Price  + Bid for task - Released after provide customer ratings
It's okay for both sides to lose money for court fee

<!-- Not necessary
Secondary Escrow  - Court fee from publisher + Court fee from worker - Released after customer accepts provided rating

Single escrow 
Price + Bid money - 100 + 100 units -->

Points of issues:
1. What if there are not enough people in a domain for jurors

Components:

1. Disapprove task extrinsic
Extrnisic for customer to disapprove the task 
Calls the probable juror list helper function

2. Disapprove customer_rating extrinsic 
Extrnisic for customer to disapprove the rating
Calls the probable juror list helper function

3. Disapprove  worker_rating extrinsic 
Extrnisic for worker to disapprove the rating
Calls the probable juror list helper function

4. Probable_juror helper function
Returs a list of all the jurors with rating >4 and matching tags to the task tag excluding the publisher and the worker

5. Accept jury_duty extrinsic 
Extrisic for probable jurors to accept whether they want to judge this particular dispute
updates dispute storage with final list of jurors

6. Voting for dispute extrinsic 
Extrinsic for jurors to cast votes and ratings of worker and publisher
updates dispute storage with necessary details
once all votes are cast, escrow is released and ratings are updated and status changed to closed by court


Edge Cases:
1. In case of too many cases to handle where there are no jurors volunteering from the user pool
Possible Fix: 
a) Have a set of default jurors where their only role on chain is to be a juror 
b) Have a automated master council where we build an algorthm to distribute ratings and funds by taking into account factors like previous task detais, previous dispute details etc.
(For the moment a simple algoritm to give the worker slighlty more advantage in terms of funds and provide equal ratings on both sides)


TODO:
1. Update Storage 
2. Move Court Dispute to Task Details 
3. Resolve Cases
4. Code Coverage - Test Cases
5. Web3 Grat Documentation
6. timeframe storage to be deleted after dispute is over
7. Need to change name timeframes to possibly hearings

Cases:
1. What if it is a tie after final votes 
2. What if no jurors vote after accepting jury duty
3. What if not all accepted jurors vote







