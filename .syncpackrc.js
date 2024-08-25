// @ts-check

/** @type {import("./src").RcFile} */
const config = {
  semverGroups: [
    {
      range: '',
    },
  ],
  versionGroups: [
    // v10 does not support Node 18
    { dependencies: ['minimatch'], pinVersion: '9.0.5' },
  ],
};

export default config;
