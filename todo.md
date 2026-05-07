### Lazarus: 20-Year Archival Hardening Roadmap

**Goal:** Ensure that an archive created by Lazarus today can be successfully unpacked in 2046, completely offline, on unknown hardware, without relying on external servers, package managers, or modern operating systems.

#### Phase 1: The "Rosetta Stone" (Format Specification)
*If the executable fails and the source code won't compile, a future engineer must be able to write an unpacker from scratch.*
- [ ] **Write `FORMAT_SPEC.md`:** Document the exact binary layout of a Lazarus archive.
    - [ ] Define the Magic Bytes / Header structure.
    - [ ] Detail the block-size, chunking methodology, and metadata locations.
    - [ ] Document the exact hashing/fingerprinting algorithms used (e.g., SHA-256, BLAKE3) and where the hashes are stored.
    - [ ] Document the compression algorithms used (e.g., LZMA, Zstd) and the exact dictionary sizes.
- [ ] **Provide a "Manual Extraction" Guide:** Write a short explanation of how someone would extract the raw data using standard hex editors or basic scripting if Lazarus didn't exist.

#### Phase 2: Hermetic Builds (Zero-Network Compilation)
*The codebase must compile completely offline. If `crates.io` or GitHub are down in 15 years, the build must still succeed.*
- [ ] **Implement Dependency Vendoring:**
    - [ ] Run `cargo vendor` and commit the `vendor/` directory to the repository (or provide a release tarball that includes it).
    - [ ] Add `.cargo/config.toml` to point cargo to the local vendor directory.
- [ ] **Pin Dependencies:** Ensure `Cargo.lock` strictly pins every dependency version. Remove or replace dependencies that rely on downloading external C libraries during the build process (build scripts hitting the network are a death sentence for archives).
- [ ] **Remove/Replace Time-Bombs:** Audit dependencies for anything tied to modern web APIs, expiring certificates, or hardcoded dates.

#### Phase 3: The "Time Machine" (Environment Preservation)
*Rust 2046 might not compile Rust 2024. We must preserve the exact environment needed to build it.*
- [ ] **Create a `Dockerfile` for Building:** Provide a Dockerfile based on a stable, long-term support OS (e.g., Alpine or Debian LTS) that installs the exact version of `rustup` and the Rust toolchain used today.
- [ ] **Nix Flake (Optional but highly recommended):** Add a `flake.nix` for bit-for-bit reproducible builds. Nix is the absolute standard for ensuring an environment can be perfectly replicated decades later.

#### Phase 4: Extreme Portability (Static Linking)
*The compiled binary must carry its own "lungs" so it can run natively on future systems without missing shared libraries (like `glibc`).*
- [ ] **Linux Static Compilation:** Set up cross-compilation targets for `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` to generate 100% statically linked binaries.
- [ ] **Windows/macOS Portability:** Ensure the CI/CD pipeline spits out standalone `.exe` and universal macOS binaries without relying on complex external frameworks.
- [ ] **No-GUI Fallback:** Ensure that even if Lazarus eventually gets a graphical interface, the core packing/unpacking engine can always be run entirely from a basic command-line interface.

#### Phase 5: The "Capsule Generator" (Archival UX)
*Make it foolproof for the end-user creating the time capsule.*
- [ ] **Create an `--export-capsule` command/script:** When a user packs an archive, allow them to run a command that outputs a final folder containing:
    - [ ] The packed Lazarus archive `.lzr` data.
    - [ ] Pre-compiled static binaries for Windows, Linux, and macOS.
    - [ ] A ZIP of the vendored source code.
    - [ ] A `README_FUTURE.txt` explaining exactly what this folder is, how to use the binaries, and what to do if the binaries fail to run.
