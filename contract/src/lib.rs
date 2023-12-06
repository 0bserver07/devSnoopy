use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Campaign {
    campaign_id: String,
    campaign_name: String,
    entity_name: String,
    creator_wallet: AccountId,
    campaign_created_at: u64,
    campaign_txn_receipt_id: String,
    launch_date_start: u64,
    end_date: u64,
    location: Option<String>,
    event_type: Option<String>,
    campaign_type: Option<String>,
    campaign_description: Option<String>,
    campaign_image: Option<String>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CampaignUpdate {
    campaign_name: Option<String>,
    entity_name: Option<String>,
    creator_wallet: Option<AccountId>,
    campaign_created_at: Option<u64>,
    campaign_txn_receipt_id: Option<String>,
    launch_date_start: Option<u64>,
    end_date: Option<u64>,
    location: Option<String>,
    event_type: Option<String>,
    campaign_type: Option<String>,
    campaign_description: Option<String>,
    campaign_image: Option<String>
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CampaignManager {
    campaigns: LookupMap<Vec<u8>, Campaign>,
    campaign_ids: Vector<Vec<u8>>,
    campaigns_by_creator: LookupMap<AccountId, Vector<String>>,
    admins: LookupMap<Vec<u8>, bool>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum CampaignUpdateResult {
    Success,
    Error(String),
}





#[near_bindgen]
impl CampaignManager {
    #[init]
    pub fn new(admins_list: Vec<AccountId>) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut admins_map = LookupMap::new(b"admins".to_vec());
        for admin in admins_list {
            admins_map.insert(&admin.as_bytes().to_vec(), &true);
        }

        Self {
            campaigns: LookupMap::new(b"campaigns".to_vec()),
            campaign_ids: Vector::new(b"campaign_ids".to_vec()),
            campaigns_by_creator: LookupMap::new(b"campaigns_by_creator".to_vec()),
            admins: admins_map,
        }
    }


    pub fn add_campaign(&mut self, campaign: Campaign) {
        self.assert_admin_permission();
        let campaign_id_bytes = campaign.campaign_id.clone().into_bytes();
        assert!(
            !self.campaigns.contains_key(&campaign_id_bytes),
            "Campaign ID already exists"
        );

        // Retrieve or create the Vector for the creator's campaigns
        let mut creator_campaigns = self.campaigns_by_creator
            .get(&campaign.creator_wallet)
            .unwrap_or_else(|| Vector::new(b"creator_campaigns".to_vec()));

        // Now you can push to creator_campaigns since it's mutable
        creator_campaigns.push(&campaign.campaign_id);

        // Insert the updated vector back into campaigns_by_creator
        self.campaigns_by_creator.insert(&campaign.creator_wallet, &creator_campaigns);

        // Insert the campaign into the main campaigns map
        self.campaign_ids.push(&campaign_id_bytes);
        self.campaigns.insert(&campaign_id_bytes, &campaign);
    }





    pub fn get_campaign(&self, campaign_id: String) -> Option<Campaign> {
        self.campaigns.get(&campaign_id.into_bytes())
    }

    pub fn get_campaigns_by_creator(&self, creator_wallet: AccountId) -> Vec<Campaign> {
        if let Some(campaign_ids) = self.campaigns_by_creator.get(&creator_wallet) {
            campaign_ids.iter()
                .filter_map(|id| self.campaigns.get(&id.into_bytes()))
                .collect()
        } else {
            Vec::new() // Return an empty vector if no campaigns are found for the creator
        }
    }

    fn assert_creator_or_admin_permission(&self, creator_wallet: &AccountId) {
        let caller = env::signer_account_id();
        assert!(
            self.is_admin(&caller) || caller == *creator_wallet,
            "Permission denied: caller is not admin or creator"
        );
    }



    pub fn list_campaign_ids(&self) -> Vec<String> {
        self.campaign_ids.iter()
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


    pub fn update_campaign(&mut self, campaign_id: String, updates: CampaignUpdate) -> CampaignUpdateResult {
        let campaign_id_bytes = campaign_id.into_bytes();
        if let Some(mut campaign) = self.campaigns.get(&campaign_id_bytes) {
            self.assert_creator_or_admin_permission(&campaign.creator_wallet);

            if let Some(new_name) = updates.campaign_name {
                campaign.campaign_name = new_name;
            }
            if let Some(new_wallet) = updates.creator_wallet {
                campaign.creator_wallet = new_wallet;
            }

            // Direct assignment for u64 fields
            if let Some(new_start) = updates.campaign_created_at {
                campaign.campaign_created_at = new_start;
            }
            if let Some(new_launch_date) = updates.launch_date_start {
                campaign.launch_date_start = new_launch_date;
            }
            if let Some(new_end_date) = updates.end_date {
                campaign.end_date = new_end_date;
            }

            if let Some(new_location) = updates.location {
                campaign.location = Some(new_location);
            }
            if let Some(new_event_type) = updates.event_type {
                campaign.event_type = Some(new_event_type);
            }
            if let Some(new_campaign_type) = updates.campaign_type {
                campaign.campaign_type = Some(new_campaign_type);
            }
            if let Some(new_description) = updates.campaign_description {
                campaign.campaign_description = Some(new_description);
            }
            if let Some(new_image) = updates.campaign_image {
                campaign.campaign_image = Some(new_image);
            }

            // Re-insert the updated campaign
            self.campaigns.insert(&campaign_id_bytes, &campaign);
            CampaignUpdateResult::Success
        } else {
            CampaignUpdateResult::Error("Campaign not found".to_string())
        }
    }


    pub fn delete_campaign(&mut self, campaign_id: String) -> CampaignUpdateResult {
        let campaign_id_bytes = campaign_id.into_bytes();
        match self.campaigns.get(&campaign_id_bytes) {
            Some(existing_campaign) => {
                self.assert_creator_or_admin_permission(&existing_campaign.creator_wallet);
                self.campaigns.remove(&campaign_id_bytes);
                self.remove_campaign_id(&campaign_id_bytes);
                CampaignUpdateResult::Success
            },
            None => CampaignUpdateResult::Error("Campaign not found".to_string()),
        }
    }


    fn remove_campaign_id(&mut self, campaign_id: &[u8]) {
        let filtered_ids: Vec<_> = self.campaign_ids.iter()
            .filter(|id| id != campaign_id)
            .collect();
        self.campaign_ids.clear();
        for id in filtered_ids {
            self.campaign_ids.push(&id);
        }
    }




}

