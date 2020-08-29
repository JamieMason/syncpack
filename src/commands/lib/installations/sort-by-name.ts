import { InstalledPackage } from './get-dependencies';

export const sortByName = (a: InstalledPackage, b: InstalledPackage): 0 | 1 | -1 => {
  if (a.name < b.name) {
    return -1;
  }
  if (a.name > b.name) {
    return 1;
  }
  return 0;
};
