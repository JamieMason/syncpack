import fs from 'fs';
import bluebird from 'bluebird';

export default (location, contents) => {
  const json = JSON.stringify(contents, null, 2);
  const writeFile = bluebird.promisify(fs.writeFile);
  return writeFile(location, `${json}\n`, { encoding: 'utf8' });
};
