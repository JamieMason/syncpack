import { writeFile } from 'fs';

export const formatJson = (contents: object) => `${JSON.stringify(contents, null, 2)}\n`;

export const writeJson = (location: string, contents: object): Promise<void> =>
  new Promise((resolve, reject) => {
    writeFile(location, formatJson(contents), { encoding: 'utf8' }, (err) => {
      err ? reject(err) : resolve();
    });
  });
