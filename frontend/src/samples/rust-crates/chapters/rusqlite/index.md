# Crate rusqlite

## Overview

Rusqlite is an ergonomic wrapper for using SQLite from Rust.

Historically, the API was based on the one from
`rust-postgres`. However, the
two have diverged in many ways, and no compatibility between the two is
intended.

```rust
use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}
```

## Items

- `auto_extension` — Automatic extension loading
- `config` — Configure database connections
- `types` — Traits dealing with SQLite data types.
- `named_params` — A macro making it more convenient to pass lists of named parameters as a &[(&str, &dyn ToSql)].
- `params` — A macro making it more convenient to pass longer lists of parameters as a &[&dyn ToSql].
- `AndThenRows` — An iterator over the mapped resulting rows of a query, with an Error type unifying with Error.
- `Batch` — Batch fallible iterator
- `Connection` — A connection to a SQLite database.
- `InterruptHandle` — Allows interrupting a long-running computation.
- `Map` — F is used to transform the streaming iterator into a fallible iterator.
- `MappedRows` — An iterator over the mapped resulting rows of a query.
- `OpenFlags` — Flags for opening SQLite database connections. See sqlite3_open_v2 for details.
- `ParamsFromIter` — Adapter type which allows any iterator over ToSql values to implement Params.
- `PrepFlags` — Prepare flags. See sqlite3_prepare_v3 for details.
- `Row` — A single result row of a query.
- `Rows` — A handle (lazy fallible streaming iterator) for the resulting rows of a query.
- `Savepoint` — Represents a savepoint on a database connection.
- `Statement` — A prepared statement.
- `Transaction` — Represents a transaction on a database connection.
- `DropBehavior` — Options for how a Transaction or Savepoint should behave when it is dropped.
- `Error` — Enum listing possible errors from rusqlite.
- `ErrorCode` — Error Codes
- `FileControl` — Standard File Control Opcodes
- `StatementStatus` — Prepared statement status counters.
- `TransactionBehavior` — Options for transaction behavior. See BEGIN TRANSACTION for details.
- `TransactionState` — Transaction state of a database
- `DEFAULT_NAME` — Shorthand for default name.
- `MAIN_DB` — Shorthand for Main database.
- `TEMP_DB` — Shorthand for Temp database.
- `BindIndex` — A trait implemented by types that can index into parameters of a statement.
- `Name` — Database, table, column, collation, function, module, vfs name
- `OptionalExtension` — See the method documentation.
- `Params` — Trait used for sets of parameter passed into SQL statements/queries.
- `RowIndex` — A trait implemented by types that can index into columns of a row.
- `params_from_iter` — Constructor function for a ParamsFromIter. See its documentation for more.
- `to_sqlite_error⚠` — Transform Rust error to SQLite error (message and code).
- `version` — Returns the SQLite version as a string; e.g., "3.16.2" for version 3.16.2.
- `version_number` — Returns the SQLite version as an integer; e.g., 3016002 for version 3.16.2.
- `Result` — A typedef of the result returned by many methods.

### constant rusqlite::DEFAULT_NAME

```rust
pub const DEFAULT_NAME: Option<&'static str>;
```

Shorthand for default name.

### constant rusqlite::MAIN_DB

```rust
pub const MAIN_DB: &CStr = c"main";
```

Shorthand for `Main` database.

### constant rusqlite::TEMP_DB

```rust
pub const TEMP_DB: &CStr = c"temp";
```

Shorthand for `Temp` database.

### enum rusqlite::DropBehavior

```rust
#[non_exhaustive]pub enum DropBehavior {
    Rollback,
    Commit,
    Ignore,
    Panic,
}
```

Options for how a Transaction or Savepoint should behave when it is dropped.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::Error

```rust
#[non_exhaustive]pub enum Error {
    SqliteFailure(Error, Option<String>),
    SqliteSingleThreadedMode,
    FromSqlConversionFailure(usize, Type, Box<dyn Error + Send + Sync + 'static>),
    IntegralValueOutOfRange(usize, i64),
    Utf8Error(usize, Utf8Error),
    NulError(NulError),
    InvalidParameterName(String),
    InvalidPath(PathBuf),
    ExecuteReturnedResults,
    QueryReturnedNoRows,
    QueryReturnedMoreThanOneRow,
    InvalidColumnIndex(usize),
    InvalidColumnName(String),
    InvalidColumnType(usize, String, Type),
    StatementChangedRows(usize),
    ToSqlConversionFailure(Box<dyn Error + Send + Sync + 'static>),
    Empty,
    InvalidQuery,
    UnwindingPanic,
    MultipleStatement,
    InvalidParameterCount(usize, usize),
    SqlInputError {
        error: Error,
        msg: String,
        sql: String,
        offset: c_int,
    },
    InvalidDatabaseIndex(usize),
}
```

Enum listing possible errors from rusqlite.

**Methods**

