import { unwrap } from 'tightrope/result/unwrap';
import type { O } from 'ts-toolbelt';
import type { CliConfig, RcConfig } from '../config/types';
import { getPackageJsonFiles } from '../get-package-json-files';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Disk } from '../lib/disk';

export interface Context {
  config: {
    cli: Partial<CliConfig>;
    rcFile: O.Partial<RcConfig, 'deep'>;
  };
  disk: Disk;
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
}

export function getContext(cli: Partial<CliConfig>, disk: Disk): Context {
  const rcFile = disk.readConfigFileSync(cli.configPath);
  const config = { cli, rcFile };
  const packageJsonFiles = unwrap(getPackageJsonFiles(disk, config));
  return {
    config,
    disk,
    isInvalid: false,
    packageJsonFiles,
  };
}
