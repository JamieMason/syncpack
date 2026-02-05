import fs from 'node:fs/promises';

const sitemap = await fs.readFile('dist/sitemap.xml', 'utf8');

const urls = sitemap.match(/<loc>(.*?)<\/loc>/g).map(url => url.replace(/<loc>|<\/loc>/g, ''));

await fs.writeFile('dist/sitemap.txt', `${urls.join('\n')}\n`);
