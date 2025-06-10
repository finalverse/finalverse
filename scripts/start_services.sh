#!/bin/bash
# Simple wrapper around finalverse.sh start for backward compatibility
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/finalverse.sh" start "$@"
