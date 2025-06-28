// Sol handlers
pub mod handler_wrap_sol;
pub mod handler_unwrap_sol;
pub use handler_wrap_sol::*;
pub use handler_unwrap_sol::*;

// Kamino handlers
pub mod handler_kamino_init_obligation;
pub use handler_kamino_init_obligation::*;

// Protocol handlers
pub mod handler_protocol_initialize;
pub mod handler_protocol_freeze;
pub mod handler_protocol_unfreeze;
pub use handler_protocol_initialize::*;
pub use handler_protocol_freeze::*;
pub use handler_protocol_unfreeze::*;

// Vault handlers
pub mod handler_vault_initialize;
pub mod handler_vault_freeze;
pub mod handler_vault_unfreeze;
pub mod handler_vault_collect_fee;
pub use handler_vault_initialize::*;
pub use handler_vault_freeze::*;
pub use handler_vault_unfreeze::*;
pub use handler_vault_collect_fee::*;

// Test handlers
pub mod handler_random_cpi;
pub use handler_random_cpi::*;