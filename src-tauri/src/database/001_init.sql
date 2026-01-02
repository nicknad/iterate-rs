-- ================================
-- 001_init.sql
-- Initial schema for Iterate
-- ================================

BEGIN IMMEDIATE TRANSACTION;

-- ----------------
-- Metadata
-- ----------------
CREATE TABLE IF NOT EXISTS metadata (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO metadata (key, value) VALUES
    ('schema_version', '1'),
    ('created_at_utc', strftime('%s','now'));

-- ----------------
-- Topics
-- ----------------
CREATE TABLE topic (
    topic_id BLOB PRIMARY KEY, -- UUIDv7 (16 bytes)
    name TEXT NOT NULL UNIQUE,
    note TEXT,
    created_at_utc INTEGER NOT NULL
);

-- ----------------
-- Prompts
-- ----------------
CREATE TABLE prompt (
    prompt_id BLOB PRIMARY KEY,
    topic_id BLOB NOT NULL,
    prompt_text TEXT NOT NULL,
    is_default INTEGER NOT NULL CHECK (is_default IN (0,1)),
    FOREIGN KEY (topic_id) REFERENCES topic(topic_id) ON DELETE CASCADE
);

-- Enforce only one default prompt per topic
CREATE UNIQUE INDEX idx_prompt_default
ON prompt(topic_id)
WHERE is_default = 1;

-- ----------------
-- Records (Journal Entries)
-- ----------------
CREATE TABLE record (
    record_id BLOB PRIMARY KEY,
    encrypted_content BLOB NOT NULL,

    sentiment_score REAL CHECK (sentiment_score BETWEEN 0.0 AND 1.0),

    is_summarized INTEGER NOT NULL CHECK (is_summarized IN (0,1)),
    is_summary_record INTEGER NOT NULL CHECK (is_summary_record IN (0,1)),
    is_archived INTEGER NOT NULL CHECK (is_archived IN (0,1)),
    is_deleted INTEGER NOT NULL CHECK (is_deleted IN (0,1)),

    created_at_utc INTEGER NOT NULL,
    last_modified_at_utc INTEGER NOT NULL,
    deleted_at_utc INTEGER
);

-- Common query optimizations
CREATE INDEX idx_record_created_at ON record(created_at_utc);
CREATE INDEX idx_record_flags ON record(is_deleted, is_archived, is_summarized);

-- ----------------
-- Record ↔ Topic (Many-to-Many)
-- ----------------
CREATE TABLE record_topic (
    record_id BLOB NOT NULL,
    topic_id BLOB NOT NULL,
    PRIMARY KEY (record_id, topic_id),
    FOREIGN KEY (record_id) REFERENCES record(record_id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES topic(topic_id) ON DELETE CASCADE
);

-- ----------------
-- Encrypted Integrity Table
-- ----------------
CREATE TABLE key_store (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    kdf_strategy    INTEGER NOT NULL, 
    kdf_salt        BLOB NOT NULL,
    kdf_params      TEXT NOT NULL,    
    wrapped_key     BLOB NOT NULL,    
    created_at_utc  INTEGER NOT NULL
);

-- Exactly one row allowed
CREATE UNIQUE INDEX idx_integrity_singleton
ON integrity_check(id);

COMMIT;
