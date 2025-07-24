#!/bin/bash

# Test script per verificare il logo di Termux
echo "Testing Termux logo display..."

# Simula l'ambiente Termux impostando le variabili d'ambiente
export TERMUX_VERSION="0.118.0"
export PREFIX="/data/data/com.termux/files/usr"

echo "Environment variables set:"
echo "TERMUX_VERSION=$TERMUX_VERSION"
echo "PREFIX=$PREFIX"
echo ""

echo "Testing Termux logo with different sizes..."
echo ""

echo "=== ASCII Logo ==="
./target/release/rfetch --logo ascii

echo ""
echo "=== Small Logo ==="
./target/release/rfetch --logo small

echo ""
echo "=== Auto Logo ==="
./target/release/rfetch --logo auto