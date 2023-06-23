/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  docs: {
    'Introduction': ['installation', 'config-file', 'github-action'],
    'Guides': ['guide/version-groups', 'guide/troubleshooting'],
    'CLI Commands': [
      'fix-mismatches',
      'format',
      'lint-semver-ranges',
      'lint',
      'list-mismatches',
      'list',
      'prompt',
      'set-semver-ranges',
      'update',
    ],
    'CLI Options': [
      'option/config',
      'option/filter',
      'option/indent',
      'option/semver-range',
      'option/source',
      'option/types',
    ],
    'Configuration File Options': [
      'config/custom-types',
      'config/dependency-types',
      'config/filter',
      'config/indent',
      {
        type: 'category',
        label: 'semverGroups',
        collapsible: true,
        collapsed: false,
        link: {
          type: 'doc',
          id: 'config/semver-groups',
        },
        items: ['config/semver-groups/ignored', 'config/semver-groups/standard'],
      },
      'config/semver-range',
      'config/sort-az',
      'config/sort-first',
      'config/source',
      {
        type: 'category',
        label: 'versionGroups',
        collapsible: true,
        collapsed: false,
        link: {
          type: 'doc',
          id: 'config/version-groups',
        },
        items: [
          'config/version-groups/banned',
          'config/version-groups/ignored',
          'config/version-groups/pinned',
          'config/version-groups/same-range',
          'config/version-groups/snapped-to',
          'config/version-groups/standard',
        ],
      },
    ],
  },
};

module.exports = sidebars;
