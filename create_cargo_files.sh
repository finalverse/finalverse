#!/bin/bash
# create_cargo_files.sh - Create all necessary Cargo.toml files

echo "Creating Cargo.toml files for Finalverse MVP..."

# Create libs/common/Cargo.toml
cat > libs/common/Cargo.toml << 'EOF'
[package]
name = "finalverse-common"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
uuid.workspace = true
chrono.workspace = true
thiserror.workspace = true
EOF

# Create libs/protocol/Cargo.toml
cat > libs/protocol/Cargo.toml << 'EOF'
[package]
name = "finalverse-protocol"
version.workspace = true
edition.workspace = true

[dependencies]
finalverse-common = { path = "../common" }
serde.workspace = true
async-trait.workspace = true
tonic.workspace = true
tokio.workspace = true
EOF

# Create services/song-engine/Cargo.toml
cat > services/song-engine/Cargo.toml << 'EOF'
[package]
name = "song-engine"
version.workspace = true
edition.workspace = true

[[bin]]
name = "song-engine"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
EOF

# Create services/world-engine/Cargo.toml
cat > services/world-engine/Cargo.toml << 'EOF'
[package]
name = "world-engine"
version.workspace = true
edition.workspace = true

[[bin]]
name = "world-engine"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
EOF

# Create services/echo-engine/Cargo.toml
cat > services/echo-engine/Cargo.toml << 'EOF'
[package]
name = "echo-engine"
version.workspace = true
edition.workspace = true

[[bin]]
name = "echo-engine"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
EOF

# Create services/ai-orchestra/Cargo.toml
cat > services/ai-orchestra/Cargo.toml << 'EOF'
[package]
name = "ai-orchestra"
version.workspace = true
edition.workspace = true

[[bin]]
name = "ai-orchestra"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
axum.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
rand.workspace = true
EOF

# Create client/mock-client/Cargo.toml
cat > client/mock-client/Cargo.toml << 'EOF'
[package]
name = "mock-client"
version.workspace = true
edition.workspace = true

[[bin]]
name = "mock-client"
path = "src/main.rs"

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
reqwest = { version = "0.11", features = ["json"] }
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
anyhow.workspace = true
EOF

# Create empty services that are in the workspace but not implemented yet
services=(
    "api-gateway"
    "harmony-service"
    "silence-service"
    "community-service"
    "asset-service"
    "procedural-gen"
    "behavior-ai"
)

for service in "${services[@]}"; do
    mkdir -p "services/$service/src"
    
    # Create Cargo.toml
    cat > "services/$service/Cargo.toml" << EOF
[package]
name = "$service"
version.workspace = true
edition.workspace = true

[dependencies]
finalverse-common = { path = "../../libs/common" }
finalverse-protocol = { path = "../../libs/protocol" }
EOF

    # Create placeholder main.rs
    cat > "services/$service/src/main.rs" << EOF
fn main() {
    println!("$service - Not implemented yet");
}
EOF
done

# Create libs/ai-common
mkdir -p libs/ai-common/src
cat > libs/ai-common/Cargo.toml << 'EOF'
[package]
name = "finalverse-ai-common"
version.workspace = true
edition.workspace = true

[dependencies]
EOF

cat > libs/ai-common/src/lib.rs << 'EOF'
// AI common library - placeholder
EOF

echo "✅ All Cargo.toml files created successfully!"