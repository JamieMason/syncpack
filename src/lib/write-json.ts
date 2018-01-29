import { writeFile } from 'fs';

export const writeJson = (location: string, contents: string): Promise<void> =>
  new Promise((resolve, reject) => {
    const json = `${JSON.stringify(contents, null, 2)}\n`;
    writeFile(location, json, { encoding: 'utf8' }, (err) => {
      err ? reject(err) : resolve();
    });
  });
