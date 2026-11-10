#!/usr/bin/env sh
# Build the site and mirror Trunk's output into the repository root.
#
# This repository's GitHub Pages site is deployed from the branch root,
# so the generated index.html and its hashed JS/WASM/CSS files must live
# at the root of the branch, not only in the ignored dist/ directory.
set -eu

cd "$(dirname "$0")/.."

trunk build --release

rm -f devnotes-site-*.js devnotes-site-*_bg.wasm style-*.css
cp dist/index.html .
cp dist/style-*.css .
cp dist/devnotes-site-*.js .
cp dist/devnotes-site-*_bg.wasm .

echo "GitHub Pages root synchronized with dist/."
