pub type AirdropResult<T> = Result<T, AirdropError>;
use airdrop::airdrop::errors::AirdropError;
use airdrop::airdrop::structs::TokenAmount;
use airdrop::airdrop::AirdropProcessor;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

fn main() -> AirdropResult<()> {
    // Initialize the airdrop processor with an initial balance and max batch size
    let mut airdrop_processor = AirdropProcessor::new(TokenAmount::new(100000000), 10);

    // Load recipients from a CSV file
    let recipients = airdrop_processor.load_recipients_from_csv("recipients.csv")?;

    // Process the airdrop for the loaded recipients
    airdrop_processor.process_batches(recipients)?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use airdrop::airdrop::errors::AirdropError;
    use airdrop::airdrop::processor::AirdropProcessor;
    use airdrop::airdrop::structs::{Address, Recipient, TokenAmount, Transactions};
    use airdrop::airdrop::utils::VirtualBlockchain;

    #[test]
    fn test_address_creation() {
        let address = Address::new("0x1234567890123456789012345678901234567890").unwrap();
        let expected = Address::new("0x1234567890123456789012345678901234567890").unwrap();

        assert_eq!(address, expected);
    }

    #[test]
    fn test_address_from_csv_file() {
        let path = Path::new("src/recipients.csv");
        let file = File::open(path).expect("Failed to open file");
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut recipients = Vec::new();
        for result in reader.deserialize() {
            let record: Recipient = result.expect("Failed to deserialize record");
            recipients.push(record);
        }
        assert_eq!(recipients.len(), 3);
        assert_eq!(
            recipients[0].address,
            Address::new("0x1234567890123456789012345678901234567890").unwrap()
        );
        assert_eq!(
            recipients[1].address,
            Address::new("0x2345678901234567890123456789012345678901").unwrap()
        );
        assert_eq!(
            recipients[2].address,
            Address::new("0x3456789012345678901234567890123456789012").unwrap()
        );
    }

    #[test]
    fn test_load_recipients_from_csv() {
        let path = Path::new("src/recipients.csv");
        let airdrop_processor = AirdropProcessor::new(TokenAmount::new(100000000), 10);
        let recipients = airdrop_processor.load_recipients_from_csv(path).unwrap();
        assert_eq!(recipients.len(), 3);
    }

    #[test]
    fn test_validate_batch() {
        let path = Path::new("src/recipients.csv");
        let mut airdrop_processor = AirdropProcessor::new(TokenAmount::new(100000000), 10);
        let recipients = airdrop_processor.load_recipients_from_csv(path).unwrap();
        assert_eq!(recipients.len(), 3);

        let validate_batch = airdrop_processor.validate_batch(&recipients).unwrap();
        // assert_eq! if is the right amount is returned
        assert_eq!(validate_batch, 600);
        // assert_eq! if the batch size is correct
        assert_eq!(airdrop_processor.get_max_batch_size(), 10);
        // assert_eq! if the balance is correct
        assert_eq!(airdrop_processor.get_balance(), TokenAmount::new(100000000));
        // assert_eq! if the transaction is correct
        assert_eq!(airdrop_processor.get_transaction().len(), 0);
        // assert_eq! if the blockchain is correct
        assert_eq!(airdrop_processor.blockchain.get_nounce(), 1);
    }

    #[test]
    fn test_process_batches() {
        let path = Path::new("src/recipients.csv");
        let mut airdrop_processor = AirdropProcessor::new(TokenAmount::new(100000000), 10);
        let recipients = airdrop_processor.load_recipients_from_csv(path).unwrap();
        assert_eq!(recipients.len(), 3);

        let validate_batch = airdrop_processor.validate_batch(&recipients).unwrap();

        let result = airdrop_processor.process_batches(recipients);
        assert!(result.is_ok());
        let transactions = airdrop_processor.get_transaction();
        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0].status.is_complete(), true);
        assert_eq!(transactions[1].status.is_complete(), true);
        assert_eq!(transactions[2].status.is_complete(), true);
        assert_eq!(
            airdrop_processor.get_balance(),
            TokenAmount::new(100000000 - validate_batch)
        );
        assert_eq!(airdrop_processor.blockchain.get_nounce(), 7);
    }
}
