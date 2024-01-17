import { cosmiconfig } from 'cosmiconfig';
import { Context } from 'effect';
import enquirer from 'enquirer';
import fs from 'fs';
import { globbySync } from 'globby';
import { sync as readYamlFileSync } from 'read-yaml-file';

export interface Io {
  cosmiconfig: {
    cosmiconfig: typeof cosmiconfig;
  };
  enquirer: {
    prompt: typeof enquirer.prompt<any>;
  };
  fs: typeof fs;
  globby: {
    sync: typeof globbySync;
  };
  process: {
    cwd: typeof process.cwd;
    exit: typeof process.exit;
  };
  readYamlFile: {
    sync: typeof readYamlFileSync<any>;
  };
}

export const IoTag = Context.Tag<Io>();

export const io: Io = {
  cosmiconfig: {
    cosmiconfig,
  },
  enquirer: {
    prompt: enquirer.prompt,
  },
  fs: fs,
  globby: {
    sync: globbySync,
  },
  process: {
    cwd: process.cwd,
    exit: process.exit,
  },
  readYamlFile: {
    sync: readYamlFileSync,
  },
};
