#!/usr/bin/env bash
set -euo pipefail

# Reproducible build wrapper for Shade Protocol contracts.
# Usage: ./scripts/reproducible-build.sh [IMAGE_TAG]

IMAGE_TAG="${1:-shade-reproducible-build:latest}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Building reproducible builder image (${IMAGE_TAG})..."
docker build -f "${REPO_ROOT}/Dockerfile.reproducible" -t "${IMAGE_TAG}" "${REPO_ROOT}"

echo "Running deterministic contract build..."
docker run --rm \
  -v "${REPO_ROOT}:/workspace" \
  -w /workspace \
  "${IMAGE_TAG}"

echo ""
echo "Optimized WASMs and SHA256SUMS written to target/wasm32v1-none/release/"
