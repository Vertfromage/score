use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::UnorderedSet;
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Score{
    high_scores: UnorderedMap<AccountId, u128>, //Map of high scores to users
    leader_board: UnorderedMap<AccountId, u128>,// Leaderboard with scores and users
    users_to_submit: UnorderedSet<AccountId>, // List of users than want their highscore added
    size_of_leaderboard : u8 
}

impl Default for Score{
    fn default() -> Self {
        Self {
            high_scores: UnorderedMap::new(b"a"),
            leader_board: UnorderedMap::new(b"b"),
            users_to_submit: UnorderedSet::new(b"c"),
            size_of_leaderboard : 10
        }
    }
}

#[near_bindgen]
impl Score{
    #[private]
    pub fn set_size_of_leaderboard(&mut self, size:u8 ){
        self.size_of_leaderboard = size;
    }

    pub fn get_size_of_leaderboard(&self) -> u8{
        self.size_of_leaderboard
    }
    // Should only be called by contract from backend
   #[private]
   pub fn insert_score(&mut self, account_id: AccountId, value: u128) -> String {
       // Should only insert if score is higher than previous score
       if self.high_scores.get(&account_id) >= Some(value) {
            return "No increase in score.".to_string();
       }

       self.high_scores.insert(&account_id, &value);
       return "Highscore set!".to_string();
   }

   #[private]
   pub fn insert_leaderboard(&mut self, account_id: AccountId, value: u128) {
        // until list up to size add score
        if self.leader_board.len() < self.size_of_leaderboard.into() {
            self.leader_board.insert(&account_id, &value);
        }else{
        let mut smallest = value;
        let mut key : AccountId = account_id.clone();
        // find smallest score in array
        for(k,v) in &self.leader_board{
            if v < smallest {
                smallest = v;
                key = k;
            }
        }
        if smallest < value {
            self.leader_board.remove(&key);
            self.leader_board.insert(&account_id, &value);
        }
    }
    }

   
    // view leaderboard
    pub fn get_leaderboard(&self) -> Vec<(AccountId, u128)> {
        return self.leader_board.to_vec();
    }

    // view user's high score
    pub fn get_score(&self, account_id: AccountId) -> Option<u128> {
        self.high_scores.get(&account_id)
    }

   // check to make sure owns a mr.brown - could do this on front end... and backend.
   // save them the deposit.. otherwise it's a cross contract call :(
   #[payable] // requires a tiny fee to have score submitted
   pub fn add_self_to_submit(&mut self) -> String{
        // Not sure how much gas... need to adjust
        assert_eq!(env::attached_deposit(),10_u128.pow(24)/100,
        "To add a score must use at least 0.01 Near to cover backend gas fees");

        // add user to list to have score added
        let account_id = env::signer_account_id();
        // should only submit once...
        self.users_to_submit.insert(&account_id);
        // This assures the user that the score will be updated
        return "The backend will upload your score to NEAR shortly.".to_string();
   }

    // How many users waiting to submit... only need to call clear_users_to_submit if it's greater than 0
    pub fn users_waiting_to_submit(&self) -> bool {
        self.users_to_submit.len() > 0
    }
   
   // clears and returns all users waiting to have scores submitted
   // could have bug if someones adding name at same time users_to_submit is getting cleared,
   // might need to clear by names assuming it's unlikely someone would submit repeatedly
   #[private]
   pub fn clear_users_to_submit(&mut self) -> Vec<AccountId> {
        let users : Vec<AccountId> = self.users_to_submit.to_vec();
        self.users_to_submit.clear();
        return users;
   }

    /** IN CASE OF CHEATING */

   #[private] // For use when someone has cheated
   pub fn reset_user_score(&mut self, account_id: AccountId) -> String {
       // Should only insert if score is higher than previous score
       self.high_scores.insert(&account_id, &0);

       return "User score reset to zero".to_string();
   }

