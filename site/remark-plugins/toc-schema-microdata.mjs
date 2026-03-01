import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { glob } from 'glob';

/**
 * Rewrites `nav[aria-labelledby="starlight__on-this-page"]` (and its mobile
 * counterpart) to add schema.org SiteNavigationElement microdata attributes.
 *
 * Before:
 *   <nav aria-labelledby="starlight__on-this-page">
 *     <ul>
 *       <li><a href="#install"><span>Install</span></a></li>
 *     </ul>
 *   </nav>
 *
 * After:
 *   <nav aria-labelledby="starlight__on-this-page"
 *        itemscope itemtype="http://schema.org/SiteNavigationElement">
 *     <ul>
 *       <li><a itemprop="url" href="#install"><span itemprop="name">Install</span></a></li>
 *     </ul>
 *   </nav>
 *
 * Only runs at build time (`astro:build:done`). The microdata is purely for
 * SEO/structured-data and has no effect in dev.
 */

/**
 * Find the closing `</nav>` that matches the `<nav>` opening at `navOpenEnd`,
 * tracking nesting depth. Returns the index just past the closing tag.
 */
function findNavClose(html, navOpenEnd) {
  let depth = 1;
  let cursor = navOpenEnd;

  while (cursor < html.length && depth > 0) {
    const openIdx = html.indexOf('<nav', cursor);
    const closeIdx = html.indexOf('</nav>', cursor);

    if (closeIdx === -1) return -1; // malformed — bail

    if (openIdx !== -1 && openIdx < closeIdx) {
      depth++;
      cursor = openIdx + 4;
    } else {
      depth--;
      cursor = closeIdx + 6;
    }
  }

  return depth === 0 ? cursor : -1;
}

/**
 * Patch a nav fragment's inner `<a>` and `<span>` tags with itemprop attrs.
 */
function patchNavFragment(fragment) {
  // Add itemprop="url" to <a ...> tags that don't already have itemprop.
  let result = fragment.replace(/<a(\s[^>]*?)>/g, (full, innerAttrs) => {
    if (innerAttrs.includes('itemprop')) return full;
    return `<a${innerAttrs} itemprop="url">`;
  });

  // Add itemprop="name" to <span ...> tags that don't already have itemprop.
  // Starlight's TOC structure: <a ...><span ...>text</span></a>
  result = result.replace(/<span(\s[^>]*?)>/g, (full, innerAttrs) => {
    if (innerAttrs.includes('itemprop')) return full;
    return `<span${innerAttrs} itemprop="name">`;
  });

  return result;
}

/**
 * Rewrite a single nav element identified by the labelledby value.
 * Returns the mutated html string (unchanged if nav not found or already patched).
 */
function rewriteNav(html, labelledby) {
  // Match the opening <nav ...> tag with the given aria-labelledby value.
  const navOpenRe = new RegExp(
    `<nav(\\s[^>]*?aria-labelledby="${labelledby}"[^>]*?)>`,
  );
  const match = navOpenRe.exec(html);
  if (!match) return html;

  const navOpenStart = match.index;
  const navOpenEnd = navOpenStart + match[0].length;
  const attrs = match[1];

  // Skip if already patched.
  if (attrs.includes('itemscope')) return html;

  // Find where this nav closes.
  const navCloseEnd = findNavClose(html, navOpenEnd);
  if (navCloseEnd === -1) return html;

  // Build the new opening tag.
  const newOpen = `<nav${attrs} itemscope itemtype="http://schema.org/SiteNavigationElement">`;

  // Grab the fragment (excluding the old opening tag, we'll replace it).
  const innerAndClose = html.slice(navOpenEnd, navCloseEnd);

  // Patch <a> and <span> inside the fragment.
  const patchedInner = patchNavFragment(innerAndClose);

  return html.slice(0, navOpenStart) + newOpen + patchedInner + html.slice(navCloseEnd);
}

/**
 * Apply microdata rewrites to both the desktop and mobile TOC navs.
 */
function rewriteHtml(html) {
  let result = rewriteNav(html, 'starlight__on-this-page');
  result = rewriteNav(result, 'starlight__on-this-page--mobile');
  return result;
}

/**
 * Astro integration — post-processes all HTML files in the build output to
 * inject schema.org microdata into Starlight's table-of-contents nav elements.
 */
export function tocSchemaMicrodata() {
  return {
    name: 'toc-schema-microdata',
    hooks: {
      'astro:build:done': async ({ dir, logger }) => {
        const outDir = fileURLToPath(dir);
        const files = await glob('**/*.html', { cwd: outDir, absolute: true });

        let count = 0;

        await Promise.all(
          files.map(async filePath => {
            const html = await readFile(filePath, 'utf-8');
            if (!html.includes('starlight__on-this-page')) return;

            const rewritten = rewriteHtml(html);
            if (rewritten === html) return;

            await writeFile(filePath, rewritten, 'utf-8');
            count++;
          }),
        );

        logger.info(`toc-schema-microdata: patched ${count} HTML file(s)`);
      },
    },
  };
}
