
/*

There's need to build some things.

------------------------- Block -------------------------

This is made of different aspects:
- Header;
- Transactions (Extrinsics);
- Challenge List;
- Challenge Solution Proposal.

Header and Transactions are already provided.

Challenge list must be just a list of integers which are representing the values of the identifiers of the challenges.

Challenge Solution Proposal must contain the list of the solution by the validators. At least one of them must be already solved.
Each validator must write in its own spot. It is made of a SolutionList

The SolutionList is a vector/list/map which is written according to the increasing IDs of the challenges.
- ID of the challenge
- Solution (depending on the challenge is defined in a different way)

The Solution is made as follows:
- Effective solution, encrypted with its private key and the public key of the retriever.
- Hash of Header + Transactions + ChallengeList + ProposedSolution
- Timestamp of the solution

In order to simplify the model, we can assume thet only a specific node can propose a challenge. The model can define a Transaction for publishing it. The nodes will consider
it as an active challenge as soon as the transaction containing it will be finalized.

In order to model the status of the challenge, the same proposer can modify it by proposing a transaction which makes it so. It's a bit inaccurate,
but it simplifies the management of the challenge.

######################## Verification #######################

Once the block has been proposed, in order to be verified the need is to check how many nodes signed the block and then check the solution part.
The proposer looks at the solutions and for each one of them does the following:

1. Decrypt the message with its privte key;
2. Decrypt the message with the public key of the proposer;
3. Get the hash of the message and the solution and compare it with the one stored;
4. Check for the timestamp consistency;
5. If everything is fine then the solutions are stored and the block can be considered valid.

------------------------- Challenge -------------------------

TODO: define the structure.

------------------------- Transactions -------------------------

The transactions are the usual ones, plus some more that we need to use to handle the logic of the challenges.


------------------------- Rewards -------------------------

The reward management system is not something we want to put in place right now but in a proper blockchain this should be used.

------------------------- Nodes choice -------------------------

The choice of the nodes can be done by keeping in consideration their GF. This can be defined as a simple random number going from 1 to 5 and then the choice of
the nodes having the highest GF possible will be able to be chosen as proposers. If that is not enough, the nodes will be chosen randomly in the lower values, giving
precedence to higher values among the ones elected.

In some test cases we can provide two nodes with GF 5 and one with GF 3 and see the behavior of the network.

_________________________ Imlementation steps _____________________________________

The steps to follow are these:

1. Block modification
2. Block verification
3. Challenge introduction
4. Transaction modification
5. Challenge update
(6. Nodes choice)
(7. Rewards system)

 */