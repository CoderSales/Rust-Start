### Specifying dependencies from other registries

```bash
[dependencies]
some-crate = { version = "1.0", registry = "my-registry" }
```

### Specifying dependencies from git repositories

```bash
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

```bash
regex-lite   = { git = "https://github.com/rust-lang/regex.git" }
regex-syntax = { git = "https://github.com/rust-lang/regex.git" }
```

### Choice of commit

```bash
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git", branch = "next" }
```

### More git dependency examples:

```bash
# .git suffix can be omitted if the host accepts such URLs - both examples work the same
regex = { git = "https://github.com/rust-lang/regex" }
regex = { git = "https://github.com/rust-lang/regex.git" }

# a commit with a particular tag
regex = { git = "https://github.com/rust-lang/regex.git", tag = "1.10.3" }

# a commit by its SHA1 hash
regex = { git = "https://github.com/rust-lang/regex.git", rev = "0c0990399270277832fbb5b91a1fa118e6f63dba" }

# HEAD commit of PR 493
regex = { git = "https://github.com/rust-lang/regex.git", rev = "refs/pull/493/head" }

# INVALID EXAMPLES

# specifying the commit after # ignores the commit ID and generates a warning
regex = { git = "https://github.com/rust-lang/regex.git#4c59b70" }

# git and path cannot be used at the same time
regex = { git = "https://github.com/rust-lang/regex.git#4c59b70", path = "../regex" }
```

### The role of the version key

```bash
[dependencies]
regex = { version = "1.10.3", git = "https://github.com/rust-lang/regex.git", branch = "next" }
```

### Specifying path dependencies

```bash
# inside of hello_world/
$ cargo new hello_utils
```

```bash
[dependencies]
hello_utils = { path = "hello_utils" }
```

### No local path traversal

```bash
# git key accepts the repo root URL and Cargo traverses the tree to find the crate
[dependencies]
regex-lite   = { git = "https://github.com/rust-lang/regex.git" }
regex-syntax = { git = "https://github.com/rust-lang/regex.git" }

# path key requires the member name to be included in the local path
[dependencies]
regex-lite   = { path = "../regex/regex-lite" }
regex-syntax = { path = "../regex/regex-syntax" }
```

### Local paths in published crates

```bash
[dependencies]
hello_utils = { path = "hello_utils", version = "0.1.0" }
```

### Multiple locations

```bash
[dependencies]
# Uses `my-bitflags` when used locally, and uses
# version 1.0 from crates.io when published.
bitflags = { path = "my-bitflags", version = "1.0" }

# Uses the given git repo when used locally, and uses
# version 1.0 from crates.io when published.
smallvec = { git = "https://github.com/servo/rust-smallvec.git", version = "1.0" }

# N.B. that if a version doesn't match, Cargo will fail to compile!
```

### Platform specific dependencies

```bash
[target.'cfg(windows)'.dependencies]
winhttp = "0.4.0"

[target.'cfg(unix)'.dependencies]
openssl = "1.0.1"

[target.'cfg(target_arch = "x86")'.dependencies]
native-i686 = { path = "native/i686" }

[target.'cfg(target_arch = "x86_64")'.dependencies]
native-x86_64 = { path = "native/x86_64" }
```

```bash
[dependencies]
foo = { version = "1.0", optional = true }
bar = { version = "1.0", optional = true }

[features]
fancy-feature = ["foo", "bar"]
```

```bash
[target.x86_64-pc-windows-gnu.dependencies]
winhttp = "0.4.0"

[target.i686-unknown-linux-gnu.dependencies]
openssl = "1.0.1"
```

### Custom target specifications

```bash
[target.bar.dependencies]
winhttp = "0.4.0"

[target.my-special-i686-platform.dependencies]
openssl = "1.0.1"
native = { path = "native/i686" }
```

### Development dependencies

```bash
[dev-dependencies]
tempdir = "0.3"
```

```bash
[target.'cfg(unix)'.dev-dependencies]
mio = "0.0.1"
```

### Build dependencies

```bash
[build-dependencies]
cc = "1.0.3"
```

```bash
[target.'cfg(unix)'.build-dependencies]
cc = "1.0.3"
```

### Choosing features

```bash
[dependencies.awesome]
version = "1.3.5"
default-features = false # do not include the default features, and optionally
                         # cherry-pick individual features
features = ["secure-password", "civet"]
```

### Renaming dependencies in Cargo.toml

```bash
[package]
name = "mypackage"
version = "0.0.1"

[dependencies]
foo = "0.1"
bar = { git = "https://github.com/example/project.git", package = "foo" }
baz = { version = "0.1", registry = "custom", package = "foo" }
```

```bash
extern crate foo; // crates.io
extern crate bar; // git repository
extern crate baz; // registry `custom`
```

```bash
[dependencies]
bar = { version = "0.1", package = 'foo', optional = true }
```

```bash
[features]
log-debug = ['bar/log-debug'] # using 'foo/log-debug' would be an error!
```

### Inheriting a dependency from a workspace

```bash
[package]
name = "bar"
version = "0.2.0"

[dependencies]
regex = { workspace = true, features = ["unicode"] }

[build-dependencies]
cc.workspace = true

[dev-dependencies]
rand = { workspace = true, optional = true }
```
