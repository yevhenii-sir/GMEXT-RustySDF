#!/usr/bin/env bash
# ##### extgen :: user entrypoint (IfMissing — customize freely) #####
# Regenerated core lives in scripts/extgen/ — this wrapper is yours.
# Core deploys libRustySDF.so (matches System.loadLibrary("RustySDF") in RustySDFBridge).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$SCRIPT_DIR/extgen/build_android.sh" "$@"
