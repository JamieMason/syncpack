import { cosmiconfig } from 'cosmiconfig';
import { Context } from 'effect';
import { prompt } from 'enquirer';
import * as fs from 'fs';
import * as globby from 'globby';
import * as readYamlFile from 'read-yaml-file';

export interface Io {
  cosmiconfig: {
    cosmiconfig: typeof cosmiconfig;
  };
  enquirer: {
    prompt: typeof prompt<any>;
  };
  fs: typeof fs;
  globby: {
    sync: typeof globby.sync;
  };
  process: {
    cwd: typeof process.cwd;
    exit: typeof process.exit;
  };
  readYamlFile: {
    sync: typeof readYamlFile.sync<any>;
  };
}

export const IoTag = Context.Tag<Io>();

export const io: Io = {
  cosmiconfig: {
    cosmiconfig,
  },
  enquirer: {
    prompt,
  },
  fs: fs,
  globby: {
    sync: globby.sync,
  },
  process: {
    cwd: process.cwd,
    exit: process.exit,
  },
  readYamlFile: {
    sync: readYamlFile.sync,
  },
};
