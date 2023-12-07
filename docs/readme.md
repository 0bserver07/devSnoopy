
Here are examples of how you can test each function:



### 0. Deploying the Contract

To deploy the contract, run the following command, and also initialize the contract with the `new` function. This will set the initial state of the contract, including the list of admins.


```bash

near call your-contract.testnet new '{"admins_list": ["your-contract.testnet"]}' --accountId your-contract.testnet

```


### 1. Adding a Event

To call the `add_event` function, you'll need to provide a full `Event` object as an argument and also have the necessary permissions.
- `event_id` should be unique and can be any string.
- `creator_wallet` should be the NEAR account ID of the creator.
- `event_txn_receipt_id` should be the transaction ID of the transaction that created the event.
- `event_image` is optional, but it can be a ipfs hash. The
- `event_created_at`, `launch_date_start`, and `end_date` should be Unix timestamps (in seconds).


```bash
near call your-contract.testnet add_event '{
    "event": {
        "event_id": "cam11p1232",
        "event_name": "Save the Forests",
        "entity_name": "GreenEarth",
        "creator_wallet": "creator.testnet",
        "event_created_at": 1633036800,
        "event_txn_receipt_id": "123abc",
        "launch_date_start": 1633123200,
        "end_date": 1635724800,
        "location": "Earth",
        "event_type": "Fundraiser",
        "event_type": "Environment",
        "event_description": "A event to save rainforests.",
        "event_image": "image_url"
    }
}' --accountId your-contract.testnet
```

### 2. Updating a Event

To update a event, use the `update_event` function. The `updates` argument should only include the fields you wish to update.

```bash
near call your-contract.testnet update_event '{
    "event_id": "camp123",
    "updates": {
        "event_name": "Save the Oceans 123"
    }
}' --accountId your-contract.testnet
```

### 3. Getting a Event

To view details of a specific event, use the `get_event` function. This is a view call, so it doesn't require any NEAR tokens.

```bash
near view your-contract.testnet get_event '{"event_id": "camp123"}'

RESULT:

View call: dev-1701841746435-46635735875844.get_event({"event_id": "camp123"})
{
  event_id: 'camp123',
  event_name: 'Save the Oceans 123',
  entity_name: 'GreenEarth',
  creator_wallet: 'creator.testnet',
  event_created_at: 1633036800,
  event_txn_receipt_id: '123abc',
  launch_date_start: 1633123200,
  end_date: 1635724800,
  location: 'Earth',
  event_type: 'Fundraiser',
  event_type: 'Environment',
  event_description: 'A event to save rainforests.',
  event_image: 'image_url'
}
```

### 4. Getting Events by Creator

To list all events created by a specific wallet, use the `get_events_by_creator` function:

```bash
near view your-contract.testnet get_events_by_creator '{"creator_wallet": "creator.testnet"}'


RESULT (after creating 3 events):

[
  {
    event_id: 'camp123',
    event_name: 'Save the Oceans 123',
    entity_name: 'GreenEarth',
    creator_wallet: 'creator.testnet',
    event_created_at: 1633036800,
    event_txn_receipt_id: '123abc',
    launch_date_start: 1633123200,
    end_date: 1635724800,
    location: 'Earth',
    event_type: 'Fundraiser',
    event_type: 'Environment',
    event_description: 'A event to save rainforests.',
    event_image: 'image_url'
  },
  {
    event_id: 'camp1232',
    event_name: 'Save the Forests',
    entity_name: 'GreenEarth',
    creator_wallet: 'creator.testnet',
    event_created_at: 1633036800,
    event_txn_receipt_id: '123abc',
    launch_date_start: 1633123200,
    end_date: 1635724800,
    location: 'Earth',
    event_type: 'Fundraiser',
    event_type: 'Environment',
    event_description: 'A event to save rainforests.',
    event_image: 'image_url'
  },
  {
    event_id: 'cam11p1232',
    event_name: 'Save the Forests',
    entity_name: 'GreenEarth',
    creator_wallet: 'creator.testnet',
    event_created_at: 1633036800,
    event_txn_receipt_id: '123abc',
    launch_date_start: 1633123200,
    end_date: 1635724800,
    location: 'Earth',
    event_type: 'Fundraiser',
    event_type: 'Environment',
    event_description: 'A event to save rainforests.',
    event_image: 'image_url'
  }
]


```

### 5. Listing All Event IDs

To list all event IDs stored in the contract:

```bash
near view your-contract.testnet list_event_ids '{}'


```

### 6. Deleting a Event

To delete a event, assuming the caller has the necessary permissions:

```bash
near call your-contract.testnet delete_event '{"event_id": "camp123"}' --accountId your-account.testnet
```

### Notes:

- Replace `your-contract.testnet` and `your-account.testnet` with the actual account IDs.
- The dates (`event_created_at`, `launch_date_start`, `end_date`) should be Unix timestamps (in seconds).
- Ensure that the caller (`--accountId`) has the necessary permissions, especially for `add_event`, `update_event`, and `delete_event`.
- These commands are for interacting with the contract via NEAR CLI. The exact structure might vary based on how your contract is deployed and the specific requirements of your functions.