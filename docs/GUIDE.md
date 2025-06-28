# Ultimate Guide to Writing Accurate Solana Programs for MirrorFi Vault

*Last Updated: June 28, 2025*

## About MirrorFi Vault:
MirrorFi Vault is a **Solana Program** written in **Anchor** that stores users funds in individual vaults handled by the manager. The vault manager will have the authority to use the funds inside the vault to interact with multiple protocols with the goal to earn yields for the users, which will be distributed as rewards to the vault depositors.

## FUNDAMENTAL PRINCIPLES FOR AI CODE GENERATION

1. **NEVER guess or approximate Solana/Anchor patterns** - always follow exact patterns shown in this guide
2. **ALWAYS include proper account validation** - every account must be properly validated
3. **ALWAYS use checked math operations** - never use unchecked arithmetic
4. **ALWAYS specify exact account sizes** - memory allocation must be precise
5. **ALWAYS handle errors explicitly** - use proper error types and propagation

## Core Development Principles

### 1. Account Model and State Management
- Use Anchor's account validation macros
- Use Account Loader to store lightweight account state
- Implement proper PDA derivation for vaults
- Ensure proper fund custody and authority checks

#### 1.1. CRITICAL ACCOUNT TYPE SELECTION

| Account Type | When to Use | Size Limit | Example |
|-------------|------------|------------|----------|
| `#[account]` | Small, simple state | < 10KB | `VaultState`, basic configuration |
| `#[account(zero_copy(unsafe))]` | Large, complex data | Unlimited | `Protocol` with array fields or complex structures |

#### 1.2. PRECISE MEMORY ALLOCATION

```rust
// ALWAYS specify exact account sizes
#[account(
    init,
    payer = user,
    // CORRECT - use explicit size_of calculation
    space = 8 + std::mem::size_of::<VaultState>()
)]

// OR implement Size trait
impl Size for Protocol {
    // Include 8-byte discriminator
    const SIZE: usize = 152 + 8;
}

// THEN verify with const_assert
const_assert_eq!(
    Protocol::SIZE,
    std::mem::size_of::<Protocol>() + 8 // Always add discriminator
);
```

```rust
// Example PDA derivation for vault
#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[derive(Accounts)]
#[repr(C)]
pub struct Protocol {
    /// Address of protocol's creator
    pub creator: Pubkey,
    /// Update authority of protocol state
    pub owner: Pubkey,
    /// Creation timestamp of protocol
    pub created_at: i64,
    /// Last updated timestamp of the protocol
    pub updated_at: i64,
     /// Fee Authority of the protocol
    pub protocol_fee_authority: Pubkey,
    /// Extra Space
    #[derivative(Debug = "ignore")]
    pub padding: [u64; 4],
    /// The Fee Rate of the protocol, 1000 = 1% (decimal = 3)
    #[derivative(Default(value="0u32"))] // 0%
    pub protocol_fee_rate: u32, // 0%
    /// Protocol Initialization State
    pub is_initialized: bool,
    /// Protocol Freeze State
    pub freeze: bool,
    /// Protocol Version
    pub version: u8,
    pub bump: u8,
}
```

### 2. Essential Security Patterns

#### Account Validation
```rust
// ALWAYS validate account relationships
#[account]
pub struct VaultState {
    pub manager: Pubkey,
    pub total_deposits: u64,
    pub rewards_available: u64,
    pub bump: u8,
}

// ALWAYS implement proper access control
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require_gt!(amount, 0, ErrorCode::InvalidAmount);
    require!(
        ctx.accounts.vault.manager == ctx.accounts.manager.key(),
        ErrorCode::InvalidManager
    );
    // ... rest of deposit logic
}
```

### 3. Error Handling

#### 3.1. PROPER ERROR DEFINITION AND USAGE

```rust
// CORRECT: Define custom errors with messages in an enum
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Invalid manager")]
    InvalidManager,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,    // Name clearly indicates error type
    #[msg("Insufficient funds")]
    InsufficientFunds,
}

// CORRECT: Import error codes explicitly
use crate::error::ErrorCode;

// CORRECT: Use pattern for requiring conditions with custom errors
require_gte!(user.lamports(), amount, ErrorCode::InsufficientSolToWrap);

// CORRECT: Propagate errors with the ? operator
let result = some_operation()?;

// CORRECT: Return errors from operations with meaningful context
self.total_deposits.checked_add(amount).ok_or(ErrorCode::ArithmeticOverflow)?
```

## Project Structure

```
mirrorfi_vault/
├── programs/
│   └── mirrorfi_vault/
│       ├── src/
│       │   ├── lib.rs        # Program entry point
│       │   ├── state.rs      # Account structures
│       │   ├── instructions/  # Program instructions
│       │   └── errors.rs     # Custom errors
│       └── Cargo.toml
├── tests/                    # Integration tests
└── Anchor.toml              # Program configuration
```

## Development Checklist

1. **Account Structure**
   - [ ] Define vault state account
   - [ ] Implement proper PDA derivation
   - [ ] Set up token accounts

