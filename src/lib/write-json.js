import bluebird from 'bluebird';
import fs from 'fs';

export default async (location, contents) => {
  const json = JSON.stringify(contents, null, 2);
  const writeFile = bluebird.promisify(fs.writeFile);
  return await writeFile(location, `${json}\n`, { encoding: 'utf8' });
};
