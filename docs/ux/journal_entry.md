# User Flow: Daily Journal Entry

## Entry Point
Authenticated session

## Steps
1. User opens "New Entry"
2. Prompt is displayed (optional)
3. User writes journal content
4. Auto-save triggers periodically

## Internal Behavior
- Plaintext exists only in memory
- Encryption occurs before persistence
- Nonce + tag generated per save

## Outcomes
- Entry saved securely
- UI reflects save status

## UX Principles
- Zero friction
- No explicit "Save" required
- Clear feedback without interruptions
