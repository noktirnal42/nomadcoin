#!/bin/bash

echo "🔍 Checking Pi3 Build Status"
echo "============================"
echo ""

# Check if build is running
if ssh noktirnal@pi3.local "pgrep -f 'cargo build'" > /dev/null 2>&1; then
    echo "⏳ Build IN PROGRESS on Pi3"
    echo ""
    echo "Showing build progress..."
    ssh noktirnal@pi3.local "ps aux | grep -E 'cargo|rustc' | grep -v grep" | head -5
    echo ""
    
    # Show disk usage
    echo "💾 Disk Usage on Pi:"
    ssh noktirnal@pi3.local "df -h | grep -E 'Filesystem|/$'" || true
else
    echo "Build may be complete or waiting..."
    echo ""
    
    # Check if binary exists
    if ssh noktirnal@pi3.local "test -f ~/nomadcoin/target/release/nomadcoin"; then
        echo "✅ Binary exists - build complete!"
        echo ""
        echo "Binary info:"
        ssh noktirnal@pi3.local "file ~/nomadcoin/target/release/nomadcoin"
        ssh noktirnal@pi3.local "ls -lh ~/nomadcoin/target/release/nomadcoin"
    else
        echo "⏳ Build still in progress..."
        echo "   Try again in a few minutes"
    fi
fi

echo ""
echo "📌 Run this script again to check progress"
