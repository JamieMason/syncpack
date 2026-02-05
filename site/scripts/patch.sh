#!/usr/bin/env bash

function patch_sitemap() {
  # Remove the sitemap index file (we only have one sitemap)
  rm dist/sitemap-index.xml
  mv dist/sitemap-0.xml dist/sitemap.xml
  # create sitemap.txt
  node scripts/patch.js
  find dist -name "*.html" | while read -r file; do
    # replace sitemap.xml with sitemap.txt and 'alt ' with 'alt="" '
    sed -e "s|sitemap.xml|sitemap.txt|g" -e "s|alt |alt=\"\" |g" "$file" > "$file.tmp"
    mv "$file.tmp" "$file"
  done
}

patch_sitemap