2. **Core Instructions**
   - [ ] Initialize vault
   - [ ] Deposit funds
   - [ ] Withdraw funds
   - [ ] Distribute rewards

3. **Security Measures**
   - [ ] Implement access control
   - [ ] Add input validation
   - [ ] Handle arithmetic safely

4. **Testing**
   - [ ] Unit tests for each instruction
   - [ ] Integration tests
   - [ ] Edge case testing

5. **Deployment**
   - [ ] Security audit
   - [ ] Gas optimization
   - [ ] Program verification

## 4. Cross-Program Invocation (CPI) Patterns

This project implements multiple CPI patterns to interact with both native Solana programs and external protocols. Below are the standardized approaches used throughout the codebase.

### 4.1. Token Interface CPI with Anchor Context

```rust
// CORRECT: Complete CPI using Anchor's CpiContext for token transfers
pub fn collect_fee(ctx: Context<CollectVaultFee>, args: CollectFeeArgs) -> Result<()> {
    // Ensure the amount is greater than zero
    require!(args.amount > 0, ErrorCode::InvalidAmount);
    
    // Set up PDA signer seeds with module-qualified constants
    let seeds = &[
        seeds::VAULT_AUTHORITY.as_ref(),
        ctx.accounts.vault.key().as_ref(),
        &[*ctx.bumps.get("vault_authority").unwrap()],
    ];
    let signer_seeds = &[&seeds[..]];
    
    // Create the transfer instruction with proper account structure
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.fee_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    
    // Execute the token transfer using the token interface
    transfer(transfer_ctx, args.amount)?;
    
    // Update the vault state with safe arithmetic
    let vault = &mut ctx.accounts.vault;
    vault.total_manager_fee = vault.total_manager_fee.checked_add(args.amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    vault.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Collected {} vault fees successfully", args.amount);
    
    Ok(())
}
```

### 4.2. Generic Protocol CPI Using Raw Instructions

For protocols without Anchor CPI interfaces, we implement a custom CPI approach using a utility library found in `src/utils/cpi.rs`. This approach uses low-level instruction creation and invocation.

```rust
// CORRECT: Using our generic CPI utility for external program invocation (handler_random_cpi.rs)
pub fn handle(ctx: Context<RandomCpi>) -> Result<()> {
    msg!("Executing CPI to plutonian_initialize instruction");

    // 1. Create account metas for the CPI call
    let account_metas = vec![
        // actor - writable and signer
        AccountMeta::new(ctx.accounts.actor.key(), true),
        // user - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        // protocol - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.protocol.key(), false),
        // Additional accounts...
    ];
    
    // 2. Create instruction data using our helper function
    // This creates the 8-byte discriminator + serialized args
    let instruction_data = create_instruction_data("plutonian_initialize", &EmptyArgs {})?;
    
    // 3. Create the instruction using our helper function
    let ix = create_instruction(
        ctx.accounts.program.key(),
        account_metas, 
        instruction_data
    );

    // 4. Gather account infos in the same order as account_metas
    let account_infos = &[
        ctx.accounts.actor.to_account_info(),
        ctx.accounts.user.to_account_info(),
        // Other account infos...
    ];
    
    // 5. Execute the instruction using our helper function
    execute_cpi(ix, account_infos)?;

    msg!("Successfully executed CPI call");
    Ok(())
}
```

### 4.3. Protocol-Specific CPI Modules (Kamino Example)

For complex protocols used frequently, we create dedicated CPI modules with structured account types. See the Kamino CPI implementation in `src/cpi/kamino/`:

```rust
// CORRECT: Protocol-specific CPI module usage (from cpi/kamino/init_obligation.rs)
pub fn execute<'info>(
    accounts: InitObligationAccounts<'info>,
    program_id: AccountInfo<'info>,
    args: InitObligationArgs,
) -> Result<()> {
    // 1. Define the accounts needed for the instruction using our struct
    let account_metas = accounts.to_account_metas(None);

    // 2. Create instruction data using our helper function
    let instruction_data = create_instruction_data("initObligation", &args)?;
    
    // 3. Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // 4. Get account infos in the same order as account metas
    let account_infos = &[
        accounts.obligation_owner.to_account_info(),
        accounts.fee_payer.to_account_info(),
        // Other account infos...
        program_id,
    ];

    // 5. Execute the instruction using our helper function
    execute_cpi(ix, account_infos)
}

// Usage in a handler:
pub fn handle_kamino_init(ctx: Context<KaminoInitContext>, args: KaminoArgs) -> Result<()> {
    // Validate protocol is not frozen
    require!(!ctx.accounts.protocol.freeze, ErrorCode::ProtocolFrozen);
    require!(!ctx.accounts.vault.freeze, ErrorCode::VaultFrozen);
    
    // Build the accounts struct
    let kamino_accounts = InitObligationAccounts {
        obligation_owner: ctx.accounts.vault_authority.to_account_info(),
        fee_payer: ctx.accounts.payer.to_account_info(),
        obligation: ctx.accounts.obligation.to_account_info(),
        // Other account mappings...
    };
    
    // Execute the CPI with our module
    kamino::init_obligation::execute(
        kamino_accounts,
        ctx.accounts.kamino_program.to_account_info(),
        InitObligationArgs { tag: 0, id: 1 }
    )?;
    
    msg!("Initialized Kamino obligation");
    Ok(())
}
```

