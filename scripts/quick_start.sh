#!/bin/bash
# AXON Quick Start Test Script

echo "╔══════════════════════════════════════╗"
echo "║   AXON Integration Test Script      ║"
echo "╚══════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Check Rust
echo -e "${YELLOW}[1/7]${NC} Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    echo -e "${GREEN}✓${NC} Rust $RUST_VERSION found"
else
    echo -e "${RED}✗${NC} Rust not found. Install from https://rustup.rs/"
    exit 1
fi

# Step 2: Check Cargo
echo -e "${YELLOW}[2/7]${NC} Checking Cargo..."
if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✓${NC} Cargo found"
else
    echo -e "${RED}✗${NC} Cargo not found"
    exit 1
fi

# Step 3: Check project structure
echo -e "${YELLOW}[3/7]${NC} Checking project structure..."
if [ -f "Cargo.toml" ]; then
    echo -e "${GREEN}✓${NC} Cargo.toml found"
else
    echo -e "${RED}✗${NC} Cargo.toml not found. Are you in the project root?"
    exit 1
fi

if [ -d "src" ]; then
    echo -e "${GREEN}✓${NC} src/ directory found"
else
    echo -e "${RED}✗${NC} src/ directory not found"
    exit 1
fi

# Step 4: Check config
echo -e "${YELLOW}[4/7]${NC} Checking config.toml..."
if [ -f "config.toml" ]; then
    echo -e "${GREEN}✓${NC} config.toml found"
    
    if grep -q "\[websocket\]" config.toml; then
        echo -e "${GREEN}✓${NC} WebSocket config present"
    else
        echo -e "${YELLOW}!${NC} WebSocket config missing. Adding..."
        cat >> config.toml << EOF

[websocket]
enabled = true
bind = "127.0.0.1"
port = 7878
EOF
        echo -e "${GREEN}✓${NC} WebSocket config added"
    fi
else
    echo -e "${YELLOW}!${NC} config.toml not found. Creating..."
    cat > config.toml << EOF
[websocket]
enabled = true
bind = "127.0.0.1"
port = 7878

[ollama]
endpoint = "http://localhost:11434"
default_model = "mistral:7b-q4"

[rag]
root_path = "."
exclude_patterns = ["target", "node_modules", ".git"]

[telegram]
enabled = false
bot_token = ""
chat_id = 0
EOF
    echo -e "${GREEN}✓${NC} config.toml created"
fi

# Step 5: Check Ollama (optional)
echo -e "${YELLOW}[5/7]${NC} Checking Ollama (optional for AI)..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Ollama is running"
    MODELS=$(curl -s http://localhost:11434/api/tags | grep -o '"name":"[^"]*"' | cut -d'"' -f4 | head -3 | tr '\n' ', ')
    echo -e "  Available models: ${MODELS%,}"
else
    echo -e "${YELLOW}!${NC} Ollama not running (AI chat will not work)"
    echo -e "  Install: https://ollama.ai/"
fi

# Step 6: Build check
echo -e "${YELLOW}[6/7]${NC} Checking if project compiles..."
echo "  (This may take a while on first run...)"
if cargo check --quiet 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Project compiles successfully"
else
    echo -e "${YELLOW}!${NC} Build issues detected. Run 'cargo check' for details"
fi

# Step 7: Port check
echo -e "${YELLOW}[7/7]${NC} Checking ports..."
if lsof -Pi :7878 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}!${NC} Port 7878 already in use"
    echo "  Kill process: kill \$(lsof -t -i:7878)"
else
    echo -e "${GREEN}✓${NC} Port 7878 available"
fi

if lsof -Pi :8000 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}!${NC} Port 8000 already in use"
else
    echo -e "${GREEN}✓${NC} Port 8000 available (for dashboard)"
fi

echo ""
echo "╔══════════════════════════════════════╗"
echo "║         Ready to Launch!             ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "Start AXON engine:"
echo -e "  ${GREEN}cargo run${NC}"
echo ""
echo "In another terminal, serve the dashboard:"
echo -e "  ${GREEN}cd dashboard && python3 -m http.server 8000${NC}"
echo ""
echo "Then open: ${GREEN}http://localhost:8000${NC}"
echo ""
echo "Check connection in browser console:"
echo "  Should see: ✅ Connected to AXON engine"
echo ""
