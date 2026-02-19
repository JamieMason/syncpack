import { visit } from 'unist-util-visit';

export function linkAliases() {
  const linksById = {
    COMMAND_FIX: '/command/fix/',
    COMMAND_FORMAT: '/command/format/',
    COMMAND_JSON: '/command/json/',
    COMMAND_LINT: '/command/lint/',
    COMMAND_LIST: '/command/list/',
    COMMAND_UPDATE: '/command/update/',

    CONFIG_CUSTOM_TYPES: '/config/custom-types/',
    CONFIG_DEPENDENCY_GROUPS: '/config/dependency-groups/',
    CONFIG_FORMAT_BUGS: '/config/format-bugs/',
    CONFIG_FORMAT_REPOSITORY: '/config/format-repository/',
    CONFIG_INDENT: '/config/indent/',
    CONFIG_SEMVER_GROUPS: '/semver-groups/',
    CONFIG_SORT_AZ: '/config/sort-az/',
    CONFIG_SORT_EXPORTS: '/config/sort-exports/',
    CONFIG_SORT_FIRST: '/config/sort-first/',
    CONFIG_SORT_PACKAGES: '/config/sort-packages/',
    CONFIG_SOURCE: '/config/source/',
    CONFIG_SYNCPACKRC: '/config/syncpackrc/',
    CONFIG_VERSION_GROUPS: '/version-groups/',

    GUIDE_PEER_DEPENDENCIES: '/guide/peer-dependencies/',

    REF_DEPENDENCY_TYPES: '/dependency-types/',
    REF_GLOSSARY: '/glossary/',
    REF_SPECIFIER_TYPES: '/specifier-types/',
    REF_STATUS_CODES: '/status/',

    TERM_CUSTOM_TYPE: '/glossary/#custom-type',
    TERM_DEPENDENCY: '/glossary/#dependency',
    TERM_DEPENDENCY_GROUP: '/glossary/#dependency-group',
    TERM_DEPENDENCY_TYPE: '/glossary/#dependency-type',
    TERM_INSTANCE: '/glossary/#instance',
    TERM_PACKAGE: '/glossary/#package',
    TERM_RCFILE: '/glossary/#rcfile',
    TERM_SEMVER: '/glossary/#semver',
    TERM_SEMVER_GROUP: '/glossary/#semver-group',
    TERM_SEMVER_RANGE: '/glossary/#semver-range',
    TERM_SPECIFIER: '/glossary/#specifier',
    TERM_SPECIFIER_TYPE: '/glossary/#specifier-type',
    TERM_STATUS_CODE: '/glossary/#status-code',
    TERM_VERSION_GROUP: '/glossary/#version-group',
    TERM_WORKSPACE: '/glossary/#workspace',

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

    SEMVER_GROUP_IGNORED: '/semver-groups/ignored/',
    SEMVER_GROUP_WITH_RANGE: '/semver-groups/with-range/',

    VERSION_GROUP_BANNED: '/version-groups/banned/',
    VERSION_GROUP_HIGHEST_SEMVER: '/version-groups/highest-semver/',
    VERSION_GROUP_IGNORED: '/version-groups/ignored/',
    VERSION_GROUP_LOWEST_SEMVER: '/version-groups/lowest-semver/',
    VERSION_GROUP_PINNED: '/version-groups/pinned/',
    VERSION_GROUP_SAME_MINOR: '/version-groups/same-minor/',
    VERSION_GROUP_SAME_RANGE: '/version-groups/same-range/',
    VERSION_GROUP_SNAPPED_TO: '/version-groups/snapped-to/',
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
}
