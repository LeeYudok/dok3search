# Module rusqlite::auto_extension

## Overview

Automatic extension loading

## Items

- `cancel_auto_extension` — Unregister the initialization routine
- `init_auto_extension⚠` — Bridge between RawAutoExtension and AutoExtension
- `register_auto_extension⚠` — Register au auto-extension
- `reset_auto_extension` — Disable all automatic extensions previously registered
- `AutoExtension` — Automatic extension initialization routine
- `RawAutoExtension` — Raw automatic extension initialization routine

### fn rusqlite::auto_extension::cancel_auto_extension

```rust
pub fn cancel_auto_extension(ax: RawAutoExtension) -> bool
```

Unregister the initialization routine

### fn rusqlite::auto_extension::init_auto_extension

```rust
pub unsafe fn init_auto_extension(
    db: *mut sqlite3,
    pz_err_msg: *mut *mut c_char,
    ax: AutoExtension,
) -> c_int
```

Bridge between `RawAutoExtension` and `AutoExtension`

**Safety**

- Opening a database from an auto-extension handler will lead to
an endless recursion of the auto-handler triggering itself
indirectly for each newly-opened database.

- Results are undefined if the given db is closed by an auto-extension.

- The list of auto-extensions should not be manipulated from an auto-extension.

### fn rusqlite::auto_extension::register_auto_extension

```rust
pub unsafe fn register_auto_extension(ax: RawAutoExtension) -> Result<()>
```

Register au auto-extension

**Safety**

- Opening a database from an auto-extension handler will lead to
an endless recursion of the auto-handler triggering itself
indirectly for each newly-opened database.

- Results are undefined if the given db is closed by an auto-extension.

- The list of auto-extensions should not be manipulated from an auto-extension.

### fn rusqlite::auto_extension::reset_auto_extension

```rust
pub fn reset_auto_extension()
```

Disable all automatic extensions previously registered

### type rusqlite::auto_extension::AutoExtension

```rust
pub type AutoExtension = fn(Connection) -> Result<()>;
```

Automatic extension initialization routine

### type rusqlite::auto_extension::RawAutoExtension

```rust
pub type RawAutoExtension = unsafe extern "C" fn(db: *mut sqlite3, pz_err_msg: *mut *mut c_char, _: *const sqlite3_api_routines) -> c_int;
```

Raw automatic extension initialization routine
