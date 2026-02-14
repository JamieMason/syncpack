import fs from 'node:fs';
import { glob } from 'node:fs/promises';
import path from 'node:path';

// Iterate over discovered file paths
for await (const entry of glob('**/*.html')) {
  const slug = entry.replace(/\.html$/, '').replace('index', '');
  const url = `https://syncpack.dev/${slug}`;
  const contents = await fs.promises.readFile(entry, 'utf8');

  if (contents.includes('Redirecting to')) continue;



  const x = `<!doctype html><head>
<title>Redirecting to: ${url}</title>
<meta http-equiv="refresh" content="0;url=${url}" />
<link rel="canonical" href="${url}"/>
</head><body>
  <a href="${url}">Redirecting from <code>/syncpack/${slug}</code> to <code>${url}</code></a>
</body>
`


  fs.writeFileSync(entry, x);
}
