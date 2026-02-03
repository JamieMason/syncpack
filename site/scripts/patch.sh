#!/usr/bin/env bash

function patch_sitemap() {
  # Remove the sitemap index file (we only have one sitemap)
  rm dist/sitemap-index.xml
  mv dist/sitemap-0.xml dist/sitemap.xml

  # Add lastmod dates to sitemap entries
  # Use current date as lastmod for all entries since we don't track per-page dates
  CURRENT_DATE=$(date -u +"%Y-%m-%d")

  # Use sed with temp file (works on both macOS and Linux)
  sed "s|</loc>|</loc><lastmod>${CURRENT_DATE}</lastmod>|g" dist/sitemap.xml > dist/sitemap.xml.tmp
  mv dist/sitemap.xml.tmp dist/sitemap.xml
}

function patch_link_rel_icon() {
  # find rel="shortcut icon" and replace with rel="icon" in every html file in dist
  find dist -name "*.html" | while read -r file; do
    sed "s|rel=\"shortcut icon\"|rel=\"icon\"|g" "$file" > "$file.tmp"
    mv "$file.tmp" "$file"
  done
}

patch_sitemap
patch_link_rel_icon
