import { unwrap } from 'tightrope/result/unwrap';
import type { O } from 'ts-toolbelt';
import type { CliConfig, RcConfig } from '../config/types';
import { getPackageJsonFiles } from '../get-package-json-files';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Effects } from '../lib/effects';

export interface Context {
  config: {
    cli: Partial<CliConfig>;
    rcFile: O.Partial<RcConfig, 'deep'>;
  };
  effects: Effects;
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
}

export function getContext(cli: Partial<CliConfig>, effects: Effects): Context {
  const rcFile = effects.readConfigFileSync(cli.configPath);
  const config = { cli, rcFile };
  const packageJsonFiles = unwrap(getPackageJsonFiles(effects, config));
  return {
    config,
    effects,
    isInvalid: false,
    packageJsonFiles,
  };
}
