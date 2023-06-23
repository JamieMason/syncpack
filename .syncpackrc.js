// @ts-check

/** @type {import("./src").RcFile} */
const config = {
  versionGroups: [
    {
      dependencies: ['@types/node'],
      packages: ['**'],
      pinVersion: '14.18.36',
    },
    {
      dependencies: ['chalk'],
      packages: ['**'],
      pinVersion: '4.1.2',
    },
    {
      dependencies: ['globby'],
      packages: ['**'],
      pinVersion: '11.1.0',
    },
    {
      dependencies: ['ora'],
      packages: ['**'],
      pinVersion: '5.4.1',
    },
  ],
};

module.exports = config;
