# User Flow: Password Verification

## Entry Point
Journal file selected

## Steps
1. User is prompted for password
2. Password input masked
3. User confirms submission

## Internal Behavior
- Password copied into SecurePasswordBuffer
- Key derived using stored salt
- Authentication check performed via encrypted check phrase

## Outcomes
- Correct password → session key set → journal unlocked
- Wrong password → authentication fails

## Error Handling
- Generic error message ("Incorrect password")
- No indication of partial correctness

## Security Guarantees
- Password never stored
- Key only lives in memory
- Buffer is zeroed after use
