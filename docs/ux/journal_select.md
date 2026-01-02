# User Flow: Journal Selection

## Entry Point
Application launch

## Steps
1. User sees welcome screen:
   - "Open Existing Journal"
   - "Create New Journal"

2. User selects "Open Existing Journal"
3. File picker opens (filtered to `.db`)
4. User selects journal file

## Validation
- File exists
- SQLite schema verified
- Required metadata present

## Outcomes
- Valid → proceed to password verification
- Invalid → error dialog + retry option
- Cancel → application exits

## UX Principles
- Fail fast
- Clear error messaging
- No silent fallback