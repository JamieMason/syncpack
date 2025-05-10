import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  site: 'https://jamiemason.github.io/syncpack',
  base: '/syncpack',
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
        baseUrl: 'https://github.com/JamieMason/syncpack/edit/starlight/site/',
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
          autogenerate: { directory: 'guide' },
        },
        {
          label: 'Commands',
          autogenerate: { directory: 'command' },
        },
        {
          label: 'Config',
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