   // For use when someone has cheated - if using this also find and add next highest score
   #[private] 
   pub fn remove_leaderboard(&mut self, account_id: AccountId) -> String{
        self.leader_board.remove(&account_id);
        return "User removed from the leaderboard".to_string();
    }

    // For use when resetting the leaderboard incase of major updates.
    #[private] 
   pub fn empty_leaderboard(&mut self) -> String {
        self.leader_board.clear();
        assert_eq!(self.leader_board.len(), 0);
        return "Leaderboard emptied!".to_string();
    }
}

/** TESTS */ 
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    use super::*;

    // Allows for modifying the environment of the mocked blockchain
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn set_get_score() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = Score::default();
        // first score added
        assert_eq!("Highscore set!", contract.insert_score(accounts(1), 100));
        // same size score added
        assert_eq!("No increase in score.", contract.insert_score(accounts(1), 100));
        assert_eq!(Some(100), contract.get_score(accounts(1)));
        // increase in score with existing score
        assert_eq!("Highscore set!", contract.insert_score(accounts(1), 200));
        assert_eq!(Some(200), contract.get_score(accounts(1)));

        // reset user score
        contract.reset_user_score(accounts(1));
        assert_eq!(Some(0), contract.get_score(accounts(1)));
    }

    #[test]
    fn get_nonexistent_score() {
        let contract = Score::default();
        assert_eq!(None, contract.get_score("francis.near".parse().unwrap()));
    }

    #[test]
    fn set_get_leaderboard() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .build());

        let mut contract = Score::default();

        // set leaderboard size to 4
        let to_add : u128 = 5;
        contract.set_size_of_leaderboard(4);
        assert_eq!(4,contract.get_size_of_leaderboard());

        //add 5 scores to leaderboard size 4, should only have 4 on list
        for i in 0..=to_add {
            contract.insert_leaderboard(accounts(i.try_into().unwrap()), (i*100).try_into().unwrap());
        }
        let mut leader_board = contract.get_leaderboard();
        assert_eq!(leader_board.len(), 4);
        println!("{:?}",leader_board);

        // Add one that got bumped to leaderboard with smaller amount should not get added
        contract.insert_leaderboard(accounts(0), 50.try_into().unwrap());
        leader_board = contract.get_leaderboard();
        println!("{:?}",leader_board); // should be same
        assert_eq!(leader_board.len(), 4);


        // remove from the leaderboard
        contract.remove_leaderboard(accounts(3));
        leader_board = contract.get_leaderboard();
        assert_eq!(leader_board.len(), 3);
        println!("Removed {} {:?}",accounts(3),leader_board);

    }

    #[test]
    fn users_to_submit_set_get() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(10000000000000000000000) // attached deposit
            .build()
            
        );

        let mut contract = Score::default();

        context.signer_account_id(accounts(2));
        context.attached_deposit(10000000000000000000000);

        // with near 
        assert_eq!("The backend will upload your score to NEAR within 5 minutes.", contract.add_self_to_submit());
        
        assert_eq!(contract.users_waiting_to_submit(), true);

        // should not clear accounts when done by account 2 -- might not be setting this
        // println!("Users: {:?} ",contract.clear_users_to_submit());
        // context.signer_account_id(accounts(2));
        // assert_eq!(contract.users_waiting_to_submit(), true);

        // // should actually clear when done by account 1 predecessor
        context.signer_account_id(accounts(1));
        println!("Users: {:?} ",contract.clear_users_to_submit());
        assert_eq!(contract.users_waiting_to_submit(), false);
    }


#[test]
#[should_panic]
fn test_no_attached_deposit() {
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context
        .predecessor_account_id(accounts(1))
        .build());

    let mut contract = Score::default();

    context.signer_account_id(accounts(2));
    assert_eq!("To add a score must use at least 0.01 Near to cover gas fees",contract.add_self_to_submit())
}
}