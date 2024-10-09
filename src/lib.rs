use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;  
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
mod web4;
use web4::{Web4Request, Web4Response};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Pages,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] 
pub struct Contract {
    owner_id: AccountId,
    base_url: String,
    pages: LookupMap<String, String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, base_url: String) -> Self {
        let mut this = Self {
            owner_id,
            base_url,
            pages: LookupMap::new(StorageKey::Pages),
        };
        this.pages.insert(&"".to_string(), &"index.html".to_string());
        this
    }

    pub fn add_page(&mut self, path: String, file_name: String) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can add pages");
        self.pages.insert(&path, &file_name);
    }

    pub fn get_page_url(&self, path: String) -> Option<String> {
        self.pages.get(&path).map(|file_name| format!("{}/{}", self.base_url, file_name))
    }

    pub fn set_base_url(&mut self, new_base_url: String) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can change the base URL");
        self.base_url = new_base_url;
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }


    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path.trim_start_matches('/');

        if path == "robots.txt" {
            return Web4Response::plain_response("User-agent: *\nDisallow:".to_string());
        }
        
        if let Some(file_name) = self.pages.get(&path.to_string()) {
            let full_url = format!("{}/{}", self.base_url, file_name);
            Web4Response::redirect(full_url)
        } else {
            if let Some(default_file) = self.pages.get(&"".to_string()) {
                let full_url = format!("{}/{}", self.base_url, default_file);
                Web4Response::redirect(full_url)
            } else {
                Web4Response::not_found()
            }
        }
    }
}
