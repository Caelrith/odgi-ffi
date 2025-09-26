# Changelog

All notable changes to this project will be documented in this file.

## [1.1.3] - 2025-09-24

### Fixed
- Removed a x86-64 specific compiler attribute from the vendored ODGI source code (`main.cpp`). This fix enables the crate to be successfully compiled and used on `arm64` architectures, such as Apple Silicon Macs.

## [1.1.0] - 2025-09-21

### Added
- `Graph::get_path_length(path_name)` to get the total length of a path in base pairs.
- `Graph::get_paths_on_edge(...)` to find all paths that traverse a specific directed edge.
- Comprehensive documentation and examples for the new functions.