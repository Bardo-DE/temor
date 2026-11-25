#!/usr/bin/env bash
# Checks temor's [patch.crates-io] against bardo's, at the bardo tag pinned in Cargo.toml: every entry
# here is bardo's exactly, and every bardo entry for a crate temor's lock holds is here. bardo's own
# path entries (its vendored crates) are not for consumers. Bardo's root Cargo.toml is read from the
# cargo git checkout of that tag or, if cargo has not fetched it, from `git show` in $BARDO_REPO.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
bardo_repo="${BARDO_REPO:-$HOME/bardo}"
cargo_home="${CARGO_HOME:-$HOME/.cargo}"

tag=$(sed -n 's/.*git = "https:\/\/github\.com\/Bardo-DE\/bardo", tag = "\([^"]*\)".*/\1/p' "$root/Cargo.toml" | sort -u)
if [ "$(printf '%s\n' "$tag" | grep -c .)" != 1 ]; then
  echo "check-fork-patches: expected one bardo tag in $root/Cargo.toml, found: ${tag:-none}" >&2
  exit 2
fi
# The commit the lock resolved the tag to: `...bardo?tag=v0.1.2#5d17f077`.
commit=$(sed -n 's/^source = "git+https:\/\/github\.com\/Bardo-DE\/bardo?tag=[^#]*#\([0-9a-f]*\)"$/\1/p' "$root/Cargo.lock" | sort -u)
if [ "$(printf '%s\n' "$commit" | grep -c .)" != 1 ]; then
  echo "check-fork-patches: expected one bardo commit in $root/Cargo.lock, found: ${commit:-none}" >&2
  exit 2
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
bardo_manifest=""
for candidate in "$cargo_home"/git/checkouts/bardo-*/"${commit:0:7}"/Cargo.toml; do
  [ -f "$candidate" ] && { bardo_manifest=$candidate; break; }
done
if [ -z "$bardo_manifest" ]; then
  if ! git -C "$bardo_repo" show "$tag:Cargo.toml" >"$tmp/Cargo.toml" 2>/dev/null; then
    echo "check-fork-patches: bardo $tag is neither in the cargo git checkouts nor in $bardo_repo" >&2
    exit 2
  fi
  bardo_manifest="$tmp/Cargo.toml"
fi

echo "check-fork-patches: bardo $tag ($bardo_manifest)"
python3 - "$bardo_manifest" "$root/Cargo.toml" "$root/Cargo.lock" <<'EOF'
import sys, tomllib

bardo_path, temor_path, lock_path = sys.argv[1:]
with open(bardo_path, "rb") as f:
    bardo = tomllib.load(f).get("patch", {}).get("crates-io", {})
with open(temor_path, "rb") as f:
    temor = tomllib.load(f).get("patch", {}).get("crates-io", {})
with open(lock_path, "rb") as f:
    locked = {p["name"] for p in tomllib.load(f)["package"]}

forks = {name: spec for name, spec in bardo.items() if "path" not in spec}
wrong = [name for name, spec in temor.items() if forks.get(name) != spec]
missing = [name for name in forks if name in locked and name not in temor]
for name in sorted(wrong):
    print(f"  {name}: {temor[name]} here, {forks.get(name, 'no entry')} in bardo", file=sys.stderr)
for name in sorted(missing):
    print(f"  {name}: missing here, {forks[name]} in bardo", file=sys.stderr)
if wrong or missing:
    print("check-fork-patches: the [patch.crates-io] block differs from bardo's", file=sys.stderr)
    sys.exit(1)
print(f"check-fork-patches: {len(temor)} entries match bardo's")
EOF
