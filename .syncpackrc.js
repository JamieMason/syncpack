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
  ],
};

module.exports = config;
