use crate::airdrop::errors::AirdropError;
use crate::airdrop::structs::{Address, TokenAmount};


pub type AirdropResult<T> = Result<T, AirdropError>;

/// Create a virtual Blockchain environment to test the airdrop program
/// 
#[derive(Debug)]
pub struct VirtualBlockchain {
    pub nounce: u64,
}



impl VirtualBlockchain{
    pub fn new() -> Self {
        Self {
            nounce: 0,
        }
    }

    pub fn get_nounce(&mut self) -> u64 {
        self.nounce += 1;
        self.nounce
    }

    pub fn send_transaction(& mut self, _recipient: &Address, _amount: &TokenAmount)-> AirdropResult<String>{

        if rand::random::<u64>() < 10 {
            return Err(AirdropError::NetworkTimeout);
        }
        self.nounce += 1;
        let transaction_id = format!("tx_{:016x}", self.nounce);

        Ok(transaction_id)
     }
}


