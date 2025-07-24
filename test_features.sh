#!/bin/bash

# Script di test per rFetch
# Testa tutte le funzionalità principali

echo "🧪 Test delle funzionalità di rFetch"
echo "===================================="
echo

# Test 1: Output normale
echo "📋 Test 1: Output normale"
./target/release/rfetch
echo

# Test 2: Lista temi
echo "🎨 Test 2: Lista temi disponibili"
./target/release/rfetch --list-themes
echo

# Test 3: Tema neon
echo "💫 Test 3: Tema neon"
./target/release/rfetch --theme neon
echo

# Test 4: Tema minimal
echo "🔹 Test 4: Tema minimal"
./target/release/rfetch --theme minimal
echo

# Test 5: Tema retro
echo "📺 Test 5: Tema retro"
./target/release/rfetch --theme retro
echo

# Test 6: Output JSON
echo "📄 Test 6: Output JSON"
./target/release/rfetch --json | head -20
echo "... (output troncato)"
echo

# Test 7: Logo piccolo
echo "🏷️  Test 7: Logo piccolo"
./target/release/rfetch --logo small
echo

# Test 8: Modalità verbose
echo "📊 Test 8: Modalità verbose"
./target/release/rfetch --verbose
echo

echo "✅ Tutti i test completati!"
echo "🚀 rFetch è pronto per l'uso!"