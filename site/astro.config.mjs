import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  site: 'https://jamiemason.github.io/syncpack',
  base: '/syncpack',
  redirects: {
    // Hoisted
    '/config/semver-groups/ignored/': '/syncpack/semver-groups/ignored/',
    '/config/semver-groups/with-range/': '/syncpack/semver-groups/with-range/',
    '/config/version-groups/banned/': '/syncpack/version-groups/banned/',
    '/config/version-groups/ignored/': '/syncpack/version-groups/ignored/',
    '/config/version-groups/lowest-version/':
      '/syncpack/version-groups/lowest-semver/',
    '/config/version-groups/pinned/': '/syncpack/version-groups/pinned/',
    '/config/version-groups/same-range/':
      '/syncpack/version-groups/same-range/',
    '/config/version-groups/snapped-to/':
      '/syncpack/version-groups/snapped-to/',
    '/config/version-groups/standard/':
      '/syncpack/version-groups/highest-semver/',
    '/guide/status-codes/': '/syncpack/status/',
    // Deprecated: Removed
    '/config/dependency-types/': '/syncpack/guide/upgrading/',
    '/config/lint-formatting/': '/syncpack/guide/upgrading/',
    '/config/lint-semver-ranges/': '/syncpack/guide/upgrading/',
    '/config/lint-versions/': '/syncpack/guide/upgrading/',
    '/config/specifier-types/': '/syncpack/guide/upgrading/',
    // Deprecated: Moved
    '/command/fix-mismatches/': '/command/fix',
    '/command/set-semver-ranges/': '/command/fix',
    '/command/lint-semver-ranges/': '/command/lint',
    '/command/list-mismatches/': '/command/lint',
  },
  integrations: [
    starlight({
      title: 'Syncpack',
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/JamieMason/syncpack',
        },
        {
          icon: 'blueSky',
          label: 'Bluesky',
          href: 'https://bsky.app/profile/foldleft.bsky.social',
        },
        {
          icon: 'x.com',
          label: 'X',
          href: 'https://x.com/fold_left',
        },
      ],
      editLink: {
        baseUrl: 'https://github.com/JamieMason/syncpack/edit/main/site/',
      },
      favicon: '/favicon.ico',
      logo: {
        src: './src/assets/logo.svg',
      },
      components: {
        Sidebar: './src/components/Sidebar.astro',
      },
      customCss: ['./src/styles/custom.css'],
      sidebar: [
        {
          label: 'Github',
          link: 'https://github.com/JamieMason/syncpack',
        },
        {
          label: 'Guides',
          items: [
            { slug: 'guide/getting-started' },
            { slug: 'guide/local-package-versions' },
            { slug: 'guide/semver-groups' },
            { slug: 'status', label: 'Status Codes' },
            { slug: 'guide/upgrading' },
            { slug: 'guide/version-groups' },
          ],
        },
        {
          label: 'Commands',
          autogenerate: { directory: 'command' },
        },
        {
          label: 'Version Groups',
          autogenerate: { directory: 'version-groups' },
        },
        {
          label: 'Semver Groups',
          autogenerate: { directory: 'semver-groups' },
        },
        {
          label: 'Configuration File',
          autogenerate: { directory: 'config' },
        },
        {
          label: 'Integrations',
          autogenerate: { directory: 'integrations' },
        },
        {
          label: 'Examples',
          autogenerate: { directory: 'examples' },
        },
      ],
      head: [
        {
          tag: 'meta',
          attrs: {
            name: 'twitter:image',
            content: '/syncpack/social-card.jpg',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image',
            content: '/syncpack/social-card.jpg',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image:width',
            content: '1200',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:image:height',
            content: '675',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'twitter:creator',
            content: '@fold_left',
          },
        },
        {
          tag: 'meta',
          attrs: {
            name: 'twitter:site',
            content: '@fold_left',
          },
        },
      ],
    }),
  ],
});
