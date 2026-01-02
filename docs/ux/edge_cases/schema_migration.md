## Scenario

User opens journal with older schema or crypto version.

## Flow

App detects version mismatch

User is prompted:

“This journal needs to be upgraded.”

Migration runs:

Backup created

Re-encrypt using AES-GCM

Success → open journal

Failure → restore backup