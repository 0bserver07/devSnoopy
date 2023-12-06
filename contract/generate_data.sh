#!/bin/bash

CONTRACT_NAME="dev-1698851173459-19456871894282"
YOUR_ACCOUNT_ID="dev-1698851173459-19456871894282"

# Add 10 questions
for i in {1..10}; do
  near call $CONTRACT_NAME add_question '{"content": "Test Post #'${i}'?"}' --account-id $YOUR_ACCOUNT_ID
done

# Metadata the first 5 questions with 3 answers each
for i in {0..4}; do
  near call $CONTRACT_NAME answer_question '{"question_id": '${i}', "content": "Metadata 1 for Post #'${i}'"}' --account-id $YOUR_ACCOUNT_ID
  near call $CONTRACT_NAME answer_question '{"question_id": '${i}', "content": "Metadata 2 for Post #'${i}'"}' --account-id $YOUR_ACCOUNT_ID
  near call $CONTRACT_NAME answer_question '{"question_id": '${i}', "content": "Metadata 3 for Post #'${i}'"}' --account-id $YOUR_ACCOUNT_ID
done
