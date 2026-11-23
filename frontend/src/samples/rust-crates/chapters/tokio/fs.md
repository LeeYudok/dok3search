# Module tokio::fs

## Overview

Asynchronous file utilities.

This module contains utility methods for working with the file system
asynchronously. This includes reading/writing to files, and working with
directories.

Be aware that most operating systems do not provide asynchronous file system
APIs. Because of that, Tokio will use ordinary blocking file operations
behind the scenes. This is done using the `spawn_blocking` threadpool to
run them in the background.

The `tokio::fs` module should only be used for ordinary files. Trying to use
it with e.g., a named pipe on Linux can result in surprising behavior,
such as hangs during runtime shutdown. For special files, you should use a
dedicated type such as `tokio::net::unix::pipe` or `AsyncFd` instead.

Tokio currently uses `spawn_blocking` on all platforms, but also uses
io_uring for file operations on Linux when compiled with `tokio_unstable`.
Operations that cannot be cancelled with `spawn_blocking` may possibly
be cancelled with io_uring.

**Cancellation**

Cancelling a future from this module will stop waiting for the result, but
the underlying blocking operation will continue to run on the thread pool.
For example, cancelling a `write()` future after it has been
polled will still result in the data being written to disk.

**Usage**

The easiest way to use this module is to use the utility functions that
operate on entire files:

- `tokio::fs::read`

- `tokio::fs::read_to_string`

- `tokio::fs::write`

The two `read` functions reads the entire file and returns its contents.
The `write` function takes the contents of the file and writes those
contents to the file. It overwrites the existing file, if any.

For example, to read the file:

```rust
let contents = tokio::fs::read_to_string("my_file.txt").await?;

println!("File has {} lines.", contents.lines().count());
```

To overwrite the file:

```rust
let contents = "First line.\nSecond line.\nThird line.\n";

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
```

**Using `File`**

The main type for interacting with files is `File`. It can be used to read
from and write to a given file. This is done using the `AsyncRead` and
`AsyncWrite` traits. This type is generally used when you want to do
something more complex than just reading or writing the entire contents in
one go.

Note: It is important to use `flush` when writing to a Tokio
`File`. This is because calls to `write`

...

## Items

- `DirBuilder` — A builder for creating directories in various manners.
- `DirEntry` — Entries returned by the ReadDir stream.
- `File` — A reference to an open file on the filesystem.
- `FileTimes` — Timestamps for a file.
- `OpenOptions` — Options and flags which can be used to configure how a file is opened.
- `ReadDir` — Reads the entries in a directory.
- `canonicalize` — Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
- `copy` — Copies the contents of one file to another. This function will also copy the permission bits of the original file to the destination file. This function will overwrite the contents of to.
- `create_dir` — Creates a new, empty directory at the provided path.
- `create_dir_all` — Recursively creates a directory and all of its parent components if they are missing.
- `hard_link` — Creates a new hard link on the filesystem.
- `metadata` — Given a path, queries the file system to get information about a file, directory, etc.
- `read` — Reads the entire contents of a file into a bytes vector.
- `read_dir` — Returns a stream over the entries within a directory.
- `read_link` — Reads a symbolic link, returning the file that the link points to.
- `read_to_string` — Creates a future which will open a file for reading and read the entire contents into a string and return said string.
- `remove_dir` — Removes an existing, empty directory.
- `remove_dir_all` — Removes a directory at this path, after removing all its contents. Use carefully!
- `remove_file` — Removes a file from the filesystem.
- `rename` — Renames a file or directory to a new name, replacing the original file if to already exists.
- `set_permissions` — Changes the permissions found on a file or a directory.
- `symlink` — Creates a new symbolic link on the filesystem.
- `symlink_metadata` — Queries the file system metadata for a path.
- `try_exists` — Returns Ok(true) if the path points at an existing entity.
- `write` — Creates a future that will open a file for writing and write the entire contents of contents to it.

### fn tokio::fs::canonicalize

```rust
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf>
```

Returns the canonical, absolute form of a path with all intermediate
components normalized and symbolic links resolved.

This is an async version of `std::fs::canonicalize`.

