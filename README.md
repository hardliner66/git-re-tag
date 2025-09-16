# `git-re-tag` — recreate a tag at `HEAD`

A tiny Rust CLI that **moves an existing Git tag to the current `HEAD`**, preserving the original **annotated tagger and message** when applicable.

---

## Install



```bash
cargo install git-re-tag
```

---

## Usage

```bash
git re-tag <tag>
```

### Re-Create Pushed Tags

#### Bash / Zsh / Fish

```bash
git re-tag v1.2.3 | bash
```

#### PowerShell

```powershell
iex (git re-tag v1.2.3)
```

Or:

```powershell
git re-tag v1.2.3 | ForEach-Object { iex $_ }
```
