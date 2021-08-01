import { SyncpackConfig } from '../../constants';
import { InstalledPackage } from './installations/get-dependencies';

type Options = Pick<SyncpackConfig, 'filter'>;

export const matchesFilter =
  (options: Options) =>
  ({ name }: InstalledPackage): boolean =>
    name.search(new RegExp(options.filter)) !== -1;
