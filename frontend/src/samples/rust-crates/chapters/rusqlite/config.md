# Module rusqlite::config

## Overview

Configure database connections

## Items

- `DbConfig` — Database Connection Configuration Options See Database Connection Configuration Options for details.

### enum rusqlite::config::DbConfig

```rust
#[non_exhaustive]#[repr(i32)]pub enum DbConfig {
    SQLITE_DBCONFIG_ENABLE_FKEY = 1_002,
    SQLITE_DBCONFIG_ENABLE_TRIGGER = 1_003,
    SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER = 1_004,
    SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE = 1_006,
    SQLITE_DBCONFIG_ENABLE_QPSG = 1_007,
    SQLITE_DBCONFIG_TRIGGER_EQP = 1_008,
    SQLITE_DBCONFIG_RESET_DATABASE = 1_009,
    SQLITE_DBCONFIG_DEFENSIVE = 1_010,
    SQLITE_DBCONFIG_WRITABLE_SCHEMA = 1_011,
    SQLITE_DBCONFIG_LEGACY_ALTER_TABLE = 1_012,
    SQLITE_DBCONFIG_DQS_DML = 1_013,
    SQLITE_DBCONFIG_DQS_DDL = 1_014,
    SQLITE_DBCONFIG_ENABLE_VIEW = 1_015,
    SQLITE_DBCONFIG_LEGACY_FILE_FORMAT = 1_016,
    SQLITE_DBCONFIG_TRUSTED_SCHEMA = 1_017,
    SQLITE_DBCONFIG_STMT_SCANSTATUS = 1_018,
    SQLITE_DBCONFIG_REVERSE_SCANORDER = 1_019,
    SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE = 1_020,
    SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE = 1_021,
    SQLITE_DBCONFIG_ENABLE_COMMENTS = 1_022,
    SQLITE_DBCONFIG_FP_DIGITS = 1_023,
}
```

Database Connection Configuration Options
See Database Connection Configuration Options for details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
