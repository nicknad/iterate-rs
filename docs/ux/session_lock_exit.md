# User Flow: Session Lock & Exit

## Entry Point
User closes app or locks journal

## Steps
1. App initiates shutdown
2. Session key is cleared
3. All buffers zeroed

## Internal Guarantees
- `FreeKeyAndLockDatabase()` called
- No encryption/decryption possible after
- Re-authentication required on reopen

## UX Principles
- Silent security
- No user action required
