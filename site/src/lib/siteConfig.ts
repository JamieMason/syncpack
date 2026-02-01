/**
 * Site-wide metadata for structured data schemas.
 *
 * These values are used across all schema builders to ensure consistency.
 */
export const siteConfig = {
  siteName: 'Syncpack',
  siteUrl: 'https://jamiemason.github.io/syncpack',
  description: 'Consistent dependency versions in large JavaScript Monorepos',
  softwareDescription:
    'CLI for managing dependency versions in JavaScript monorepos. Used by AWS, Cloudflare, DataDog, Electron, Microsoft, Vercel, and others.',
  logoUrl: 'https://jamiemason.github.io/syncpack/logo.svg',
  npmUrl: 'https://www.npmjs.com/package/syncpack',
  githubUrl: 'https://github.com/JamieMason/syncpack',
  authorName: 'Jamie Mason',
  authorUrl: 'https://github.com/JamieMason',
  authorImage: 'https://avatars.githubusercontent.com/u/320492?v=4',
  authorSocial: [
    'https://github.com/JamieMason',
    'https://bsky.app/profile/foldleft.bsky.social',
    'http://uk.linkedin.com/in/jamiemasonleeds',
    'https://twitter.com/fold_left',
  ],
  sponsorUrl: 'https://github.com/sponsors/JamieMason',
} as const;