**Platform-specific behavior**

This function currently corresponds to the `realpath` function on Unix
and the `CreateFile` and `GetFinalPathNameByHandle` functions on Windows.
Note that, this may change in the future.

On Windows, this converts the path to use extended length path
syntax, which allows your program to use longer path names, but means you
can only join backslash-delimited paths to it, and it may be incompatible
with other applications (if passed to the application on the command-line,
or written to a file another application may read).

**Errors**

This function will return an error in the following situations, but is not
limited to just these cases:

- `path` does not exist.

- A non-final component in path is not a directory.

**Examples**

```rust
use tokio::fs;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = fs::canonicalize("../a/../foo.txt").await?;
    Ok(())
}
```

### fn tokio::fs::copy

```rust
pub async fn copy(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<u64, Error>
```

Copies the contents of one file to another. This function will also copy the permission bits
of the original file to the destination file.
This function will overwrite the contents of to.

This is the async equivalent of `std::fs::copy`.

**Examples**

```rust
use tokio::fs;

fs::copy("foo.txt", "bar.txt").await?;
```

### fn tokio::fs::create_dir

```rust
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()>
```

Creates a new, empty directory at the provided path.

This is an async version of `std::fs::create_dir`.

**Platform-specific behavior**

This function currently corresponds to the `mkdir` function on Unix
and the `CreateDirectory` function on Windows.
Note that, this may change in the future.

NOTE: If a parent of the given path doesn’t exist, this function will
return an error. To create a directory and all its missing parents at the
same time, use the `create_dir_all` function.

**Errors**

This function will return an error in the following situations, but is not
limited to just these cases:

- User lacks permissions to create directory at `path`.

- A parent of the given path doesn’t exist. (To create a directory and all
its missing parents at the same time, use the `create_dir_all`
function.)

- `path` already exists.

**Examples**

```rust
use tokio::fs;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    fs::create_dir("/some/dir").await?;
    Ok(())
}
```

### fn tokio::fs::create_dir_all

```rust
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()>
```

Recursively creates a directory and all of its parent components if they
are missing.

This is an async version of `std::fs::create_dir_all`.

**Platform-specific behavior**

This function currently corresponds to the `mkdir` function on Unix
and the `CreateDirectory` function on Windows.
Note that, this may change in the future.

**Errors**

This function will return an error in the following situations, but is not
limited to just these cases:

- If any directory in the path specified by `path` does not already exist
and it could not be created otherwise. The specific error conditions for
when a directory is being created (after it is determined to not exist) are
outlined by `fs::create_dir`.

If the final component at `path` already exists and is a directory, this
function will also return successfully without error.

Notable exception is made for situations where any of the directories
specified in the `path` could not be created as it was being created concurrently.
Such cases are considered to be successful. That is, calling `create_dir_all`
concurrently from multiple threads or processes is guaranteed not to fail
due to a race condition with itself.

