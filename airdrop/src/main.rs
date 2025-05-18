pub type AirdropResult<T> = Result<T, AirdropError>;
use airdrop::airdrop::AirdropProcessor;
 use airdrop::airdrop::structs::TokenAmount;
 use airdrop::airdrop::errors::AirdropError;


fn main() -> AirdropResult<()> {
    // Initialize the airdrop processor with an initial balance and max batch size
    let mut airdrop_processor = AirdropProcessor::new(TokenAmount::new(100000000), 10);

    // Load recipients from a CSV file
    let recipients = airdrop_processor.load_recipients_from_csv("recipients.csv")?;

    // Process the airdrop for the loaded recipients
    airdrop_processor.process_batches(recipients)?;

    Ok(())

    
    
}
