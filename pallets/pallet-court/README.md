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
1. Move Court Dispute to Task Details - Done
2. Update Storage  
   1.a hearing storage to be deleted after dispute is over - Done
   1.b task autocompletion storage to be deleted - Done
3. Resolve Cases - Done
4. Cast vote elements as  helper function - Make parts of the cast vote extrinsic to helper function - Done
5. Code Coverage - Test Cases
6. Web3 Grant Documentation
7. Need to change name timeframes to hearings - Done
8. Create events for the updates on the court cases - Done

Cases:
1. What if it is a tie after final votes - Done - Need to be resolved 
Possible fix - Super User casts the deciding vote 
2. What if no jurors vote after accepting jury duty - Done 
Fix - Go to next set of probable users (Create accounts from polkadot js extension)
3. What if not all accepted jurors vote - Done
(even cases - super user, odd cases-verdict)


Changes:
1. Instead of automatic release of funds, create an extrinsic that is to be called by the customer to be satisfied with the rating provided to him and close the  task - Done
2. Create Register Dispute extrinsic for court ot be summoned at any stage by anyone - Done
 ( Do we need to check if the worker is the bidder and the customer is the publisher while calling the extrinsic) 
3. Resolve Cases 

// Have alice by defaut as super user who will ideally be a council member

In the case of no one votes, do we need to calculate potential jurors again inclusive of people who accepted duty last time
Pass entire court adjourned in a if condition based on final juror length
(Once Super user is added change maximum number of court initations to 3)

----- EVENT NOT BEONG DEPOSITED when all other functions are working in function adjour court, last else block

Final:
1. Check the entire flow - Done
2. Refactor collect cases [DRY code] - Done
3. Add remaining ensures in raise dispute extrinsic - Done
4. Add unit test cases