**Examples**

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all("/some/dir").await?;
    Ok(())
}
```

### fn tokio::fs::hard_link

```rust
pub async fn hard_link(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Result<()>
```

Creates a new hard link on the filesystem.

This is an async version of `std::fs::hard_link`.

The `link` path will be a link pointing to the `original` path. Note that systems
often require these two paths to both be located on the same filesystem.

**Platform-specific behavior**

This function currently corresponds to the `link` function on Unix
and the `CreateHardLink` function on Windows.
Note that, this may change in the future.

**Errors**

This function will return an error in the following situations, but is not
limited to just these cases:

- The `original` path is not a file or doesn’t exist.

**Examples**

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    fs::hard_link("a.txt", "b.txt").await?; // Hard link a.txt to b.txt
    Ok(())
}
```

### fn tokio::fs::metadata

```rust
pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata>
```

Given a path, queries the file system to get information about a file,
directory, etc.

This is an async version of `std::fs::metadata`.

This function will traverse symbolic links to query information about the
destination file.

**Platform-specific behavior**

This function currently corresponds to the `stat` function on Unix and the
`GetFileAttributesEx` function on Windows.  Note that, this may change in
the future.

**Errors**

This function will return an error in the following situations, but is not
limited to just these cases:

- The user lacks permissions to perform `metadata` call on `path`.

- `path` does not exist.

**Examples**

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let attr = fs::metadata("/some/file/path.txt").await?;
    // inspect attr ...
    Ok(())
}
```

### fn tokio::fs::read

```rust
pub async fn read(path: impl AsRef<Path>) -> Result<Vec<u8>>
```

Reads the entire contents of a file into a bytes vector.

This is an async version of `std::fs::read`.

This is a convenience function for using `File::open` and `read_to_end`
with fewer imports and without an intermediate variable. It pre-allocates a
buffer based on the file size when available, so it is generally faster than
reading into a vector created with `Vec::new()`.

This operation is implemented by running the equivalent blocking operation
on a separate thread pool using `spawn_blocking`.

**Errors**

This function will return an error if `path` does not already exist.
Other errors may also be returned according to `OpenOptions::open`.

It will also return an error if it encounters while reading an error
of a kind other than `ErrorKind::Interrupted`.

**io_uring support**

On Linux, you can also use io_uring for executing system calls. To enable
io_uring, you need to specify the `--cfg tokio_unstable` flag at compile time,
enable the io-uring cargo feature, and set the `Builder::enable_io_uring`
runtime option.

Support for io_uring is currently experimental, so its behavior may change
or it may be removed in future versions.

**Examples**

```rust
use tokio::fs;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let contents = fs::read("address.txt").await?;
    let foo: SocketAddr = String::from_utf8_lossy(&contents).parse()?;
    Ok(())
}
```

### fn tokio::fs::read_dir

```rust
pub async fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir>
```

Returns a stream over the entries within a directory.

This is an async version of `std::fs::read_dir`.

This operation is implemented by running the equivalent blocking
operation on a separate thread pool using `spawn_blocking`.

### fn tokio::fs::read_link

```rust
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf>
```

Reads a symbolic link, returning the file that the link points to.

This is an async version of `std::fs::read_link`.

### fn tokio::fs::read_to_string

```rust
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String>
```

Creates a future which will open a file for reading and read the entire
contents into a string and return said string.

This is the async equivalent of `std::fs::read_to_string`.

This operation is implemented by running the equivalent blocking operation
on a separate thread pool using `spawn_blocking`.

**Examples**

```rust
use tokio::fs;

