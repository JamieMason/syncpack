#!/usr/bin/env bash

function patch_sitemap() {
  # Remove the sitemap index file (we only have one sitemap)
  rm dist/sitemap-index.xml
  mv dist/sitemap-0.xml dist/sitemap.xml
  # create sitemap.txt
  node scripts/patch.js
  # replace sitemap.xml with sitemap.txt
  find dist -name "*.html" | while read -r file; do
    sed "s|sitemap.xml|sitemap.txt|g" "$file" > "$file.tmp"
    mv "$file.tmp" "$file"
  done
}

patch_sitemap
