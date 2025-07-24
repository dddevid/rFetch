#!/bin/bash

# Test script per simulare l'ambiente Termux
echo "Testing Termux support..."

# Simula l'ambiente Termux impostando le variabili d'ambiente
export TERMUX_VERSION="0.118.0"
export PREFIX="/data/data/com.termux/files/usr"

echo "Environment variables set:"
echo "TERMUX_VERSION=$TERMUX_VERSION"
echo "PREFIX=$PREFIX"
echo ""

echo "Running rfetch with Termux environment simulation..."
./target/release/rfetch

echo ""
echo "Testing with small logo..."
./target/release/rfetch --logo small

echo ""
echo "Testing with JSON output..."
./target/release/rfetch --json | head -20

echo ""
echo "Testing with neon theme..."
./target/release/rfetch --theme neon