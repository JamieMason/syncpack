import { rehypeHeadingIds } from '@astrojs/markdown-remark';
import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import rehypeExternalLinks from 'rehype-external-links';
import { headingAnchorLinks } from './remark-plugins/heading-anchor-links.mjs';
import { linkAliases } from './remark-plugins/link-aliases.mjs';
import { sectionWrapper } from './remark-plugins/section-wrapper.mjs';
import { tocSchemaMicrodata } from './remark-plugins/toc-schema-microdata.mjs';

export default defineConfig({
  site: 'https://syncpack.dev',
  base: '/',
  output: 'static',
  markdown: {
    smartypants: false,
    rehypePlugins: [rehypeHeadingIds, headingAnchorLinks, [rehypeExternalLinks, { rel: ['nofollow', 'noopener'] }]],
    remarkPlugins: [sectionWrapper, linkAliases],
  },
  integrations: [
    tocSchemaMicrodata(),
    starlight({
      title: 'Syncpack',
      lastUpdated: true,
      markdown: {
        headingLinks: false,
      },
      routeMiddleware: './src/route-data.ts',
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/JamieMason/syncpack',
        },
        {
          icon: 'npm',
          label: 'npm',
          href: 'https://npmx.dev/package/syncpack',
        },
        {
          icon: 'blueSky',
          label: 'Bluesky',
          href: 'https://bsky.app/profile/foldleft.bsky.social',
        },
      ],
      editLink: {
        baseUrl: 'https://github.com/JamieMason/syncpack/edit/main/site/',
      },
      favicon: '/favicon.ico',
      logo: {
        src: './src/assets/logo.svg',
      },
      tableOfContents: {
        minHeadingLevel: 2,
        maxHeadingLevel: 4,
      },
      expressiveCode: {
        themes: ['everforest-dark', 'everforest-light'],
        removeUnusedThemes: false,
        frames: false,
        styleOverrides: {
          // Borders and spacing
          borderRadius: '6px',
          borderWidth: '1px',
          borderColor: 'var(--sl-color-gray-5)',

          // Code area
          codeBackground: 'var(--sl-color-gray-6)',
          codeFontFamily: 'var(--sl-font-mono)',
          codeFontSize: '0.8rem',
          codeLineHeight: '1.65',
          codePaddingBlock: '1rem',
          codePaddingInline: '1.35rem',
          codeForeground: 'var(--sl-color-gray-2)',

          // Accent color for highlights
          focusBorder: 'var(--sl-color-accent)',
        },
      },
      customCss: ['@fontsource-variable/inter', '@fontsource-variable/jetbrains-mono', './src/styles/custom.css'],
      pagination: false,
      sidebar: [
        {
          label: 'Guides',
          autogenerate: { directory: 'guide' },
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
          items: [
            'config/syncpackrc',
            'config/custom-types',
            'config/dependency-groups',
            'config/format-bugs',
            'config/format-repository',
            'config/indent',
            'config/max-concurrent-requests',
            { label: 'semverGroups', link: '/semver-groups/' },
            'config/sort-az',
            'config/sort-exports',
            'config/sort-first',
            'config/sort-packages',
            'config/source',
            'config/strict',
            { label: 'versionGroups', link: '/version-groups/' },
          ],
        },
        {
          label: 'Reference',
          autogenerate: { directory: 'reference' },
        },
      ],
      components: {
        Footer: './src/components/Footer.astro',
        Head: './src/components/Head.astro',
        SocialIcons: './src/components/SocialIcons.astro',
      },
      head: [
        {
          tag: 'link',
          attrs: {
            rel: 'apple-touch-icon',
            href: '/apple-touch-icon-180x180.png',
          },
        },
        {
          tag: 'script',
          attrs: {
            async: true,
            src: 'https://www.googletagmanager.com/gtag/js?id=G-DXPH5LLJ0N',
          },
        },
        {
          tag: 'script',
          content: [
            'window.dataLayer=window.dataLayer||[];',
            'function gtag(){dataLayer.push(arguments);}',
            `gtag('js', new Date());`,
            `gtag('config', 'G-DXPH5LLJ0N');`,
          ].join(''),
        },
      ],
    }),
  ],
});
