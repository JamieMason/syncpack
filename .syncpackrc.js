// @ts-check

/** @type {import("./src").RcFile} */
const config = {
  semverGroups: [
    {
      dependencyTypes: ['overrides'],
      isIgnored: true,
    },
    {
      range: '',
    },
  ],
  versionGroups: [
    { dependencies: ['string-width'], pinVersion: '<5.0.0' },
    { dependencies: ['strip-ansi'], pinVersion: '<7.0.0' },
    { dependencies: ['wrap-ansi'], pinVersion: '<8.0.0' },
    { dependencies: ['chalk'], pinVersion: '4.1.2' },
    { dependencies: ['globby'], pinVersion: '11.1.0' },
    { dependencies: ['ora'], pinVersion: '5.4.1' },
  ],
};

export default config;
