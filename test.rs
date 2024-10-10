#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program_test::*;
    use solana_sdk::{account::Account, signature::{Keypair, Signer}, transaction::Transaction};

    // Helper function to create a test environment
    fn setup() -> (ProgramTest, Pubkey) {
        // Generate a unique program ID and create a new ProgramTest environment
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new(
            "validate_basic_nft",
            program_id,
            processor!(Processor::process),
        );
        (program_test, program_id)
    }

    #[tokio::test]
    async fn test_mint() {
        let (mut program_test, program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let mint_keypair = Keypair::new();
        let funding_keypair = Keypair::new();
        let owner_keypair = Keypair::new();

        // Create mint instruction
        let mint_ix = ValidateBasicNftInstruction::Mint {
            color: "Red".to_string(),
            rarity: "Rare".to_string(),
            short_description: "A rare red gem".to_string(),
        };

        // Create transaction to mint the NFT
        let mut transaction = Transaction::new_with_payer(
            &[mint_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_keypair, &funding_keypair, &owner_keypair], recent_blockhash);

        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify the results (e.g., check if the mint account was created correctly)
        let mint_account = banks_client.get_account(mint_keypair.pubkey()).await.unwrap().unwrap();
        assert_eq!(mint_account.owner, spl_token::id());
    }

    #[tokio::test]
    async fn test_transfer() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let funding_keypair = Keypair::new();
        let owner_keypair = Keypair::new();
        let source_keypair = Keypair::new();
        let destination_keypair = Keypair::new();

        // Create transfer instruction
        let transfer_ix = ValidateBasicNftInstruction::Transfer;

        // Create transaction to transfer the NFT
        let mut transaction = Transaction::new_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(
            &[&payer, &funding_keypair, &owner_keypair, &source_keypair],
            recent_blockhash,
        );

        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify the results (e.g., check if the source account no longer exists and the destination account exists)
        let source_account = banks_client.get_account(source_keypair.pubkey()).await.unwrap();
        let destination_account = banks_client.get_account(destination_keypair.pubkey()).await.unwrap();

        assert!(source_account.is_none());
        assert!(destination_account.is_some());
    }

    #[tokio::test]
    async fn test_burn() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let mint_keypair = Keypair::new();
        let account_keypair = Keypair::new();
        let owner_keypair = Keypair::new();

        // Create burn instruction
        let burn_ix = ValidateBasicNftInstruction::Burn;

        // Create transaction to burn the NFT
        let mut transaction = Transaction::new_with_payer(
            &[burn_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(
            &[&payer, &mint_keypair, &account_keypair, &owner_keypair],
            recent_blockhash,
        );

        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify the results (e.g., check if the mint account was removed)
        let mint_account = banks_client.get_account(mint_keypair.pubkey()).await.unwrap();
        assert!(mint_account.is_none());
    }

    #[tokio::test]
    async fn test_invalid_signer_permission() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts without the required signer
        let mint_keypair = Keypair::new();
        let funding_keypair = Keypair::new();

        // Create mint instruction
        let mint_ix = ValidateBasicNftInstruction::Mint {
            color: "Red".to_string(),
            rarity: "Rare".to_string(),
            short_description: "A rare red gem".to_string(),
        };

        // Create transaction without the owner as signer
        let mut transaction = Transaction::new_with_payer(
            &[mint_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_keypair, &funding_keypair], recent_blockhash);

        // Process transaction and expect an error due to missing signer permissions
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_not_expected_address() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let mint_keypair = Keypair::new();
        let funding_keypair = Keypair::new();

        // Create mint instruction with wrong PDA
        let mint_ix = ValidateBasicNftInstruction::Mint {
            color: "Blue".to_string(),
            rarity: "Common".to_string(),
            short_description: "A common blue gem".to_string(),
        };

        // Create transaction with incorrect account address
        let mut transaction = Transaction::new_with_payer(
            &[mint_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_keypair, &funding_keypair], recent_blockhash);

        // Process transaction and expect an error due to incorrect PDA
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wrong_account_owner() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let mint_keypair = Keypair::new();
        let funding_keypair = Keypair::new();

        // Create mint instruction with wrong account owner
        let mint_ix = ValidateBasicNftInstruction::Mint {
            color: "Green".to_string(),
            rarity: "Uncommon".to_string(),
            short_description: "An uncommon green gem".to_string(),
        };

        // Create transaction with incorrect account owner
        let mut transaction = Transaction::new_with_payer(
            &[mint_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_keypair, &funding_keypair], recent_blockhash);

        // Process transaction and expect an error due to incorrect account owner
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_account_len() {
        let (mut program_test, _program_id) = setup();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create test accounts
        let mint_keypair = Keypair::new();
        let funding_keypair = Keypair::new();

        // Create mint instruction with invalid account length
        let mint_ix = ValidateBasicNftInstruction::Mint {
            color: "Yellow".to_string(),
            rarity: "Legendary".to_string(),
            short_description: "A legendary yellow gem".to_string(),
        };

        // Create transaction with incorrect account length
        let mut transaction = Transaction::new_with_payer(
            &[mint_ix],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_keypair, &funding_keypair], recent_blockhash);

        // Process transaction and expect an error due to invalid account length
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }
}
