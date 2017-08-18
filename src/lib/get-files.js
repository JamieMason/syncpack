import glob from 'glob';
import * as log from './log';

export default pattern =>
  new Promise(resolve =>
    glob(pattern, { absolute: true }, (err, files) => {
      if (err) {
        log.bug(`failed to search for files using pattern "${pattern}"`, err);
        process.exit(1);
      }
      resolve(files);
    })
  );
