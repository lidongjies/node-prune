#!/bin/bash

# build, test and generate docs in this phase

set -ex

. "$(dirname $0)/utils.sh"

main() {
    # Test a normal debug build.
    if is_arm; then
        cargo build --target "$TARGET" --verbose
    else
        cargo build --target "$TARGET" --verbose --all --features 'pcre2'
    fi

    # Show the output of the most recent build.rs stderr.
    set +x
    stderr="$(find "target/$TARGET/debug" -name stderr -print0 | xargs -0 ls -t | head -n1)"
    if [ -s "$stderr" ]; then
      echo "===== $stderr ====="
      cat "$stderr"
      echo "====="
    fi
    set -x

    # sanity check the file type
    file target/"$TARGET"/debug/node-prune

    # Check that we've generated man page and other shell completions.
    outdir="$(cargo_out_dir "target/$TARGET/debug")"
    file "$outdir/node-prune.bash"
    file "$outdir/node-prune.fish"
    file "$outdir/_node-prune.ps1"
    file "$outdir/node-prune.1"

    # Apparently tests don't work on arm, so just bail now. I guess we provide
    # ARM releases on a best effort basis?
    if is_arm; then
      return 0
    fi

    # Test that zsh completions are in sync with node-prune's actual args.
    "$(dirname "${0}")/test_complete.sh"

    # Run tests for node-prune and all sub-crates.
    cargo test --target "$TARGET" --verbose --all --features 'pcre2'
}

main