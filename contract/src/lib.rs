use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    event_id: String,
    event_name: String,
    entity_name: String,
    creator_wallet: AccountId,
    event_created_at: u64,
    event_txn_receipt_id: String,
    launch_date_start: u64,
    end_date: u64,
    location: Option<String>,
    event_type: Option<String>,
    campaign_type: Option<String>,
    event_description: Option<String>,
    event_image: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventUpdate {
    event_name: Option<String>,
    entity_name: Option<String>,
    creator_wallet: Option<AccountId>,
    event_created_at: Option<u64>,
    event_txn_receipt_id: Option<String>,
    launch_date_start: Option<u64>,
    end_date: Option<u64>,
    location: Option<String>,
    event_type: Option<String>,
    campaign_type: Option<String>,
    event_description: Option<String>,
    event_image: Option<String>
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EventManager {
    events: LookupMap<Vec<u8>, Event>,
    event_ids: Vector<Vec<u8>>,
    events_by_creator: LookupMap<AccountId, Vector<String>>,
    admins: LookupMap<Vec<u8>, bool>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum EventUpdateResult {
    Success,
    Error(String),
}





#[near_bindgen]
impl EventManager {
    #[init]
    pub fn new(admins_list: Vec<AccountId>) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut admins_map = LookupMap::new(b"admins".to_vec());
        for admin in admins_list {
            admins_map.insert(&admin.as_bytes().to_vec(), &true);
        }

        Self {
            events: LookupMap::new(b"events".to_vec()),
            event_ids: Vector::new(b"event_ids".to_vec()),
            events_by_creator: LookupMap::new(b"events_by_creator".to_vec()),
            admins: admins_map,
        }
    }


    pub fn add_event(&mut self, event: Event) {
        self.assert_admin_permission();
        let event_id_bytes = event.event_id.clone().into_bytes();
        assert!(
            !self.events.contains_key(&event_id_bytes),
            "Event ID already exists"
        );

        // Retrieve or create the Vector for the creator's events
        let mut creator_events = self.events_by_creator
            .get(&event.creator_wallet)
            .unwrap_or_else(|| Vector::new(b"creator_events".to_vec()));

        // Now you can push to creator_events since it's mutable
        creator_events.push(&event.event_id);

        // Insert the updated vector back into events_by_creator
        self.events_by_creator.insert(&event.creator_wallet, &creator_events);

        // Insert the event into the main events map
        self.event_ids.push(&event_id_bytes);
        self.events.insert(&event_id_bytes, &event);
    }





    pub fn get_event(&self, event_id: String) -> Option<Event> {
        self.events.get(&event_id.into_bytes())
    }

    pub fn get_events_by_creator(&self, creator_wallet: AccountId) -> Vec<Event> {
        if let Some(event_ids) = self.events_by_creator.get(&creator_wallet) {
            event_ids.iter()
                .filter_map(|id| self.events.get(&id.into_bytes()))
                .collect()
        } else {
            Vec::new() // Return an empty vector if no events are found for the creator
        }
    }

    fn assert_creator_or_admin_permission(&self, creator_wallet: &AccountId) {
        let caller = env::signer_account_id();
        assert!(
            self.is_admin(&caller) || caller == *creator_wallet,
            "Permission denied: caller is not admin or creator"
        );
    }



    pub fn list_event_ids(&self) -> Vec<String> {
        self.event_ids.iter()
            .map(|bytes| String::from_utf8(bytes).unwrap_or_default())
            .collect()
    }


    fn is_admin(&self, account_id: &AccountId) -> bool {
        self.admins.get(&account_id.as_bytes().to_vec()).is_some()
    }

    fn assert_admin_permission(&self) {
        assert!(
            self.is_admin(&env::signer_account_id()),
            "Admin permission denied"
        );
    }


    pub fn update_event(&mut self, event_id: String, updates: EventUpdate) -> EventUpdateResult {
        let event_id_bytes = event_id.into_bytes();
        if let Some(mut event) = self.events.get(&event_id_bytes) {
            self.assert_creator_or_admin_permission(&event.creator_wallet);

            if let Some(new_name) = updates.event_name {
                event.event_name = new_name;
            }
            if let Some(new_wallet) = updates.creator_wallet {
                event.creator_wallet = new_wallet;
            }

            // Direct assignment for u64 fields
            if let Some(new_start) = updates.event_created_at {
                event.event_created_at = new_start;
            }
            if let Some(new_launch_date) = updates.launch_date_start {
                event.launch_date_start = new_launch_date;
            }
            if let Some(new_end_date) = updates.end_date {
                event.end_date = new_end_date;
            }

            if let Some(new_location) = updates.location {
                event.location = Some(new_location);
            }
            if let Some(new_event_type) = updates.event_type {
                event.event_type = Some(new_event_type);
            }
            if let Some(new_campaign_type) = updates.campaign_type {
                event.event_type = Some(new_campaign_type);
            }
            if let Some(new_description) = updates.event_description {
                event.event_description = Some(new_description);
            }
            if let Some(new_image) = updates.event_image {
                event.event_image = Some(new_image);
            }

            // Re-insert the updated event
            self.events.insert(&event_id_bytes, &event);
            EventUpdateResult::Success
        } else {
            EventUpdateResult::Error("Event not found".to_string())
        }
    }


    pub fn delete_event(&mut self, event_id: String) -> EventUpdateResult {
        let event_id_bytes = event_id.into_bytes();
        match self.events.get(&event_id_bytes) {
            Some(existing_event) => {
                self.assert_creator_or_admin_permission(&existing_event.creator_wallet);
                self.events.remove(&event_id_bytes);
                self.remove_event_id(&event_id_bytes);
                EventUpdateResult::Success
            },
            None => EventUpdateResult::Error("Event not found".to_string()),
        }
    }


    fn remove_event_id(&mut self, event_id: &[u8]) {
        let filtered_ids: Vec<_> = self.event_ids.iter()
            .filter(|id| id != event_id)
            .collect();
        self.event_ids.clear();
        for id in filtered_ids {
            self.event_ids.push(&id);
        }
    }




}

