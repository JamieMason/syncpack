// @ts-check

/** @type {import("./src").RcFile} */
const config = {
  semverGroups: [
    {
      dependencyTypes: ['resolutions'],
      isIgnored: true,
    },
    {
      range: '',
    },
  ],
  versionGroups: [
    {
      dependencies: ['@types/node'],
      pinVersion: '18.19.3',
    },
    {
      dependencies: ['chalk'],
      pinVersion: '4.1.2',
    },
    {
      dependencies: ['globby'],
      pinVersion: '11.1.0',
    },
    {
      dependencies: ['ora'],
      pinVersion: '5.4.1',
    },
  ],
};

module.exports = config;
