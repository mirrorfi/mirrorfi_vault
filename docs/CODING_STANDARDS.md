# MirrorFi Vault Project Coding Standards

*Last Updated: June 28, 2025*

This document outlines the coding standards and best practices to follow when developing the MirrorFi Vault project.

## 1. Import Guidelines

### Specific Imports
Always import specific items rather than using wildcard imports:

```rust
// ❌ Don't do this:
use crate::state::vault::*;
use crate::utils::seeds::*;

// ✅ Do this instead:
use crate::state::vault::{Vault, InitVaultParams};
use crate::utils::seeds; // Import the module, not specific items
```

The only exception is `anchor_lang::prelude::*` which is standard practice for Anchor projects.

### Error Imports
Always import the `ErrorCode` enum directly and reference error variants without the path prefix:

```rust
// ❌ Don't do this:
require!(amount > 0, crate::error::ErrorCode::InvalidAmount);

// ✅ Do this instead:
use crate::error::ErrorCode;
// ...
require!(amount > 0, ErrorCode::InvalidAmount);
```

## 2. Token Program Usage

Always use the modern token interface approach:

```rust
// ❌ Don't do this:
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
// ...
pub token_program: Program<'info, Token>,
// ...
token::transfer(transfer_ctx, args.amount)?;

// ✅ Do this instead:
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, transfer, Transfer};
// ...
pub token_program: Interface<'info, TokenInterface>,
pub associated_token_program: Program<'info, AssociatedToken>,
// ...
transfer(transfer_ctx, args.amount)?;
```

For token accounts, use the following pattern:

```rust
#[account(
    mut,
    token_interface::mint = deposit_token_mint,
    token_interface::authority = vault_authority,
)]
pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
```

## 3. Hierarchy Control

Always enforce the proper hierarchy validation:

1. Protocol is the top-level entity
   - If a protocol is frozen, no operations on its vaults are allowed
   - Always include protocol freeze check when performing operations on vaults

2. Vault is a child entity of Protocol
   - If a vault is frozen, no operations on that specific vault are allowed

Example:
```rust
// Check protocol first
#[account(
    constraint = !protocol.freeze @ ErrorCode::ProtocolFrozen
)]
pub protocol: Account<'info, Protocol>,

// Then check vault
#[account(
    has_one = protocol @ ErrorCode::InvalidProtocol,
    constraint = !vault.freeze @ ErrorCode::VaultFrozen,
)]
pub vault: Account<'info, Vault>,
```

## 4. Account Validation

Always validate account relationships using constraints:

```rust
#[account(
    has_one = manager @ ErrorCode::UnauthorizedAccess,
    has_one = vault_authority @ ErrorCode::InvalidVaultAuthority,
)]
```

For optional accounts, use proper validation before accessing.

### Authority Accounts

Always use `AccountInfo` instead of `UncheckedAccount` for authority PDAs:

```rust
// ❌ Don't do this:
#[account(
    seeds = [seeds::PROTOCOL_AUTHORITY.as_ref(), protocol.key().as_ref()],
    bump,
)]
pub protocol_authority: UncheckedAccount<'info>,

// ✅ Do this instead:
#[account(
    seeds = [seeds::PROTOCOL_AUTHORITY.as_ref(), protocol.key().as_ref()],
    bump,
)]
pub protocol_authority: AccountInfo<'info>,
```

Types of authority accounts in the project:
- `authority`: Used to store token balances. For example, all balances in the vault will be stored in vault_authority token accounts.
- `fee_authority`: Used to store collected fees.

## 5. Error Handling

Use descriptive error codes and proper error handling:

```rust
// Always check arithmetic operations for overflow
vault.total_manager_fee = vault.total_manager_fee.checked_add(args.amount)
    .ok_or(ErrorCode::ArithmeticOverflow)?;
```

## 6. PDA Derivation and Seeds

Use consistent seed patterns as defined in the `seeds.rs` file:

- Protocol: `"mf_protocol"`, creator.key(), args.id`
- Protocol Authority: `"mf_protocol_authority"`, protocol.key()`
- Protocol Fee Authority: `"mf_protocol_fee_authority"`, protocol.key()`
- Vault: `"mf_vault"`, creator.key(), args.id`
- Vault Authority: `"mf_vault_authority"`, vault.key()`
- Vault Fee Authority: `"mf_vault_fee_authority"`, vault.key()`

Always reference seeds through the module to improve readability:

```rust
// ❌ Don't do this:
use crate::utils::seeds::{PROTOCOL, PROTOCOL_AUTHORITY};
// ...
seeds = [PROTOCOL_AUTHORITY.as_ref(), protocol.key().as_ref()],

// ✅ Do this instead:
use crate::utils::seeds;
// ...
seeds = [seeds::PROTOCOL_AUTHORITY.as_ref(), protocol.key().as_ref()],
```

This makes it clear that you're using a seed constant from the seeds module.

## 7. Documentation

Always include clear documentation for public functions and complex logic:

```rust
/// Collects accumulated fees from the vault
/// 
/// This function transfers tokens from the vault's token account to the
/// fee recipient account. It can only be called by the vault manager.
pub fn handle(ctx: Context<CollectVaultFee>, args: CollectFeeArgs) -> Result<()> {
    // ...
}
```

## 8. Handler Structure

Each handler should follow the structure:
1. Input validation
2. Logic implementation
3. State updates
4. Event emission (msg logs)
