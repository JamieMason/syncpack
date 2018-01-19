import * as glob from 'glob';
import * as log from './log';

export const getFiles = (pattern: string): Promise<string[]> =>
  new Promise((resolve) =>
    glob(pattern, { absolute: true }, (err, files) => {
      if (err) {
        log.bug(`failed to search for files using pattern "${pattern}"`, err);
        process.exit(1);
      }
      resolve(files);
    })
  );
