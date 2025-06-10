#!/bin/bash
# Wrapper around finalverse.sh stop for backward compatibility
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/finalverse.sh" stop "$@"
