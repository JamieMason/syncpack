import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import { visit } from 'unist-util-visit';

export default defineConfig({
  site: 'https://jamiemason.github.io/syncpack',
  base: '/syncpack',
  redirects: {
    // Hoisted docs
    '/config/semver-groups/ignored/': '/syncpack/semver-groups/ignored/',
    '/config/semver-groups/with-range/': '/syncpack/semver-groups/with-range/',
    '/config/version-groups/banned/': '/syncpack/version-groups/banned/',
    '/config/version-groups/ignored/': '/syncpack/version-groups/ignored/',
    '/config/version-groups/lowest-version/': '/syncpack/version-groups/lowest-semver/',
    '/config/version-groups/pinned/': '/syncpack/version-groups/pinned/',
    '/config/version-groups/same-range/': '/syncpack/version-groups/same-range/',
    '/config/version-groups/snapped-to/': '/syncpack/version-groups/snapped-to/',
    '/config/version-groups/standard/': '/syncpack/version-groups/highest-semver/',
    '/guide/status-codes/': '/syncpack/status/',
    '/guide/getting-started/': '/syncpack/',
    // Merged docs
    '/guide/local-package-versions/': '/syncpack/dependency-types',
    // Deprecated Feature: Removed
    '/config/dependency-types/': '/syncpack/guide/upgrading/',
    '/config/lint-formatting/': '/syncpack/guide/upgrading/',
    '/config/lint-semver-ranges/': '/syncpack/guide/upgrading/',
    '/config/lint-versions/': '/syncpack/guide/upgrading/',
    '/config/specifier-types/': '/syncpack/guide/upgrading/',
    // Deprecated Feature: Moved
    '/command/fix-mismatches/': '/command/fix',
    '/command/set-semver-ranges/': '/command/fix',
    '/command/lint-semver-ranges/': '/command/lint',
    '/command/list-mismatches/': '/command/lint',
  },
  markdown: {
    smartypants: false,
    remarkPlugins: [
      function globalReferenceLinks() {
        const linksById = {
          CONFIG_CUSTOM_TYPES: '/syncpack/config/custom-types/',
          CONFIG_SOURCE: '/syncpack/config/source/',

          GUIDE_CATALOGS: '/syncpack/guide/catalogs/',
          GUIDE_PEER_DEPENDENCIES: '/syncpack/guide/peer-dependencies/',
          GUIDE_SEMVER_GROUPS: '/syncpack/guide/semver-groups/',
          GUIDE_VERSION_GROUPS: '/syncpack/guide/version-groups/',

          REF_DEPENDENCY_TYPES: '/syncpack/dependency-types/',
          REF_GLOSSARY: '/syncpack/glossary/',
          REF_SPECIFIER_TYPES: '/syncpack/specifier-types/',
          REF_STATUS_CODES: '/syncpack/status/',

          TERM_CUSTOM_TYPE: '/syncpack/glossary/#custom-type',
          TERM_DEPENDENCY: '/syncpack/glossary/#dependency',
          TERM_DEPENDENCY_GROUP: '/syncpack/glossary/#dependency-group',
          TERM_DEPENDENCY_TYPE: '/syncpack/glossary/#dependency-type',
          TERM_INSTANCE: '/syncpack/glossary/#instance',
          TERM_PACKAGE: '/syncpack/glossary/#package',
          TERM_RCFILE: '/syncpack/glossary/#rcfile',
          TERM_SEMVER: '/syncpack/glossary/#semver',
          TERM_SEMVER_GROUP: '/syncpack/glossary/#semver-group',
          TERM_SEMVER_RANGE: '/syncpack/glossary/#semver-range',
          TERM_SPECIFIER: '/syncpack/glossary/#specifier',
          TERM_SPECIFIER_TYPE: '/syncpack/glossary/#specifier-type',
          TERM_STATUS_CODE: '/syncpack/glossary/#status-code',
          TERM_VERSION_GROUP: '/syncpack/glossary/#version-group',
          TERM_WORKSPACE: '/syncpack/glossary/#workspace',

          HREF_ANSI: 'https://en.wikipedia.org/wiki/ANSI_escape_code',
          HREF_AWS_SDK: 'https://aws.amazon.com/sdk-for-javascript/',
          HREF_CONDITIONAL_EXPORTS: 'https://nodejs.org/api/packages.html#conditional-exports',
          HREF_COSMICONFIG: 'https://github.com/cosmiconfig/cosmiconfig',
          HREF_DEPENDENCIES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#dependencies',
          HREF_DEV_DEPENDENCIES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#devDependencies',
          HREF_ENGINES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#engines',
          HREF_GLOB: 'https://github.com/rust-lang/glob',
          HREF_LERNA: 'https://lerna.js.org/',
          HREF_NEW_ISSUE: 'https://github.com/JamieMason/syncpack-github-action/issues/new',
          HREF_NPM_EXEC: 'https://docs.npmjs.com/cli/v11/commands/npm-exec',
          HREF_NPX: 'https://docs.npmjs.com/cli/v11/commands/npx',
          HREF_OVERRIDES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#overrides',
          HREF_PACKAGE_MANAGER: 'https://nodejs.org/api/packages.html#packagemanager',
          HREF_PEER_DEPENDENCIES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#peerDependencies',
          HREF_PNPM: 'https://pnpm.js.org/',
          HREF_PNPM_OVERRIDES: 'https://pnpm.io/package_json#pnpmoverrides',
          HREF_RESOLUTIONS: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#resolutions',
          HREF_SYNCPACK_GITHUB_ACTION:
            'https://github.com/marketplace/actions/syncpack-synchronise-monorepo-dependency-versions',
          HREF_TYPES: 'https://github.com/DefinitelyTyped/DefinitelyTyped',
          HREF_VERSION: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#version',
          HREF_WORKSPACE_PROTOCOL: 'https://pnpm.io/workspaces#workspace-protocol-workspace',
          HREF_YARN_WORKSPACES: 'https://yarnpkg.com/lang/en/docs/workspaces/',
        };

        return function transformer(tree) {
          visit(tree, 'link', node => {
            if (linksById[node.url]) {
              node.url = linksById[node.url];
            }
          });
          return tree;
        };
      },
    ],
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
      // expressiveCode: false,
      customCss: ['./src/styles/custom.css'],
      sidebar: [
        {
          label: 'Github',
          link: 'https://github.com/JamieMason/syncpack',
        },
        {
          label: 'Guides',
          items: [
            { slug: 'index', label: 'Getting Started' },
            { slug: 'guide/catalogs' },
            { slug: 'guide/peer-dependencies' },
            { slug: 'guide/semver-groups' },
            { slug: 'guide/upgrading' },
            { slug: 'guide/version-groups' },
          ],
        },
        {
          label: 'Reference',
          items: [
            { slug: 'dependency-types' },
            { slug: 'glossary' },
            { slug: 'specifier-types' },
            { slug: 'status', label: 'Status Codes' },
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
