use crate::*;
const ACCOUNT_ID: &str = "dev-1652475485403-88942939189193";
const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = 5_000_000_000_000;

#[ext_contract(ext_message)]
trait MessageReciever {
    //cross contract call to an external contract that is initiated during nft_approve
    fn get_message(&self, number: u64) -> Promise;
}

#[near_bindgen]
impl NewContract {

    pub fn get_message_xcc(&mut self, number: u64) -> Promise {
        let receiver_id: AccountId = ACCOUNT_ID.to_string();
        let deposit: Balance = NO_DEPOSIT;
        let gas: Gas = 5_000_000_000_000;
        ext_message::get_message(number, &receiver_id, deposit, gas)
        }
    
}