- `pub fn sqlite_error(&self) -> Option<&Error>`
- `pub fn sqlite_error_code(&self) -> Option<ErrorCode>`
- `pub fn sqlite_extended_error_code(&self) -> Option<c_int>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(err: FromSqlError) -> Self`
- `fn from(err: NulError) -> Self`
- `fn from(err: Utf8Error) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::ErrorCode

```rust
#[non_exhaustive]pub enum ErrorCode {
    InternalMalfunction,
    PermissionDenied,
    OperationAborted,
    DatabaseBusy,
    DatabaseLocked,
    OutOfMemory,
    ReadOnly,
    OperationInterrupted,
    SystemIoFailure,
    DatabaseCorrupt,
    NotFound,
    DiskFull,
    CannotOpen,
    FileLockingProtocolFailed,
    SchemaChanged,
    TooBig,
    ConstraintViolation,
    TypeMismatch,
    ApiMisuse,
    NoLargeFileSupport,
    AuthorizationForStatementDenied,
    ParameterOutOfRange,
    NotADatabase,
    Unknown,
}
```

Error Codes

**Methods**

- `fn clone(&self) -> ErrorCode`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &ErrorCode) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::FileControl

```rust
#[non_exhaustive]pub enum FileControl<'p> {
    SizeHint(&'p mut sqlite3_int64),
    SizeLimit(&'p mut sqlite3_int64),
    ChunkSize(&'p mut c_int),
    PersistWal(&'p mut c_int),
    PowerSafeOverwrite(&'p mut c_int),
    MMapSize(&'p mut sqlite3_int64),
    HasMoved(&'p mut c_int),
    LockTimeout(&'p mut c_int),
    BlockOnConnect(&'p mut c_int),
    DataVersion(&'p mut c_uint),
    ReserveBytes(&'p mut c_int),
}
```

Standard File Control Opcodes

See `https://sqlite.org/c3ref/c_fcntl_begin_atomic_write.html` for explanations of each.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### enum rusqlite::StatementStatus

```rust
#[non_exhaustive]#[repr(i32)]pub enum StatementStatus {
    FullscanStep = 1,
    Sort = 2,
    AutoIndex = 3,
    VmStep = 4,
    RePrepare = 5,
    Run = 6,
    FilterMiss = 7,
    FilterHit = 8,
    MemUsed = 99,
}
```

Prepared statement status counters.

See `https://www.sqlite.org/c3ref/c_stmtstatus_counter.html`
for explanations of each.

Note that depending on your version of SQLite, all of these
may not be available.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### enum rusqlite::TransactionBehavior

```rust
#[non_exhaustive]pub enum TransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
}
```

Options for transaction behavior. See BEGIN
TRANSACTION for details.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`

### enum rusqlite::TransactionState

```rust
#[non_exhaustive]pub enum TransactionState {
    None,
    Read,
    Write,
}
```

Transaction state of a database

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### fn rusqlite::params_from_iter

```rust
pub fn params_from_iter<I>(iter: I) -> ParamsFromIter<I>where
    I: IntoIterator,
    I::Item: ToSql,
```

Constructor function for a `ParamsFromIter`. See its documentation for
more.

### fn rusqlite::to_sqlite_error

```rust
pub unsafe fn to_sqlite_error(e: &Error, err_msg: *mut *mut c_char) -> c_int
```

Transform Rust error to SQLite error (message and code).

**Safety**

This function is unsafe because it uses raw pointer

### fn rusqlite::version

```rust
pub fn version() -> &'static str
```

Returns the SQLite version as a string; e.g., `"3.16.2"` for version 3.16.2.

See `sqlite3_libversion()`.

**Panics**

Panics when version is not valid UTF-8.

### fn rusqlite::version_number

```rust
pub fn version_number() -> i32
```

Returns the SQLite version as an integer; e.g., `3016002` for version
3.16.2.

See `sqlite3_libversion_number()`.

### macro rusqlite::named_params

```rust
macro_rules! named_params {
    () => { ... };
    ($($param_name:literal: $param_val:expr),+ $(,)?) => { ... };
}
```

A macro making it more convenient to pass lists of named parameters
as a `&[(&str, &dyn ToSql)]`.

**Example**

```rust

struct Person {
    name: String,
    age_in_years: u8,
    data: Option<Vec<u8>>,
}

fn add_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person (name, age_in_years, data)
         VALUES (:name, :age, :data)",
        named_params! {
            ":name": person.name,
            ":age": person.age_in_years,
            ":data": person.data,
        },
    )?;
    Ok(())
}
```

### macro rusqlite::params

```rust
macro_rules! params {
    () => { ... };
    ($($param:expr),+ $(,)?) => { ... };
}
```

A macro making it more convenient to pass longer lists of
parameters as a `&[&dyn ToSql]`.

**Example**

```rust

struct Person {
    name: String,
    age_in_years: u8,
    data: Option<Vec<u8>>,
}

fn add_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person(name, age_in_years, data) VALUES (?1, ?2, ?3)",
        params![person.name, person.age_in_years, person.data],
    )?;
    Ok(())
}
```

### struct rusqlite::AndThenRows

```rust
pub struct AndThenRows<'stmt, F> { /* private fields */ }
```

An iterator over the mapped resulting rows of a query, with an Error type
unifying with Error.

**Methods**

- `fn next(&mut self) -> Option<Self::Item>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryTypewhere Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`
- `fn try_reduce<R>( &mut self, f: impl FnMut(Self::Item, Self::Item) -> R, ) -> <<R as Try>::Residual as Residual<Option<<R as Try>::Output>>>::TryTypewhere Self: Sized, R: Try<Output = Self::Item>, <R as Try>::Residual: Residual<Option<Self::Item>>,`

### struct rusqlite::Batch

```rust
pub struct Batch<'conn, 'sql> { /* private fields */ }
```

Batch fallible iterator

**Warning**

There is no recovery on parsing error, when an invalid statement is found in `sql`, SQLite cannot jump to the next statement.
So you should break the loop when an error is raised by the `next` method.

```rust
use fallible_iterator::FallibleIterator;
use rusqlite::{Batch, Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let sql = r"
    CREATE TABLE tbl1 (col);
    CREATE TABLE tbl2 (col);
    ";
    let mut batch = Batch::new(&conn, sql);
    while let Some(mut stmt) = batch.next()? {
        stmt.execute([])?;
    }
    Ok(())
}
```

**Methods**

- `pub fn new(conn: &'conn Connection, sql: &'sql str) -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn next(&mut self) -> Result<Option<Statement<'conn>>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn count(self) -> Result<usize, Self::Error>where Self: Sized,`
- `fn last(self) -> Result<Option<Self::Item>, Self::Error>where Self: Sized,`
- `fn nth(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error>`
- `fn step_by(self, step: usize) -> StepBy<Self>where Self: Sized,`
- `fn chain<I>(self, it: I) -> Chain<Self, I>where I: IntoFallibleIterator<Item = Self::Item, Error = Self::Error>, Self: Sized,`
- `fn zip<I>( self, o: I, ) -> Zip<Self, <I as IntoFallibleIterator>::IntoFallibleIter>where Self: Sized, I: IntoFallibleIterator<Error = Self::Error>,`
- `fn map<F, B>(self, f: F) -> Map<Self, F>where Self: Sized, F: FnMut(Self::Item) -> Result<B, Self::Error>,`
- `fn for_each<F>(self, f: F) -> Result<(), Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<(), Self::Error>,`
- `fn filter<F>(self, f: F) -> Filter<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>where Self: Sized, F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn enumerate(self) -> Enumerate<Self>where Self: Sized,`
- `fn peekable(self) -> Peekable<Self>where Self: Sized,`
- `fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>where Self: Sized, P: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>where Self: Sized, P: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn skip(self, n: usize) -> Skip<Self>where Self: Sized,`
- `fn take(self, n: usize) -> Take<Self>where Self: Sized,`
- `fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>where Self: Sized, F: FnMut(&mut St, Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>where Self: Sized, U: IntoFallibleIterator<Error = Self::Error>, F: FnMut(Self::Item) -> Result<U, Self::Error>,`
- `fn fuse(self) -> Fuse<Self>where Self: Sized,`
- `fn inspect<F>(self, f: F) -> Inspect<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> Result<(), Self::Error>,`
- `fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `fn collect<T>(self) -> Result<T, Self::Error>where T: FromIterator<Self::Item>, Self: Sized,`
- `fn partition<B, F>(self, f: F) -> Result<(B, B), Self::Error>where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn fold<B, F>(self, init: B, f: F) -> Result<B, Self::Error>where Self: Sized, F: FnMut(B, Self::Item) -> Result<B, Self::Error>,`
- `fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>where Self: Sized, E: From<Self::Error>, F: FnMut(B, Self::Item) -> Result<B, E>,`
- `fn all<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn any<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn find<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn find_map<B, F>(&mut self, f: F) -> Result<Option<B>, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn position<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn max_by_key<B, F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, B: Ord, F: FnMut(&Self::Item) -> Result<B, Self::Error>,`
- `fn max_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,`
- `fn min_by_key<B, F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, B: Ord, F: FnMut(&Self::Item) -> Result<B, Self::Error>,`
- `fn min_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,`
- `fn unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), Self::Error>where Self: Sized + FallibleIterator<Item = (A, B)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>,`
- `fn cloned<'a, T>(self) -> Cloned<Self>where Self: Sized + FallibleIterator<Item = &'a T>, T: 'a + Clone,`

### struct rusqlite::Connection

```rust
pub struct Connection { /* private fields */ }
```

A connection to a SQLite database.

**Methods**

- `pub fn busy_timeout(&self, timeout: Duration) -> Result<()>`
- `pub fn busy_handler<F>(&mut self, callback: Option<F>) -> Result<()>where F: FnMut(i32) -> bool + Send + 'static,`
- `pub fn column_exists<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<bool>`
- `pub fn table_exists<N: Name>( &self, db_name: Option<N>, table_name: N, ) -> Result<bool>`
- `pub fn column_metadata<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<(Option<CString>, Option<CString>, bool, bool, bool)>`
- `pub fn db_config(&self, config: DbConfig) -> Result<bool>`
- `pub fn set_db_config(&self, config: DbConfig, new_val: bool) -> Result<bool>`
- `pub fn pragma_query_one<T: FromSql>( &self, schema_name: Option<&str>, pragma_name: &str, ) -> Result<T>`
- `pub fn pragma_query_value<T, F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn pragma_query<F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>,`
- `pub fn pragma<F, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>, V: ToSql,`
- `pub fn pragma_update<V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, ) -> Result<()>where V: ToSql,`
- `pub fn pragma_update_and_check<F, T, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>, V: ToSql,`
- `pub fn transaction(&mut self) -> Result<Transaction<'_>>`
- `pub fn transaction_with_behavior( &mut self, behavior: TransactionBehavior, ) -> Result<Transaction<'_>>`
- `pub fn unchecked_transaction(&self) -> Result<Transaction<'_>>`
- `pub fn savepoint(&mut self) -> Result<Savepoint<'_>>`
- `pub fn savepoint_with_name<T: Into<String>>( &mut self, name: T, ) -> Result<Savepoint<'_>>`
- `pub fn transaction_state<N: Name>( &self, db_name: Option<N>, ) -> Result<TransactionState>`
- `pub fn set_transaction_behavior(&mut self, behavior: TransactionBehavior)`
- `pub fn open<P: AsRef<Path>>(path: P) -> Result<Self>`
- `pub fn open_in_memory() -> Result<Self>`
- `pub fn open_with_flags<P: AsRef<Path>>( path: P, flags: OpenFlags, ) -> Result<Self>`
- `pub fn open_with_flags_and_vfs<P: AsRef<Path>, V: Name>( path: P, flags: OpenFlags, vfs: V, ) -> Result<Self>`
- `pub fn open_in_memory_with_flags(flags: OpenFlags) -> Result<Self>`
- `pub fn open_in_memory_with_flags_and_vfs<V: Name>( flags: OpenFlags, vfs: V, ) -> Result<Self>`
- `pub fn execute_batch(&self, sql: &str) -> Result<()>`
- `pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>`
- `pub fn path(&self) -> Option<&str>`
- `pub fn release_memory(&self) -> Result<()>`
- `pub fn last_insert_rowid(&self) -> i64`
- `pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_one<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_row_and_then<T, E, P, F>( &self, sql: &str, params: P, f: F, ) -> Result<T, E>where P: Params, F: FnOnce(&Row<'_>) -> Result<T, E>, E: From<Error>,`
- `pub fn prepare(&self, sql: &str) -> Result<Statement<'_>>`
- `pub fn prepare_with_flags( &self, sql: &str, flags: PrepFlags, ) -> Result<Statement<'_>>`
- `pub fn close(self) -> Result<(), (Self, Error)>`
- `pub unsafe fn handle(&self) -> *mut sqlite3`
- `pub unsafe fn from_handle(db: *mut sqlite3) -> Result<Self>`
- `pub unsafe fn from_handle_owned(db: *mut sqlite3) -> Result<Self>`

### struct rusqlite::InterruptHandle

```rust
pub struct InterruptHandle { /* private fields */ }
```

Allows interrupting a long-running computation.

**Methods**

- `pub fn interrupt(&self)`

### struct rusqlite::Map

```rust
pub struct Map<'stmt, F> { /* private fields */ }
```

`F` is used to transform the streaming iterator into a fallible
iterator.

**Methods**

- `fn next(&mut self) -> Result<Option<B>>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn count(self) -> Result<usize, Self::Error>where Self: Sized,`
- `fn last(self) -> Result<Option<Self::Item>, Self::Error>where Self: Sized,`
- `fn nth(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error>`
- `fn step_by(self, step: usize) -> StepBy<Self>where Self: Sized,`
- `fn chain<I>(self, it: I) -> Chain<Self, I>where I: IntoFallibleIterator<Item = Self::Item, Error = Self::Error>, Self: Sized,`
- `fn zip<I>( self, o: I, ) -> Zip<Self, <I as IntoFallibleIterator>::IntoFallibleIter>where Self: Sized, I: IntoFallibleIterator<Error = Self::Error>,`
- `fn map<F, B>(self, f: F) -> Map<Self, F>where Self: Sized, F: FnMut(Self::Item) -> Result<B, Self::Error>,`
- `fn for_each<F>(self, f: F) -> Result<(), Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<(), Self::Error>,`
- `fn filter<F>(self, f: F) -> Filter<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>where Self: Sized, F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn enumerate(self) -> Enumerate<Self>where Self: Sized,`
- `fn peekable(self) -> Peekable<Self>where Self: Sized,`
- `fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>where Self: Sized, P: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>where Self: Sized, P: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn skip(self, n: usize) -> Skip<Self>where Self: Sized,`
- `fn take(self, n: usize) -> Take<Self>where Self: Sized,`
- `fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>where Self: Sized, F: FnMut(&mut St, Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>where Self: Sized, U: IntoFallibleIterator<Error = Self::Error>, F: FnMut(Self::Item) -> Result<U, Self::Error>,`
- `fn flatten(self) -> Flatten<Self>where Self: Sized, Self::Item: IntoFallibleIterator<Error = Self::Error>,`
- `fn fuse(self) -> Fuse<Self>where Self: Sized,`
- `fn inspect<F>(self, f: F) -> Inspect<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> Result<(), Self::Error>,`
- `fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `fn collect<T>(self) -> Result<T, Self::Error>where T: FromIterator<Self::Item>, Self: Sized,`
- `fn partition<B, F>(self, f: F) -> Result<(B, B), Self::Error>where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn fold<B, F>(self, init: B, f: F) -> Result<B, Self::Error>where Self: Sized, F: FnMut(B, Self::Item) -> Result<B, Self::Error>,`
- `fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>where Self: Sized, E: From<Self::Error>, F: FnMut(B, Self::Item) -> Result<B, E>,`
- `fn all<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn any<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn find<F>(&mut self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> Result<bool, Self::Error>,`
- `fn find_map<B, F>(&mut self, f: F) -> Result<Option<B>, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<Option<B>, Self::Error>,`
- `fn position<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>where Self: Sized, F: FnMut(Self::Item) -> Result<bool, Self::Error>,`
- `fn max(self) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, Self::Item: Ord,`
- `fn max_by_key<B, F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, B: Ord, F: FnMut(&Self::Item) -> Result<B, Self::Error>,`
- `fn max_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,`
- `fn min(self) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, Self::Item: Ord,`
- `fn min_by_key<B, F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, B: Ord, F: FnMut(&Self::Item) -> Result<B, Self::Error>,`
- `fn min_by<F>(self, f: F) -> Result<Option<Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Result<Ordering, Self::Error>,`
- `fn unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), Self::Error>where Self: Sized + FallibleIterator<Item = (A, B)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>,`

### struct rusqlite::MappedRows

```rust
pub struct MappedRows<'stmt, F> { /* private fields */ }
```

An iterator over the mapped resulting rows of a query.

`F` is used to transform the streaming iterator into a standard
iterator.

**Methods**

- `fn next(&mut self) -> Option<Result<T>>`
- `fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>where Self: Sized,`
- `1.0.0 (const: unstable) · fn size_hint(&self) -> (usize, Option<usize>)`
- `1.0.0 (const: unstable) · fn count(self) -> usizewhere Self: Sized,`
- `1.0.0 (const: unstable) · fn last(self) -> Option<Self::Item>where Self: Sized,`
- `fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>`
- `1.0.0 (const: unstable) · fn nth(&mut self, n: usize) -> Option<Self::Item>`
- `1.28.0 (const: unstable) · fn step_by(self, step: usize) -> StepBy<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator<Item = Self::Item>,`
- `1.0.0 (const: unstable) · fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> ⓘwhere Self: Sized, U: IntoIterator,`
- `fn intersperse(self, separator: Self::Item) -> Intersperse<Self> ⓘwhere Self: Sized, Self::Item: Clone,`
- `fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> ⓘwhere Self: Sized, G: FnMut() -> Self::Item,`
- `1.0.0 (const: unstable) · fn map<B, F>(self, f: F) -> Map<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> B,`
- `1.21.0 (const: unstable) · fn for_each<F>(self, f: F)where Self: Sized, F: FnMut(Self::Item),`
- `1.0.0 (const: unstable) · fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> ⓘwhere Self: Sized, F: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn enumerate(self) -> Enumerate<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn peekable(self) -> Peekable<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.0.0 (const: unstable) · fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(&Self::Item) -> bool,`
- `1.57.0 (const: unstable) · fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> ⓘwhere Self: Sized, P: FnMut(Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn skip(self, n: usize) -> Skip<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn take(self, n: usize) -> Take<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> ⓘwhere Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,`
- `1.0.0 (const: unstable) · fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> ⓘwhere Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,`
- `1.29.0 (const: unstable) · fn flatten(self) -> Flatten<Self> ⓘwhere Self: Sized, Self::Item: IntoIterator,`
- `fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> ⓘwhere Self: Sized, F: FnMut(&[Self::Item; N]) -> R,`
- `1.0.0 (const: unstable) · fn fuse(self) -> Fuse<Self> ⓘwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn inspect<F>(self, f: F) -> Inspect<Self, F> ⓘwhere Self: Sized, F: FnMut(&Self::Item),`
- `1.0.0 (const: unstable) · fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `1.0.0 (const: unstable) · fn collect<B>(self) -> Bwhere B: FromIterator<Self::Item>, Self: Sized,`
- `fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryTypewhere Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,`
- `fn collect_into<E>(self, collection: &mut E) -> &mut Ewhere E: Extend<Self::Item>, Self: Sized,`
- `1.0.0 (const: unstable) · fn partition<B, F>(self, f: F) -> (B, B)where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,`
- `fn is_partitioned<P>(self, predicate: P) -> boolwhere Self: Sized, P: FnMut(Self::Item) -> bool,`
- `1.27.0 (const: unstable) · fn try_fold<B, F, R>(&mut self, init: B, f: F) -> Rwhere Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,`
- `1.27.0 (const: unstable) · fn try_for_each<F, R>(&mut self, f: F) -> Rwhere Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,`
- `1.0.0 (const: unstable) · fn fold<B, F>(self, init: B, f: F) -> Bwhere Self: Sized, F: FnMut(B, Self::Item) -> B,`
- `1.51.0 (const: unstable) · fn reduce<F>(self, f: F) -> Option<Self::Item>where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,`
- `fn try_reduce<R>( &mut self, f: impl FnMut(Self::Item, Self::Item) -> R, ) -> <<R as Try>::Residual as Residual<Option<<R as Try>::Output>>>::TryTypewhere Self: Sized, R: Try<Output = Self::Item>, <R as Try>::Residual: Residual<Option<Self::Item>>,`

### struct rusqlite::OpenFlags

```rust
pub struct OpenFlags(/* private fields */);
```

Flags for opening SQLite database connections. See
sqlite3_open_v2 for details.

The default open flags are `SQLITE_OPEN_READ_WRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_URI | SQLITE_OPEN_NO_MUTEX`. See `Connection::open` for
some discussion about these flags.

**Methods**

- `pub const fn empty() -> Self`
- `pub const fn all() -> Self`
- `pub const fn bits(&self) -> c_int`
- `pub const fn from_bits(bits: c_int) -> Option<Self>`
- `pub const fn from_bits_truncate(bits: c_int) -> Self`
- `pub const fn from_bits_retain(bits: c_int) -> Self`
- `pub fn from_name(name: &str) -> Option<Self>`
- `pub const fn is_empty(&self) -> bool`
- `pub const fn is_all(&self) -> bool`
- `pub const fn intersects(&self, other: Self) -> bool`
- `pub const fn contains(&self, other: Self) -> bool`
- `pub fn insert(&mut self, other: Self)`
- `pub fn remove(&mut self, other: Self)`
- `pub fn toggle(&mut self, other: Self)`
- `pub fn set(&mut self, other: Self, value: bool)`
- `pub const fn intersection(self, other: Self) -> Self`
- `pub const fn union(self, other: Self) -> Self`
- `pub const fn difference(self, other: Self) -> Self`
- `pub const fn symmetric_difference(self, other: Self) -> Self`
- `pub const fn complement(self) -> Self`
- `pub const fn iter(&self) -> Iter<OpenFlags>`
- `pub const fn iter_names(&self) -> IterNames<OpenFlags>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn bitand(self, other: Self) -> Self`
- `fn bitand_assign(&mut self, other: Self)`
- `fn bitor(self, other: OpenFlags) -> Self`
- `fn bitor_assign(&mut self, other: Self)`
- `fn bitxor(self, other: Self) -> Self`
- `fn bitxor_assign(&mut self, other: Self)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn extend<T: IntoIterator<Item = Self>>(&mut self, iterator: T)`
- `fn extend_one(&mut self, item: T)`
- `fn extend_reserve(&mut self, additional: usize)`
- `fn bits(&self) -> c_int`
- `fn from_bits_retain(bits: c_int) -> OpenFlags`
- `fn all_named() -> OpenFlags`
- `fn empty() -> Self`

### struct rusqlite::ParamsFromIter

```rust
pub struct ParamsFromIter<I>(/* private fields */);
```

Adapter type which allows any iterator over `ToSql` values to implement
`Params`.

This struct is created by the `params_from_iter` function.

This can be useful if you have something like an `&[String]` (of unknown
length), and you want to use them with an API that wants something
implementing `Params`. This way, you can avoid having to allocate storage
for something like a `&[&dyn ToSql]`.

This essentially is only ever actually needed when dynamically generating
SQL — static SQL (by definition) has the number of parameters known
statically. As dynamically generating SQL is itself pretty advanced, this
API is itself for advanced use cases (See “Realistic use case” in the
examples).

**Example**

**Basic usage**

```rust
use rusqlite::{Connection, Result, params_from_iter};
use std::collections::BTreeSet;

fn query(conn: &Connection, ids: &BTreeSet<String>) -> Result<()> {
    assert_eq!(ids.len(), 3, "Unrealistic sample code");

    let mut stmt = conn.prepare("SELECT * FROM users WHERE id IN (?1, ?2, ?3)")?;
    let _rows = stmt.query(params_from_iter(ids.iter()))?;

    // use _rows...
    Ok(())
}
```

**Realistic use case**

Here’s how you’d use `ParamsFromIter` to call `Statement::exists` with a
dynamic number of parameters.

...

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct rusqlite::PrepFlags

```rust
pub struct PrepFlags(/* private fields */);
```

Prepare flags. See
sqlite3_prepare_v3 for details.

**Methods**

- `pub const fn empty() -> Self`
- `pub const fn all() -> Self`
- `pub const fn bits(&self) -> c_uint`
- `pub const fn from_bits(bits: c_uint) -> Option<Self>`
- `pub const fn from_bits_truncate(bits: c_uint) -> Self`
- `pub const fn from_bits_retain(bits: c_uint) -> Self`
- `pub fn from_name(name: &str) -> Option<Self>`
- `pub const fn is_empty(&self) -> bool`
- `pub const fn is_all(&self) -> bool`
- `pub const fn intersects(&self, other: Self) -> bool`
- `pub const fn contains(&self, other: Self) -> bool`
- `pub fn insert(&mut self, other: Self)`
- `pub fn remove(&mut self, other: Self)`
- `pub fn toggle(&mut self, other: Self)`
- `pub fn set(&mut self, other: Self, value: bool)`
- `pub const fn intersection(self, other: Self) -> Self`
- `pub const fn union(self, other: Self) -> Self`
- `pub const fn difference(self, other: Self) -> Self`
- `pub const fn symmetric_difference(self, other: Self) -> Self`
- `pub const fn complement(self) -> Self`
- `pub const fn iter(&self) -> Iter<PrepFlags>`
- `pub const fn iter_names(&self) -> IterNames<PrepFlags>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn bitand(self, other: Self) -> Self`
- `fn bitand_assign(&mut self, other: Self)`
- `fn bitor(self, other: PrepFlags) -> Self`
- `fn bitor_assign(&mut self, other: Self)`
- `fn bitxor(self, other: Self) -> Self`
- `fn bitxor_assign(&mut self, other: Self)`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn extend<T: IntoIterator<Item = Self>>(&mut self, iterator: T)`
- `fn extend_one(&mut self, item: T)`
- `fn extend_reserve(&mut self, additional: usize)`
- `fn bits(&self) -> c_uint`
- `fn from_bits_retain(bits: c_uint) -> PrepFlags`
- `fn all_named() -> PrepFlags`
- `fn empty() -> Self`

### struct rusqlite::Row

```rust
pub struct Row<'stmt> { /* private fields */ }
```

A single result row of a query.

**Methods**

- `pub fn get_unwrap<I: RowIndex, T: FromSql>(&self, idx: I) -> T`
- `pub fn get<I: RowIndex, T: FromSql>(&self, idx: I) -> Result<T>`
- `pub fn get_ref<I: RowIndex>(&self, idx: I) -> Result<ValueRef<'_>>`
- `pub fn get_ref_unwrap<I: RowIndex>(&self, idx: I) -> ValueRef<'_>`
- `fn as_ref(&self) -> &Statement<'stmt>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`
- `fn try_from(row: &'a Row<'a>) -> Result<Self>`

### struct rusqlite::Rows

```rust
pub struct Rows<'stmt> { /* private fields */ }
```

A handle (lazy fallible streaming iterator) for the resulting rows of a query.

**Methods**

- `pub fn next(&mut self) -> Result<Option<&Row<'stmt>>>`
- `pub fn map<F, B>(self, f: F) -> Map<'stmt, F>where F: FnMut(&Row<'_>) -> Result<B>,`
- `pub fn mapped<F, B>(self, f: F) -> MappedRows<'stmt, F> ⓘwhere F: FnMut(&Row<'_>) -> Result<B>,`
- `pub fn and_then<F, T, E>(self, f: F) -> AndThenRows<'stmt, F> ⓘwhere F: FnMut(&Row<'_>) -> Result<T, E>,`
- `pub fn as_ref(&self) -> Option<&Statement<'stmt>>`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`
- `fn advance(&mut self) -> Result<()>`
- `fn get(&self) -> Option<&Row<'stmt>>`
- `fn next(&mut self) -> Result<Option<&Self::Item>, Self::Error>`
- `fn size_hint(&self) -> (usize, Option<usize>)`
- `fn all<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn any<F>(&mut self, f: F) -> Result<bool, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn by_ref(&mut self) -> &mut Selfwhere Self: Sized,`
- `fn count(self) -> Result<usize, Self::Error>where Self: Sized,`
- `fn filter<F>(self, f: F) -> Filter<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn find<F>(&mut self, f: F) -> Result<Option<&Self::Item>, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn for_each<F>(self, f: F) -> Result<(), Self::Error>where Self: Sized, F: FnMut(&Self::Item),`
- `fn fuse(self) -> Fuse<Self>where Self: Sized,`
- `fn map<F, B>(self, f: F) -> Map<Self, F, B>where Self: Sized, F: FnMut(&Self::Item) -> B,`
- `fn map_ref<F, B>(self, f: F) -> MapRef<Self, F>where Self: Sized, F: Fn(&Self::Item) -> &B, B: ?Sized,`
- `fn map_err<F, B>(self, f: F) -> MapErr<Self, F>where Self: Sized, F: Fn(Self::Error) -> B,`
- `fn nth(&mut self, n: usize) -> Result<Option<&Self::Item>, Self::Error>`
- `fn position<F>(&mut self, f: F) -> Result<Option<usize>, Self::Error>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn skip(self, n: usize) -> Skip<Self>where Self: Sized,`
- `fn skip_while<F>(self, f: F) -> SkipWhile<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> bool,`
- `fn take(self, n: usize) -> Take<Self>where Self: Sized,`
- `fn take_while<F>(self, f: F) -> TakeWhile<Self, F>where Self: Sized, F: FnMut(&Self::Item) -> bool,`

### struct rusqlite::Savepoint

```rust
pub struct Savepoint<'conn> { /* private fields */ }
```

Represents a savepoint on a database connection.

**Note**

Savepoints will roll back by default. Use `commit` method to explicitly
commit the savepoint, or use `set_drop_behavior` to change what happens
when the savepoint is dropped.

**Example**

```rust
fn perform_queries(conn: &mut Connection) -> Result<()> {
    let sp = conn.savepoint()?;

    do_queries_part_1(&sp)?; // sp causes rollback if this fails
    do_queries_part_2(&sp)?; // sp causes rollback if this fails

    sp.commit()
}
```

**Methods**

- `pub fn new(conn: &mut Connection) -> Result<Savepoint<'_>>`
- `pub fn with_name<T: Into<String>>( conn: &mut Connection, name: T, ) -> Result<Savepoint<'_>>`
- `pub fn savepoint(&mut self) -> Result<Savepoint<'_>>`
- `pub fn savepoint_with_name<T: Into<String>>( &mut self, name: T, ) -> Result<Savepoint<'_>>`
- `pub fn drop_behavior(&self) -> DropBehavior`
- `pub fn set_drop_behavior(&mut self, drop_behavior: DropBehavior)`
- `pub fn commit(self) -> Result<()>`
- `pub fn rollback(&mut self) -> Result<()>`
- `pub fn finish(self) -> Result<()>`
- `pub fn busy_timeout(&self, timeout: Duration) -> Result<()>`
- `pub fn column_exists<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<bool>`
- `pub fn table_exists<N: Name>( &self, db_name: Option<N>, table_name: N, ) -> Result<bool>`
- `pub fn column_metadata<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<(Option<CString>, Option<CString>, bool, bool, bool)>`
- `pub fn db_config(&self, config: DbConfig) -> Result<bool>`
- `pub fn set_db_config(&self, config: DbConfig, new_val: bool) -> Result<bool>`
- `pub fn pragma_query_one<T: FromSql>( &self, schema_name: Option<&str>, pragma_name: &str, ) -> Result<T>`
- `pub fn pragma_query_value<T, F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn pragma_query<F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>,`
- `pub fn pragma<F, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>, V: ToSql,`
- `pub fn pragma_update<V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, ) -> Result<()>where V: ToSql,`
- `pub fn pragma_update_and_check<F, T, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>, V: ToSql,`
- `pub fn unchecked_transaction(&self) -> Result<Transaction<'_>>`
- `pub fn transaction_state<N: Name>( &self, db_name: Option<N>, ) -> Result<TransactionState>`
- `pub fn execute_batch(&self, sql: &str) -> Result<()>`
- `pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>`
- `pub fn path(&self) -> Option<&str>`
- `pub fn release_memory(&self) -> Result<()>`
- `pub fn last_insert_rowid(&self) -> i64`
- `pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_one<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_row_and_then<T, E, P, F>( &self, sql: &str, params: P, f: F, ) -> Result<T, E>where P: Params, F: FnOnce(&Row<'_>) -> Result<T, E>, E: From<Error>,`
- `pub fn prepare(&self, sql: &str) -> Result<Statement<'_>>`
- `pub fn prepare_with_flags( &self, sql: &str, flags: PrepFlags, ) -> Result<Statement<'_>>`
- `pub unsafe fn handle(&self) -> *mut sqlite3`
- `pub fn get_interrupt_handle(&self) -> InterruptHandle`
- `pub fn changes(&self) -> u64`
- `pub fn total_changes(&self) -> u64`
- `pub fn is_autocommit(&self) -> bool`
- `pub fn is_busy(&self) -> bool`
- `pub fn cache_flush(&self) -> Result<()>`

### struct rusqlite::Statement

```rust
pub struct Statement<'conn> { /* private fields */ }
```

A prepared statement.

**Methods**

- `pub fn column_names( &self, ) -> impl DoubleEndedIterator<Item = &str> + ExactSizeIterator`
- `pub fn column_count(&self) -> usize`
- `pub fn column_name(&self, col: usize) -> Result<&str>`
- `pub fn column_index(&self, name: &str) -> Result<usize>`
- `pub fn execute<P: Params>(&mut self, params: P) -> Result<usize>`
- `pub fn insert<P: Params>(&mut self, params: P) -> Result<i64>`
- `pub fn query<P: Params>(&mut self, params: P) -> Result<Rows<'_>>`
- `pub fn query_map<T, P, F>( &mut self, params: P, f: F, ) -> Result<MappedRows<'_, F>>where P: Params, F: FnMut(&Row<'_>) -> Result<T>,`
- `pub fn query_and_then<T, E, P, F>( &mut self, params: P, f: F, ) -> Result<AndThenRows<'_, F>>where P: Params, E: From<Error>, F: FnMut(&Row<'_>) -> Result<T, E>,`
- `pub fn exists<P: Params>(&mut self, params: P) -> Result<bool>`
- `pub fn query_row<T, P, F>(&mut self, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_one<T, P, F>(&mut self, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn finalize(self) -> Result<()>`
- `pub fn parameter_index(&self, name: &str) -> Result<Option<usize>>`
- `pub fn parameter_name(&self, index: usize) -> Option<&str>`
- `pub fn parameter_count(&self) -> usize`
- `pub fn raw_bind_parameter<I: BindIndex, T: ToSql>( &mut self, one_based_index: I, param: T, ) -> Result<()>`
- `pub fn raw_execute(&mut self) -> Result<usize>`
- `pub fn raw_query(&mut self) -> Rows<'_>`
- `pub fn expanded_sql(&self) -> Option<String>`
- `pub fn get_status(&self, status: StatementStatus) -> i32`
- `pub fn reset_status(&self, status: StatementStatus) -> i32`
- `pub fn is_explain(&self) -> i32`
- `pub fn readonly(&self) -> bool`
- `pub fn clear_bindings(&mut self)`
- `fn as_ref(&self) -> &Statement<'stmt>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn drop(&mut self)`
- `fn pin_drop(self: Pin<&mut Self>)`

### struct rusqlite::Transaction

```rust
pub struct Transaction<'conn> { /* private fields */ }
```

Represents a transaction on a database connection.

**Note**

Transactions will roll back by default. Use `commit` method to explicitly
commit the transaction, or use `set_drop_behavior` to change what happens
when the transaction is dropped.

**Example**

```rust
fn perform_queries(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    do_queries_part_1(&tx)?; // tx causes rollback if this fails
    do_queries_part_2(&tx)?; // tx causes rollback if this fails

    tx.commit()
}
```

**Methods**

- `pub fn new( conn: &mut Connection, behavior: TransactionBehavior, ) -> Result<Transaction<'_>>`
- `pub fn new_unchecked( conn: &Connection, behavior: TransactionBehavior, ) -> Result<Transaction<'_>>`
- `pub fn savepoint(&mut self) -> Result<Savepoint<'_>>`
- `pub fn savepoint_with_name<T: Into<String>>( &mut self, name: T, ) -> Result<Savepoint<'_>>`
- `pub fn drop_behavior(&self) -> DropBehavior`
- `pub fn set_drop_behavior(&mut self, drop_behavior: DropBehavior)`
- `pub fn commit(self) -> Result<()>`
- `pub fn rollback(self) -> Result<()>`
- `pub fn finish(self) -> Result<()>`
- `pub fn busy_timeout(&self, timeout: Duration) -> Result<()>`
- `pub fn column_exists<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<bool>`
- `pub fn table_exists<N: Name>( &self, db_name: Option<N>, table_name: N, ) -> Result<bool>`
- `pub fn column_metadata<N: Name>( &self, db_name: Option<N>, table_name: N, column_name: N, ) -> Result<(Option<CString>, Option<CString>, bool, bool, bool)>`
- `pub fn db_config(&self, config: DbConfig) -> Result<bool>`
- `pub fn set_db_config(&self, config: DbConfig, new_val: bool) -> Result<bool>`
- `pub fn pragma_query_one<T: FromSql>( &self, schema_name: Option<&str>, pragma_name: &str, ) -> Result<T>`
- `pub fn pragma_query_value<T, F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn pragma_query<F>( &self, schema_name: Option<&str>, pragma_name: &str, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>,`
- `pub fn pragma<F, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<()>where F: FnMut(&Row<'_>) -> Result<()>, V: ToSql,`
- `pub fn pragma_update<V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, ) -> Result<()>where V: ToSql,`
- `pub fn pragma_update_and_check<F, T, V>( &self, schema_name: Option<&str>, pragma_name: &str, pragma_value: V, f: F, ) -> Result<T>where F: FnOnce(&Row<'_>) -> Result<T>, V: ToSql,`
- `pub fn unchecked_transaction(&self) -> Result<Transaction<'_>>`
- `pub fn transaction_state<N: Name>( &self, db_name: Option<N>, ) -> Result<TransactionState>`
- `pub fn execute_batch(&self, sql: &str) -> Result<()>`
- `pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize>`
- `pub fn path(&self) -> Option<&str>`
- `pub fn release_memory(&self) -> Result<()>`
- `pub fn last_insert_rowid(&self) -> i64`
- `pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_one<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>where P: Params, F: FnOnce(&Row<'_>) -> Result<T>,`
- `pub fn query_row_and_then<T, E, P, F>( &self, sql: &str, params: P, f: F, ) -> Result<T, E>where P: Params, F: FnOnce(&Row<'_>) -> Result<T, E>, E: From<Error>,`
- `pub fn prepare(&self, sql: &str) -> Result<Statement<'_>>`
- `pub fn prepare_with_flags( &self, sql: &str, flags: PrepFlags, ) -> Result<Statement<'_>>`
- `pub unsafe fn handle(&self) -> *mut sqlite3`
- `pub fn get_interrupt_handle(&self) -> InterruptHandle`
- `pub fn changes(&self) -> u64`
- `pub fn total_changes(&self) -> u64`
- `pub fn is_autocommit(&self) -> bool`
- `pub fn is_busy(&self) -> bool`
- `pub fn cache_flush(&self) -> Result<()>`

### trait rusqlite::BindIndex

```rust
pub trait BindIndex: Sealed + Copy {
    // Required method
    fn idx(&self, stmt: &Statement<'_>) -> Result<usize>;
}
```

A trait implemented by types that can index into parameters of a statement.

It is only implemented for `usize` and `&str` and `&CStr`.

**Methods**

- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`
- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`
- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`
- `fn idx(&self, _: &Statement<'_>) -> Result<usize>`

### trait rusqlite::Name

```rust
pub trait Name: Debug + Copy {
    // Required method
    fn as_cstr(&self) -> Result<Named<'_>>;
}
```

Database, table, column, collation, function, module, vfs name

**Methods**

- `fn as_cstr(&self) -> Result<Named<'_>>`
- `fn as_cstr(&self) -> Result<Named<'_>>`
- `fn as_cstr(&self) -> Result<Named<'_>>`

### trait rusqlite::OptionalExtension

```rust
pub trait OptionalExtension<T> {
    // Required method
    fn optional(self) -> Result<Option<T>>;
}
```

See the method documentation.

**Methods**

- `fn optional(self) -> Result<Option<T>>`

### trait rusqlite::Params

```rust
pub trait Params: Sealed { }
```

Trait used for sets of parameter passed into SQL
statements/queries.

Note: Currently, this trait can only be implemented inside this crate.
Additionally, it’s methods (which are `doc(hidden)`) should currently not be
considered part of the stable API, although it’s possible they will
stabilize in the future.

**Passing parameters to SQLite**

Many functions in this library let you pass parameters to SQLite. Doing this
lets you avoid any risk of SQL injection, and is simpler than escaping
things manually. Aside from deprecated functions and a few helpers, this is
indicated by the function taking a generic argument that implements `Params`
(this trait).

**Positional parameters**

For cases where you want to pass a list of parameters where the number of
parameters is known at compile time, this can be done in one of the
following ways:

- 

For small lists of parameters up to 16 items, they may alternatively be
passed as a tuple, as in `thing.query((1, "foo"))`.
This is somewhat inconvenient for a single item, since you need a
weird-looking trailing comma: `thing.query(("example",))`. That case is
perhaps more cleanly expressed as `thing.query(["example"])`.

- 

Using the `rusqlite::params!` macro, e.g.
`thing.query(rusqlite::params![1, "foo", bar])`. This is mostly useful for
heterogeneous lists where the number of parameters greater than 16, or
homogeneous lists of parameters where the number of parameters exceeds 32.

- 

For small homogeneous lists of parameters, they can either be passed as:

- 

an array, as in `thing.query([1i32, 2, 3, 4])` or `thing.query(["foo", "b

...

### trait rusqlite::RowIndex

```rust
pub trait RowIndex: Sealed + Copy {
    // Required method
    fn idx(&self, stmt: &Statement<'_>) -> Result<usize>;
}
```

A trait implemented by types that can index into columns of a row.

It is only implemented for `usize` and `&str`.

**Methods**

- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`
- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`
- `fn idx(&self, stmt: &Statement<'_>) -> Result<usize>`

### type rusqlite::Result

```rust
pub type Result<T, E = Error> = Result<T, E>;
```

A typedef of the result returned by many methods.

**Methods**

- `fn optional(self) -> Result<Option<T>>`
