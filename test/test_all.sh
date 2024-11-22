#!/bin/env bash 

export project=$(realpath $(printf $(dirname $(realpath $0))))
echo $project

echo Creating Multisig... && 
node $project/CreateMultiSig.js && 
echo
echo Initilizing Action... &&
node $project/InitMultiSigAction.js &&
echo
echo Vote from key 1... &&
node $project/VoteMultiSigAction.js &&
echo
echo Voting from key 2... &&
node $project/VoteMultiSigAction1.js &&
echo
echo Executing the action... &&
node $project/ExecuteMultiSigAction.js