let contents = fs::read_to_string("foo.txt").await?;
println!("foo.txt contains {} bytes", contents.len());
```

### fn tokio::fs::remove_dir

```rust
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()>
```

Removes an existing, empty directory.

This is an async version of `std::fs::remove_dir`.

### fn tokio::fs::remove_dir_all

```rust
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()>
```

Removes a directory at this path, after removing all its contents. Use carefully!

This is an async version of `std::fs::remove_dir_all`

### fn tokio::fs::remove_file

```rust
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()>
```

Removes a file from the filesystem.

Note that there is no guarantee that the file is immediately deleted (e.g.
depending on platform, other open file descriptors may prevent immediate
removal).

This is an async version of `std::fs::remove_file`.

### fn tokio::fs::rename

```rust
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()>
```

Renames a file or directory to a new name, replacing the original file if
`to` already exists.

This will not work if the new name is on a different mount point.

This is an async version of `std::fs::rename`.

### fn tokio::fs::set_permissions

```rust
pub async fn set_permissions(
    path: impl AsRef<Path>,
    perm: Permissions,
) -> Result<()>
```

Changes the permissions found on a file or a directory.

This is an async version of `std::fs::set_permissions`

### fn tokio::fs::symlink

```rust
pub async fn symlink(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> Result<()>
```

Creates a new symbolic link on the filesystem.

The `link` path will be a symbolic link pointing to the `original` path.

This is an async version of `std::os::unix::fs::symlink`.

### fn tokio::fs::symlink_metadata

```rust
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata>
```

Queries the file system metadata for a path.

This is an async version of `std::fs::symlink_metadata`

### fn tokio::fs::try_exists

```rust
pub async fn try_exists(path: impl AsRef<Path>) -> Result<bool>
```

Returns `Ok(true)` if the path points at an existing entity.

This function will traverse symbolic links to query information about the
destination file. In case of broken symbolic links this will return `Ok(false)`.

This is the async equivalent of `std::path::Path::try_exists`.

**Examples**

```rust
use tokio::fs;

fs::try_exists("foo.txt").await?;
```

### fn tokio::fs::write

```rust
pub async fn write(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> Result<()>
```

Creates a future that will open a file for writing and write the entire
contents of `contents` to it.

This is the async equivalent of `std::fs::write`.

This operation is implemented by running the equivalent blocking operation
on a separate thread pool using `spawn_blocking`.

**Examples**

```rust
use tokio::fs;

fs::write("foo.txt", b"Hello world!").await?;
```

### struct tokio::fs::DirBuilder

```rust
pub struct DirBuilder { /* private fields */ }
```

A builder for creating directories in various manners.

This is a specialized version of `std::fs::DirBuilder` for usage on
the Tokio runtime.

**Methods**

- `pub fn new() -> Self`
- `pub fn recursive(&mut self, recursive: bool) -> &mut Self`
- `pub async fn create(&self, path: impl AsRef<Path>) -> Result<()>`
- `pub fn mode(&mut self, mode: u32) -> &mut Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tokio::fs::DirEntry

```rust
pub struct DirEntry { /* private fields */ }
```

Entries returned by the `ReadDir` stream.

This is a specialized version of `std::fs::DirEntry` for usage from the
Tokio runtime.

An instance of `DirEntry` represents an entry inside of a directory on the
filesystem. Each entry can be inspected via methods to learn about the full
path or possibly other metadata through per-platform extension traits.

**Methods**

- `pub fn ino(&self) -> u64`
- `pub fn path(&self) -> PathBuf`
- `pub fn file_name(&self) -> OsString`
- `pub async fn metadata(&self) -> Result<Metadata>`
- `pub async fn file_type(&self) -> Result<FileType>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`

### struct tokio::fs::File

```rust
pub struct File { /* private fields */ }
```

A reference to an open file on the filesystem.

This is a specialized version of `std::fs::File` for usage from the
Tokio runtime.

An instance of a `File` can be read and/or written depending on what options
it was opened with. Files also implement `AsyncSeek` to alter the logical
cursor that the file contains internally.

A file will not be closed immediately when it goes out of scope if there
are any IO operations that have not yet completed. To ensure that a file is
closed immediately when it is dropped, you should call `flush` before
dropping it. Note that this does not ensure that the file has been fully
written to disk; the operating system might keep the changes around in an
in-memory buffer. See the `sync_all` method for telling the OS to write
the data to disk.

Reading and writing to a `File` is usually done using the convenience
methods found on the `AsyncReadExt` and `AsyncWriteExt` traits.

**Examples**

Create a new file and asynchronously write bytes to it:

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()

let mut file = File::create("foo.txt").await?;
file.write_all(b"hello, world!").await?;
```

Read the contents of a file into a buffer:

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

let mut file = File::open("foo.txt").await?;

let mut contents = vec![];
file.read_to_end(&mut contents).await?;

println!("len = {}", contents.len());
```

**Methods**

- `pub async fn open(path: impl AsRef<Path>) -> Result<File>`
- `pub async fn create(path: impl AsRef<Path>) -> Result<File>`
- `pub async fn create_new<P: AsRef<Path>>(path: P) -> Result<File>`
- `pub fn options() -> OpenOptions`
- `pub fn from_std(std: StdFile) -> File`
- `pub async fn sync_all(&self) -> Result<()>`
- `pub async fn sync_data(&self) -> Result<()>`
- `pub async fn set_len(&self, size: u64) -> Result<()>`
- `pub async fn metadata(&self) -> Result<Metadata>`
- `pub async fn try_clone(&self) -> Result<File>`
- `pub async fn into_std(self) -> StdFile`
- `pub fn try_into_std(self) -> Result<StdFile, Self>`
- `pub async fn set_permissions(&self, perm: Permissions) -> Result<()>`
- `pub async fn set_times(&self, times: FileTimes) -> Result<()>`
- `pub async fn set_modified(&self, time: SystemTime) -> Result<()>`
- `pub fn set_max_buf_size(&mut self, max_buf_size: usize)`
- `pub fn max_buf_size(&self) -> usize`
- `fn as_fd(&self) -> BorrowedFd<'_>`
- `fn as_raw_fd(&self) -> RawFd`
- `fn poll_read( self: Pin<&mut Self>, cx: &mut Context<'_>, dst: &mut ReadBuf<'_>, ) -> Poll<Result<()>>`
- `fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> Result<()>`
- `fn poll_complete( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<u64>>`
- `fn poll_write( self: Pin<&mut Self>, cx: &mut Context<'_>, src: &[u8], ) -> Poll<Result<usize>>`
- `fn poll_write_vectored( self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>], ) -> Poll<Result<usize, Error>>`
- `fn is_write_vectored(&self) -> bool`
- `fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn poll_shutdown( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<(), Error>>`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `fn from(std: StdFile) -> Self`
- `fn from(fd: OwnedFd) -> Self`
- `unsafe fn from_raw_fd(fd: RawFd) -> Self`

### struct tokio::fs::FileTimes

```rust
pub struct FileTimes { /* private fields */ }
```

Timestamps for a file.

This is a specialized version of `std::fs::FileTimes` for usage with
`File::set_times`. Unlike the standard library version,
this type records the timestamps itself, so a future `io-uring` file
implementation can consume them without converting through the standard
library type.

**Examples**

```rust
use tokio::fs::FileTimes;

let times = FileTimes::new()
    .set_accessed(std::time::SystemTime::now())
    .set_modified(std::time::SystemTime::now());
```

**Methods**

- `pub fn new() -> Self`
- `pub fn set_accessed(self, t: SystemTime) -> Self`
- `pub fn set_modified(self, t: SystemTime) -> Self`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct tokio::fs::OpenOptions

```rust
pub struct OpenOptions { /* private fields */ }
```

Options and flags which can be used to configure how a file is opened.

This builder exposes the ability to configure how a `File` is opened and
what operations are permitted on the open file. The `File::open` and
`File::create` methods are aliases for commonly used options using this
builder.

Generally speaking, when using `OpenOptions`, you’ll first call `new`,
then chain calls to methods to set each option, then call `open`, passing
the path of the file you’re trying to open. This will give you a
`io::Result` with a `File` inside that you can further operate
on.

This is a specialized version of `std::fs::OpenOptions` for usage from
the Tokio runtime.

`From<std::fs::OpenOptions>` is implemented for more advanced configuration
than the methods provided here.

**Examples**

Opening a file to read:

```rust
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

Opening a file for both reading and writing, as well as creating it if it
doesn’t exist:

```rust
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

**Methods**

- `pub fn new() -> OpenOptions`
- `pub fn read(&mut self, read: bool) -> &mut OpenOptions`
- `pub fn write(&mut self, write: bool) -> &mut OpenOptions`
- `pub fn append(&mut self, append: bool) -> &mut OpenOptions`
- `pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions`
- `pub fn create(&mut self, create: bool) -> &mut OpenOptions`
- `pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions`
- `pub async fn open(&self, path: impl AsRef<Path>) -> Result<File>`
- `pub fn mode(&mut self, mode: u32) -> &mut OpenOptions`
- `pub fn custom_flags(&mut self, flags: i32) -> &mut OpenOptions`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn from(options: StdOpenOptions) -> OpenOptions`

### struct tokio::fs::ReadDir

```rust
pub struct ReadDir(/* private fields */);
```

Reads the entries in a directory.

This struct is returned from the `read_dir` function of this module and
will yield instances of `DirEntry`. Through a `DirEntry` information
like the entry’s path and possibly other metadata can be learned.

A `ReadDir` can be turned into a `Stream` with `ReadDirStream`.

**Errors**

This stream will return an `Err` if there’s some sort of intermittent
IO error during iteration.

**Methods**

- `pub async fn next_entry(&mut self) -> Result<Option<DirEntry>>`
- `pub fn poll_next_entry( &mut self, cx: &mut Context<'_>, ) -> Poll<Result<Option<DirEntry>>>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
