console.log("stray output from a config file");
module.exports = {
  semverGroups: [
    {
      label: "Dependencies should use caret dependency ranges",
      dependencies: ["**"],
      dependencyTypes: ["dev", "peer", "prod"],
      packages: ["**"],
      range: "^",
    },
  ],
};
