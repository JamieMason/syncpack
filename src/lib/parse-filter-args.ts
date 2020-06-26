export const parseFilterArgs = (filterArgs: string | string[]): RegExp[] => {
  if (Array.isArray(filterArgs)) {
    return filterArgs.map((filter) => new RegExp(filter));
  } else if (!filterArgs) {
    [new RegExp('.')];
  }
  return [new RegExp(filterArgs)];
};