### 4.4. Using CPI with PDA Signers

When the CPI requires a PDA to sign, use the PDA seeds following our project's conventions:

```rust
// CORRECT: Using PDA signers in CPI calls
pub fn vault_action_with_authority(ctx: Context<VaultActionContext>, args: ActionArgs) -> Result<()> {
    // 1. Prepare PDA signing using project's seed constants
    let authority_seeds = &[
        seeds::VAULT_AUTHORITY.as_ref(),
        ctx.accounts.vault.key().as_ref(),
        &[*ctx.bumps.get("vault_authority").unwrap()]
    ];
    let signer_seeds = &[&authority_seeds[..]];
    
    // 2. Create instruction as shown in previous examples
    // ... instruction creation code ...
    
    // 3. Execute with PDA signing
    execute_cpi_with_signer(ix, account_infos, signer_seeds)?;
    
    Ok(())
}
```

### 4.5. CPI Utility Functions

The project provides the following CPI utility functions in `utils/cpi.rs`:

```rust
// Calculate an Anchor instruction discriminator (first 8 bytes of SHA-256 hash)
get_discriminator(name: &str) -> [u8; 8]

// Create instruction data with discriminator and serialized args
create_instruction_data<T: AnchorSerialize>(instruction_name: &str, args: &T) -> Result<Vec<u8>>

// Create a CPI instruction with the given program ID, accounts, and instruction data
create_instruction(program_id: Pubkey, accounts: Vec<AccountMeta>, data: Vec<u8>) -> Instruction

// Execute a CPI call with no signing PDA
execute_cpi(instruction: Instruction, account_infos: &[AccountInfo]) -> Result<()>

// Execute a CPI call with a signing PDA
execute_cpi_with_signer(
    instruction: Instruction,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]] 
) -> Result<()>
```

## 5. PDA Management

```rust
// CORRECT: PDA derivation and validation in account validation struct
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,
    
    #[account(
        init,
        payer = manager,
        space = VaultAccount::SIZE,
        seeds = [seeds::VAULT.as_ref(), manager.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultAccount>,
    
    #[account(
        seeds = [seeds::VAULT_AUTHORITY.as_ref(), vault.key().as_ref()],
        bump,
    )]
    /// CHECK: This is a PDA authority that's validated by the seed constraint
    pub vault_authority: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

// CORRECT: Manual PDA validation when needed
pub fn validate_vault_authority(vault_authority: &AccountInfo, manager: &Pubkey, program_id: &Pubkey) -> Result<u8> {
    let (expected_vault_authority, bump) = Pubkey::find_program_address(
        &[seeds::VAULT.as_ref(), manager.as_ref()],
        program_id
    );
    
    require_keys_eq!(
        *vault_authority.key,
        expected_vault_authority,
        ErrorCode::InvalidVaultAuthority
    );
    
    Ok(bump)
}
```

## 8. Security Measures

```rust
// CORRECT: Ownership validation for token accounts
require_eq!(
    token_account.owner,
    &anchor_spl::token_interface::ID,  // Use token_interface ID
    ErrorCode::InvalidTokenAccountOwner
);

// CORRECT: Signer validation
require!(ctx.accounts.authority.is_signer, ErrorCode::SignerRequired);

// CORRECT: Reentrancy protection
pub struct ReentrancyGuard {
    pub count: u64
}

impl ReentrancyGuard {
    pub fn enter(&mut self) -> Result<()> {
        require_eq!(self.count, 0, ErrorCode::ReentrancyDetected);
        self.count = 1;
        Ok(())
    }

    pub fn exit(&mut self) {
        self.count = 0;
    }
}
```

## 9. Common Mistakes to Avoid

1. **NEVER** use unchecked arithmetic operations (`+`, `-`, `*`, `/`)
2. **NEVER** skip account validation or rely on client-side validation
3. **NEVER** hardcode addresses that might change between environments
4. **NEVER** use string concatenation for PDA seeds - use byte arrays
5. **NEVER** ignore return values from CPIs
6. **NEVER** assume token decimals - always check mint metadata
7. **NEVER** make security assumptions about account ordering
8. **NEVER** use raw account data directly without proper deserializing
9. **NEVER** deploy without testing all error scenarios
10. **NEVER** ignore transaction and compute unit limits

## 10. AI Code Generation Guidelines

1. **Generate exact imports** - list ALL required dependencies at the top of the file
2. **Generate complete code** - provide ALL necessary structs, enums, and functions
3. **Use strict types** - avoid `impl` when concrete types are known
4. **Provide inline docs** - explain complex code sections
5. **Be explicit** - avoid shortcuts even if they appear cleaner
6. **Include tests** - generate test cases for all functions

Remember: Solana programs are immutable once deployed. Thorough testing and validation before deployment is crucial.
