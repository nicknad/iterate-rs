## Data Design & Schema

### 1 Storage Strategy

Each journal is a distinct .db file. This allows users to easily backup, move, or separate journals (e.g., Work vs. Personal).

### 2 Entity Relationship Diagram (ERD)

```
 erDiagram
    METADATA {
        TEXT key PK
        TEXT value
    }

    KEY_STORE {
        INTEGER id PK "Singleton (id=1)"
        INTEGER kdf_strategy "Enum discriminant"
        BLOB kdf_params "Postcard Serialized KdfParams"
        BLOB wrapped_key "CryptoEnvelope(MasterKey)"
        INTEGER created_at_utc
    }

    TOPIC {
        BLOB topic_id PK "UUIDv7"
        TEXT name "Unique"
        TEXT note
        INTEGER created_at_utc
    }

    PROMPT {
        BLOB prompt_id PK "UUIDv7"
        BLOB topic_id FK
        TEXT prompt_text
        INTEGER is_default "Boolean (0/1)"
    }

    RECORD {
        BLOB record_id PK "UUIDv7"
        BLOB encrypted_content "Postcard Serialized CryptoEnvelope"
        REAL sentiment_score "0.0 to 1.0"
        INTEGER is_summarized "Boolean"
        INTEGER is_summary_record "Boolean"
        INTEGER is_archived "Boolean"
        INTEGER is_deleted "Boolean"
        INTEGER created_at_utc "Unix Timestamp"
        INTEGER last_modified_at_utc
        INTEGER deleted_at_utc
    }

    RECORD_TOPIC {
        BLOB record_id FK
        BLOB topic_id FK
    }

    TOPIC ||--o{ PROMPT : "contains"
    TOPIC ||--o{ RECORD_TOPIC : "tagged"
    RECORD ||--o{ RECORD_TOPIC : "categorized"
```
