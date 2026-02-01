#!/usr/bin/env bash

# Remove the sitemap index file (we only have one sitemap)
rm dist/sitemap-index.xml
mv dist/sitemap-0.xml dist/sitemap.xml

# Add lastmod dates to sitemap entries
# Use current date as lastmod for all entries since we don't track per-page dates
CURRENT_DATE=$(date -u +"%Y-%m-%d")

# Use sed to add <lastmod> after each <loc> tag
# This works on both macOS and Linux
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  sed -i '' "s|</loc>|</loc><lastmod>${CURRENT_DATE}</lastmod>|g" dist/sitemap.xml
else
  # Linux
  sed -i "s|</loc>|</loc><lastmod>${CURRENT_DATE}</lastmod>|g" dist/sitemap.xml
fi
