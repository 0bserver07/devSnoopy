
Here are examples of how you can test each function:


```

near call your-contract.testnet new '{"admins_list": ["your-contract.testnet"]}' --accountId your-contract.testnet

```


### 1. Adding a Campaign

To call the `add_campaign` function, you'll need to provide a full `Campaign` object as an argument.

```bash
near call your-contract.testnet add_campaign '{
    "campaign": {
        "campaign_id": "cam11p1232",
        "campaign_name": "Save the Forests",
        "entity_name": "GreenEarth",
        "creator_wallet": "creator.testnet",
        "campaign_created_at": 1633036800,
        "campaign_txn_receipt_id": "123abc",
        "launch_date_start": 1633123200,
        "end_date": 1635724800,
        "location": "Earth",
        "event_type": "Fundraiser",
        "campaign_type": "Environment",
        "campaign_description": "A campaign to save rainforests.",
        "campaign_image": "image_url"
    }
}' --accountId your-contract.testnet
```

### 2. Updating a Campaign

To update a campaign, use the `update_campaign` function. The `updates` argument should only include the fields you wish to update.

```bash
near call your-contract.testnet update_campaign '{
    "campaign_id": "camp123",
    "updates": {
        "campaign_name": "Save the Oceans 123"
    }
}' --accountId your-contract.testnet
```

### 3. Getting a Campaign

To view details of a specific campaign, use the `get_campaign` function. This is a view call, so it doesn't require any NEAR tokens.

```bash
near view your-contract.testnet get_campaign '{"campaign_id": "camp123"}'
```

### 4. Getting Campaigns by Creator

To list all campaigns created by a specific wallet, use the `get_campaigns_by_creator` function:

```bash
near view your-contract.testnet get_campaigns_by_creator '{"creator_wallet": "creator.testnet"}'
```

### 5. Listing All Campaign IDs

To list all campaign IDs stored in the contract:

```bash
near view your-contract.testnet list_campaign_ids '{}'
```

### 6. Deleting a Campaign

To delete a campaign, assuming the caller has the necessary permissions:

```bash
near call your-contract.testnet delete_campaign '{"campaign_id": "camp123"}' --accountId your-account.testnet
```

### Notes:

- Replace `your-contract.testnet` and `your-account.testnet` with the actual account IDs.
- The dates (`campaign_created_at`, `launch_date_start`, `end_date`) should be Unix timestamps (in seconds).
- Ensure that the caller (`--accountId`) has the necessary permissions, especially for `add_campaign`, `update_campaign`, and `delete_campaign`.
- These commands are for interacting with the contract via NEAR CLI. The exact structure might vary based on how your contract is deployed and the specific requirements of your functions.