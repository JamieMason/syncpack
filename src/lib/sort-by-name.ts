interface WithName {
  name: string;
}

export function sortByName(a: WithName, b: WithName): 0 | 1 | -1 {
  if (a.name < b.name) {
    return -1;
  }
  if (a.name > b.name) {
    return 1;
  }
  return 0;
}
