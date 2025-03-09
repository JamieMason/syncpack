// @ts-check

/** @type {import("./src").RcFile} */
const config = {
  semverGroups: [
    {
      dependencyTypes: ['prod', 'dev'],
      range: '^',
    },
  ],
  versionGroups: [
    {
      label: 'v10 does not support Node 18',
      dependencies: ['minimatch'],
      pinVersion: '9.0.5',
    },
    {
      label: 'v10 does not support Node 18',
      dependencies: ['@release-it/conventional-changelog'],
      pinVersion: '8.0.2',
    },
    {
      label: 'v18 does not support Node 18',
      dependencies: ['release-it'],
      pinVersion: '17.11.0',
    },
    {
      label: '0.3.0 has issues',
      dependencies: ['tightrope'],
      pinVersion: '0.2.0',
    },
  ],
};

export default config;
