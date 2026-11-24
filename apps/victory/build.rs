//! Puts the git commit the binary is built from in `VICTORY_COMMIT`, so a
//! run's log names the exact source (with `-dirty` for uncommitted changes).

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

fn main() {
    // Rebuilt when HEAD moves, a branch gets a commit, or the index changes.
    // shortcut: an edit no `git add`/`git status` has seen leaves `-dirty`
    // as the last build script run found it; upgrade if that misleads.
    for path in ["HEAD", "index", "refs", "packed-refs"] {
        if let Some(p) = git(&["rev-parse", "--git-path", path]) {
            println!("cargo:rerun-if-changed={p}");
        }
    }
    let commit = git(&["describe", "--always", "--dirty", "--abbrev=12", "--exclude=*"])
        .unwrap_or_else(|| "unknown".into());
    let branch = git(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|| "?".into());
    println!("cargo:rustc-env=VICTORY_COMMIT={commit} ({branch})");
}
