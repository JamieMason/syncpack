import * as glob from 'glob';

export const getFiles = (pattern: string): Promise<string[]> =>
  new Promise((resolve, reject) =>
    glob(pattern, { absolute: true }, (err, files) => {
      err ? reject(err) : resolve(files);
    })
  );
