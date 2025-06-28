# MirrorFi Vault Documentation

*Last Updated: June 28, 2025*

This directory contains all the documentation for the MirrorFi Vault project.

## Available Documentation

- [**GUIDE.md**](./GUIDE.md) - The primary reference for Solana program development patterns and best practices specific to this project
- [**CODING_STANDARDS.md**](./CODING_STANDARDS.md) - Detailed coding standards and conventions to follow when developing for this project
- [**AI_CONTEXT.md**](./AI_CONTEXT.md) - Context information for AI assistance with this project

## Quick Reference

### Project Structure
- `programs/mirrorfi_vault/src/handler/` - Instruction handler modules
- `programs/mirrorfi_vault/src/state/` - Program state definitions
- `programs/mirrorfi_vault/src/utils/` - Utility functions and constants
- `programs/mirrorfi_vault/src/error.rs` - Error definitions

### Key Standards
1. Use specific imports instead of wildcards
2. Reference seeds via module (`seeds::PROTOCOL_AUTHORITY`) instead of direct import
3. Use `AccountInfo` instead of `UncheckedAccount` for authority PDAs
4. Follow hierarchy validation (protocol → vault)
5. Use the token interface instead of the token program directly
6. Use explicit error handling with properly imported error codes

## Getting Started

For new developers to the project, we recommend reading the documentation in this order:
1. GUIDE.md - For understanding the core concepts
2. CODING_STANDARDS.md - For learning the specific code style and patterns
3. AI_CONTEXT.md - For context on AI assistance with the project
