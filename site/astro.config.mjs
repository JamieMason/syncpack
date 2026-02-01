import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import rehypeExternalLinks from 'rehype-external-links';
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
    // Moved docs
    '/guide/semver-groups/': '/semver-groups/',
    '/guide/version-groups/': '/version-groups/',
    // Removed docs
    '/integrations/github-actions': '/syncpack/',
    '/integrations/intellisense': '/syncpack/',
    '/integrations/json-schema': '/syncpack/',
    '/integrations/lerna': '/syncpack/',
    '/integrations/npm': '/syncpack/',
    '/integrations/pnpm': '/syncpack/',
    '/integrations/yarn': '/syncpack/',
    // Merged docs
    '/guide/local-package-versions/': '/syncpack/dependency-types',
    // Deprecated Feature: Removed
    // @TODO: change to /syncpack/guide/upgrading/ once written
    '/config/dependency-types/': '/syncpack/',
    '/config/lint-formatting/': '/syncpack/',
    '/config/lint-semver-ranges/': '/syncpack/',
    '/config/lint-versions/': '/syncpack/',
    '/config/specifier-types/': '/syncpack/',
    // Deprecated Feature: Moved
    '/command/fix-mismatches/': '/command/fix',
    '/command/set-semver-ranges/': '/command/fix',
    '/command/lint-semver-ranges/': '/command/lint',
    '/command/list-mismatches/': '/command/lint',
  },
  markdown: {
    smartypants: false,
    rehypePlugins: [[rehypeExternalLinks, { rel: ['nofollow', 'noopener'] }]],
    remarkPlugins: [
      function globalReferenceLinks() {
        const linksById = {
          COMMAND_FIX: '/syncpack/command/fix/',
          COMMAND_FORMAT: '/syncpack/command/format/',
          COMMAND_JSON: '/syncpack/command/json/',
          COMMAND_LINT: '/syncpack/command/lint/',
          COMMAND_LIST: '/syncpack/command/list/',
          COMMAND_UPDATE: '/syncpack/command/update/',

          CONFIG_CUSTOM_TYPES: '/syncpack/config/custom-types/',
          CONFIG_DEPENDENCY_GROUPS: '/syncpack/config/dependency-groups/',
          CONFIG_FORMAT_BUGS: '/syncpack/config/format-bugs/',
          CONFIG_FORMAT_REPOSITORY: '/syncpack/config/format-repository/',
          CONFIG_INDENT: '/syncpack/config/indent/',
          CONFIG_SEMVER_GROUPS: '/syncpack/semver-groups/',
          CONFIG_SORT_AZ: '/syncpack/config/sort-az/',
          CONFIG_SORT_EXPORTS: '/syncpack/config/sort-exports/',
          CONFIG_SORT_FIRST: '/syncpack/config/sort-first/',
          CONFIG_SORT_PACKAGES: '/syncpack/config/sort-packages/',
          CONFIG_SOURCE: '/syncpack/config/source/',
          CONFIG_SYNCPACKRC: '/syncpack/config/syncpackrc/',
          CONFIG_VERSION_GROUPS: '/syncpack/version-groups/',

          GUIDE_PEER_DEPENDENCIES: '/syncpack/guide/peer-dependencies/',

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
          HREF_CATEGORIZE_YOUR_DEPENDENCIES: 'https://antfu.me/posts/categorize-deps',
          HREF_CONDITIONAL_EXPORTS: 'https://nodejs.org/api/packages.html#conditional-exports',
          HREF_COSMICONFIG: 'https://github.com/cosmiconfig/cosmiconfig',
          HREF_DEPENDENCIES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#dependencies',
          HREF_DEV_DEPENDENCIES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#devDependencies',
          HREF_ENGINES: 'https://docs.npmjs.com/cli/v11/configuring-npm/package-json#engines',
          HREF_GLOB: 'https://github.com/rust-lang/glob',
          HREF_LERNA: 'https://lerna.js.org/',
          HREF_NEW_ISSUE: 'https://github.com/JamieMason/syncpack-github-action/issues/new',
          HREF_NPM_EXEC: 'https://docs.npmjs.com/cli/v11/commands/npm-exec',
          HREF_NPM_WORKSPACES: 'https://docs.npmjs.com/cli/v11/using-npm/workspaces',
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

          SEMVER_GROUP_IGNORED: '/syncpack/semver-groups/ignored/',
          SEMVER_GROUP_WITH_RANGE: '/syncpack/semver-groups/with-range/',

          VERSION_GROUP_BANNED: '/syncpack/version-groups/banned/',
          VERSION_GROUP_HIGHEST_SEMVER: '/syncpack/version-groups/highest-semver/',
          VERSION_GROUP_IGNORED: '/syncpack/version-groups/ignored/',
          VERSION_GROUP_LOWEST_SEMVER: '/syncpack/version-groups/lowest-semver/',
          VERSION_GROUP_PINNED: '/syncpack/version-groups/pinned/',
          VERSION_GROUP_SAME_MINOR: '/syncpack/version-groups/same-minor/',
          VERSION_GROUP_SAME_RANGE: '/syncpack/version-groups/same-range/',
          VERSION_GROUP_SNAPPED_TO: '/syncpack/version-groups/snapped-to/',
        };

        return function transformer(tree) {
          visit(tree, 'link', node => {
            const [id, hash] = node.url.split('#');
            const link = linksById[id];
            if (link) {
              node.url = hash ? `${link}#${hash}` : link;
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
      lastUpdated: true,
      routeMiddleware: './src/route-data.ts',
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
      expressiveCode: {
        themes: ['github-light-default', 'github-dark-default'],
        removeUnusedThemes: false,
      },
      customCss: ['./src/styles/custom.css'],
      pagination: false,
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
        Head: './src/components/Head.astro',
      },
      head: [
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
