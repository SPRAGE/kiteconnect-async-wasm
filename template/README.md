# Rust Library Template

This directory contains reusable GitHub workflows and helper scripts for managing
Rust crate releases. Copy the `.github` and `scripts` directories into a new
project and adjust the crate name via the `CRATE_NAME` environment variable in
`release.yml`.

## Quick start
1. Set the `CRATE_NAME` variable in `template/.github/workflows/release.yml` to
   match your crate.
2. Copy the contents of `template/.github` and `template/scripts` into your new
   repository.
3. Ensure the scripts have execute permission and commit them.

See `VERSION_MANAGEMENT.md` for detailed usage instructions.
