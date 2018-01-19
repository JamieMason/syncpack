import { writeFile } from 'fs';

export const writeJson = (location: string, contents: string): Promise<void> =>
  new Promise((resolve, reject) => {
    const json = JSON.stringify(contents, null, 2);
    writeFile(location, `${json}\n`, { encoding: 'utf8' }, (err) => {
      if (err) {
        reject(err);
      } else {
        resolve();
      }
    });
  });
