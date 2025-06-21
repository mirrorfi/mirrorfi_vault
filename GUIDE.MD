# Ultimate Guide to Writing Accurate Solana Programs for MirrorFi Vault

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
    require_gt!(amount, 0, VaultError::InvalidAmount);
    require!(
        ctx.accounts.vault.manager == ctx.accounts.manager.key(),
        VaultError::InvalidManager
    );
    // ... rest of deposit logic
}
```

### 3. Critical Operations

#### 3.1. TOKEN HANDLING PATTERNS

```rust
// CORRECT: Proper token account initialization with Anchor
#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    // Initialize associated token account
    #[account(
        init_if_needed,  // Create if doesn't exist
        payer = payer,   // Who pays for account creation
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub owner: SystemAccount<'info>,
    
    // ALWAYS include required programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
```

#### 3.2. HANDLING SOL/WSOL OPERATIONS

```rust
// CORRECT: Proper wrapping of SOL to WSOL
#[inline(never)]  // Prevents inlining for better stack usage
pub fn handle_wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_ata = &ctx.accounts.user_ata;

    // ALWAYS validate sufficient balance
    require_gte!(user.lamports(), amount, Errors::InsufficientSolToWrap);

    // 1. Transfer SOL to WSOL ATA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: user.to_account_info(),
            to: user_ata.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    // 2. SYNC SOL AS WSOL
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative{
            account: user_ata.to_account_info(),
        },
    );
    sync_native(cpi_ctx)?;

    msg!("Successfully wrapped {} SOL to WSOL", amount);
    Ok(())
}
```

#### 3.3. STRUCTURED HANDLER PATTERN

```rust
// GOOD PATTERN: Separate handler function with error handling

// 1. Define instruction processing function in lib.rs
pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(data)?;
    
    match instruction {
        VaultInstruction::Initialize { params } => {
            handlers::initialize::handle(program_id, accounts, params)
        },
        VaultInstruction::Deposit { amount } => {
            handlers::deposit::handle(program_id, accounts, amount)
        },
    }
}

// 2. Use module structure for handlers
mod handlers {
    pub mod initialize { /* handler code */ }
    pub mod deposit { /* handler code */ }
}

// 3. Clear handler implementation
#[inline(never)] // Prevents inlining for better stack usage
pub fn handle(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Validate input
    require_gt!(amount, 0, VaultError::InvalidAmount);
    
    // Execute business logic
    execute_deposit(ctx, amount)?;
    
    // Update state
    update_vault_state(ctx, amount)?;
    
    // Emit event
    emit!(DepositEvent { amount });
    
    Ok(())
}
```

### 4. Error Handling

#### 4.1. PROPER ERROR DEFINITION AND USAGE

```rust
// CORRECT: Define custom errors with messages in an enum
#[error_code]
pub enum VaultError {
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Invalid manager")]
    InvalidManager,
    #[msg("Arithmetic overflow")]
    MathOverflow,    // Name clearly indicates error type
    #[msg("Insufficient funds")]
    InsufficientFunds,
}

// CORRECT: Descriptive module-level errors 
pub mod Errors {
    use anchor_lang::prelude::*;
    
    #[error_code]
    pub enum VaultError {
        #[msg("Math overflow occurred during calculation")]
        MathOverflow,
        #[msg("Insufficient SOL to wrap")]
        InsufficientSolToWrap,
    }
}

// CORRECT: Use pattern for requiring conditions with custom errors
require_gte!(user.lamports(), amount, Errors::InsufficientSolToWrap);

// CORRECT: Propagate errors with the ? operator
let result = some_operation()?;

// CORRECT: Return errors from operations with meaningful context
self.total_deposits.checked_add(amount).ok_or(VaultError::MathOverflow)?
```

#### 4.2 ERROR LOGGING AND TRACING

```rust
// CORRECT: Log before critical operations
msg!("Processing deposit of {} tokens from {}", amount, user.key());

// CORRECT: Log successful operation completions
msg!("Successfully wrapped {} SOL to WSOL", amount);

// NEVER ignore errors without logging
if let Err(err) = result {
    msg!("Operation failed: {:?}", err);
    return Err(err);
}
```

### 5. Testing Requirements

```typescript
describe('mirrorfi_vault', () => {
    it('Initializes vault correctly', async () => {
        // Arrange
        const manager = anchor.web3.Keypair.generate();
        
        // Act
        await program.methods
            .initializeVault()
            .accounts({
                vault: vaultPDA,
                manager: manager.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([manager])
            .rpc();
            
        // Assert
        const vaultAccount = await program.account.vaultState.fetch(vaultPDA);
        assert.equal(vaultAccount.manager.toBase58(), manager.publicKey.toBase58());
    });
});
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

## 6. Cross-Program Invocation (CPI) Patterns

```rust
// CORRECT: Complete CPI with proper context
pub fn withdraw_from_external_protocol(ctx: Context<ExternalProtocolWithdraw>, amount: u64) -> Result<()> {
    // 1. Build CPI accounts with exact structure required by the target program
    let cpi_accounts = ExternalWithdraw {
        user: ctx.accounts.user.to_account_info(),
        vault: ctx.accounts.vault.to_account_info(),
        token_account: ctx.accounts.token_account.to_account_info(),
        external_program: ctx.accounts.external_program.to_account_info(),
    };
    
    // 2. Create CPI context with account validation
    let external_program_id = ctx.accounts.external_program.key();
    require_eq!(external_program_id, external_protocol::ID, VaultError::InvalidExternalProgram);
    
    // 3. Create proper CPI context
    let cpi_ctx = CpiContext::new(
        ctx.accounts.external_program.to_account_info(),
        cpi_accounts,
    );
    
    // 4. Sign with PDA if needed
    let seeds = &[
        b"vault",
        ctx.accounts.manager.key().as_ref(),
        &[ctx.accounts.vault.bump],
    ];
    let signer = &[&seeds[..]];
    
    let cpi_ctx = cpi_ctx.with_signer(signer);
    
    // 5. Call the external program
    external_protocol::cpi::withdraw(cpi_ctx, amount)?;
    
    Ok(())
}
```

## 7. PDA Management

```rust
// CORRECT: PDA derivation and validation in account validation struct
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = manager,
        space = 8 + std::mem::size_of::<VaultState>(),
        seeds = [b"vault", manager.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    
    // Rest of accounts...
}

// CORRECT: Manual PDA validation when needed
pub fn validate_vault_authority(vault_authority: &AccountInfo, manager: &Pubkey, program_id: &Pubkey) -> Result<u8> {
    let (expected_vault_authority, bump) = Pubkey::find_program_address(
        &[b"vault", manager.as_ref()],
        program_id
    );
    
    require_keys_eq!(
        *vault_authority.key,
        expected_vault_authority,
        VaultError::InvalidVaultAuthority
    );
    
    Ok(bump)
}
```

## 8. Security Measures

```rust
// CORRECT: Ownership validation
require_eq!(
    token_account.owner,
    &Token::id(),
    VaultError::InvalidTokenAccountOwner
);

// CORRECT: Signer validation
require!(ctx.accounts.authority.is_signer, VaultError::SignerRequired);

// CORRECT: Reentrancy protection
pub struct ReentrancyGuard {
    pub count: u64
}

impl ReentrancyGuard {
    pub fn enter(&mut self) -> Result<()> {
        require_eq!(self.count, 0, VaultError::ReentrancyDetected);
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
