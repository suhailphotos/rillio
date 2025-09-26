# rillio

**A pluggable, feature‑aware database façade: one clean CRUD API over multiple backends (Notion, Supabase, Postgres, SQLite), with a Rust core, CLI, and Python wrapper.**

> Status: name‑reservation / pre‑alpha. This repo initially ships a “hello world” so the names on crates.io, PyPI, and (optionally) npm are secured. The real code and workspace layout will follow.

---

## Why rillio?

You often need the *same* data operations across very different storage systems (HTTP APIs like Notion/Supabase and SQL engines like Postgres/SQLite). rillio aims to provide:

- **One façade**: a single async CRUD interface.
- **Multiple adapters**: backends implement a common trait (Strategy/Adapter pattern).
- **Capability negotiation**: opt into extras (transactions, rich text, relations).
- **Multiple frontends**: use it as a Rust library, a CLI, or from Python.
- **Feature flags**: compile only what you need.

This README focuses on **locking names** (publish tiny placeholders) and the high‑level plan. The full workspace will come right after.


## Quick goals

- Reserve package names:
  - crates.io: `rillio`
  - PyPI: `rillio`
  - npm (optional): `@suhailphotos/rillio` (recommended) or try to grab `rillio` if available
- Spin up a public GitHub repo at `$MATRIX/crates/rillio`
- Keep the initial code minimal (“hello world”) so publishing is trivial


---

## Step‑by‑step: bootstrap + publish placeholders

> Assumptions: macOS/Linux, `gh`, `git`, `cargo`, `python3`, `pipx` (optional), `twine`, `build`, and `npm` installed. You already have your `truss` helpers checked out.

### 0) Sanity‑check name availability (optional but nice)

You already have a helper:

```bash
cd ~/Library/CloudStorage/Dropbox/matrix/truss
scripts/names/namecheck.sh -r all rillio
```

If `rillio` is taken on npm, prefer the scoped form `@suhailphotos/rillio`.


### 1) Create the GitHub repo and local checkout

We’ll seed it with this README and a tiny Rust bin so `cargo publish` is possible.

```bash
# Save this README locally (if you haven’t already)
# Then run bootstrap (using your generic script)
./scripts/bootstrap/bootstrap_repo_generic.sh   -u suhailphotos   -r rillio   -t rust   -d "Pluggable, feature-aware database façade with Rust core, CLI, and Python bindings"   -R "/Users/suhail/Documents/Scratch/notes/README.md"   -O "op://devTools/GitHub Repo Key/secret key"   -V public
```

After this, your repo lives at:

```
$MATRIX/crates/rillio
```

The rust scaffold will have a minimal `Cargo.toml` and `src/main.rs`.


### 2) Prepare the crate for crates.io (hello world)

Edit `Cargo.toml` to include the metadata crates.io expects:

```toml
[package]
name = "rillio"
version = "0.0.1"
edition = "2021"
description = "Pluggable, feature-aware database façade with Rust core, CLI, and Python bindings"
license = "MIT"
readme = "README.md"
homepage = "https://github.com/suhailphotos/rillio"
repository = "https://github.com/suhailphotos/rillio"
keywords = ["database", "adapter", "facade", "notion", "postgres", "sqlite"]
categories = ["database", "command-line-utilities"]
```

Keep `src/main.rs` extremely simple for now:

```rust
fn main() {
    println!("rillio: hello world (name reservation)");
}
```

Now publish:

```bash
# login once (opens browser to create a token)
cargo login
# dry-run the package to verify
cargo package
# publish for real
cargo publish
```

> If you see “package name is taken,” stop and pick an alternate spelling or add a suffix.


### 3) Publish a tiny Python package on PyPI (name reservation)

Do this **inside the same repo** in a `python/` subfolder (keeps things tidy for now).

```bash
cd $MATRIX/crates/rillio
mkdir -p python/src/rillio
cat > python/src/rillio/__init__.py <<'PY'
__all__ = ["hello"]
__version__ = "0.0.1"

def hello() -> str:
    return "rillio: hello world (name reservation)"
PY

cat > python/pyproject.toml <<'TOML'
[project]
name = "rillio"
version = "0.0.1"
description = "Pluggable, feature-aware database façade (name reservation)"
readme = "../README.md"
requires-python = ">=3.10"
authors = [{name = "Suhail"}]
classifiers = [
  "Programming Language :: Python :: 3",
  "License :: OSI Approved :: MIT License",
]

[build-system]
requires = ["setuptools>=68", "wheel"]
build-backend = "setuptools.build_meta"

[tool.setuptools]
package-dir = {"" = "src"}
TOML

# build + upload
python3 -m pip install --upgrade build twine
cd python
python3 -m build
python3 -m twine upload dist/*
```

You’ll be prompted for PyPI credentials or a token. Consider setting up an API token on PyPI and using a `~/.pypirc` for convenience later.


### 4) (Optional) Publish an npm placeholder

If unscoped `rillio` is free, you can try it; otherwise use your scope `@suhailphotos/rillio` which is safer.

```bash
cd $MATRIX/crates/rillio
mkdir -p npm
cat > npm/package.json <<'JSON'
{
  "name": "@suhailphotos/rillio",
  "version": "0.0.1",
  "description": "Pluggable, feature-aware database façade (name reservation)",
  "license": "MIT",
  "repository": "github:suhailphotos/rillio",
  "homepage": "https://github.com/suhailphotos/rillio",
  "type": "module",
  "main": "index.js"
}
JSON

echo 'export const hello = () => "rillio: hello world (name reservation)";' > npm/index.js

cd npm
npm login            # if not already
npm publish --access public
```

If you later want to switch from scoped to unscoped (or vice‑versa), you can publish a new package; npm doesn’t allow renaming a published package.


---

## After the names are locked

We’ll replace the placeholder with the real multi‑crate workspace:

```
rillio/
├─ Cargo.toml        # workspace
├─ crates/
│  ├─ core/          # façade traits, types, errors, factory
│  ├─ backend-notion/
│  ├─ backend-supabase/
│  ├─ backend-postgres/
│  ├─ backend-sqlite/
│  └─ cli/
├─ py/               # pyo3 wrapper (published as rillio on PyPI after migration)
└─ README.md
```

Milestones:
1) Implement `core` with a mock backend (compiles, no deps).  
2) Fill in SQLite adapter with `sqlx` for first real E2E.  
3) Add Notion adapter (HTTP + rate‑limit handling).  
4) Add Supabase, then Postgres.  
5) Expose Python bindings via `pyo3` and ship CLI commands.  
6) Split docs into `docs/` and add examples + tests.


## License

MIT © Suhail

