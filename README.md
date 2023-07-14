# Installation

## 0. bin
- create directory `bin\` under `flux-test\`

## 1. z3
- `wget https://github.com/Z3Prover/z3/releases/download/z3-4.12.2/z3-4.12.2-x64-glibc-2.35.zip`
- move the `z3` binary to `bin/`

## 2. stack (haskell builder)
- `wget -qO- https://get.haskellstack.org/ | sh`

## 3. liquid-fixpoint
- Follow https://github.com/ucsd-progsys/liquid-fixpoint#how-to-build-and-install
- move the `fixpoint` binary to `bin/`

## 4. add $PATH to binaries
- `export PATH="<PATH_TO>/flux-test/bin:$PATH"`

## 5. flux
- Follow https://flux-rs.github.io/flux/guide/install.html *BUT*
  - Use `cargo install` with `--locked` flag
  - To use existing `Cargo.lock` file
  - As recomputing versions from `Cargo.toml` breaks dependency
