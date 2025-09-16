# `git-retag` — recreate a tag at `HEAD`

A tiny Rust CLI that **moves an existing Git tag to the current `HEAD`**, preserving the original **annotated tagger and message** when applicable.

---

## Install



```bash
cargo install git-re-tag
```

---

## Usage

```bash
git-re-tag <tag>
```

### Re-Create Pushed Tags

#### Bash / Zsh / Fish

```bash
git-retag v1.2.3 | bash
```

#### PowerShell

```powershell
iex (git-retag v1.2.3)
```

Or:

```powershell
git-retag v1.2.3 | ForEach-Object { iex $_ }
```
