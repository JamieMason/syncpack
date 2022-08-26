import { isArray, isNonEmptyString } from 'expect-more';
import type { Disk } from '../lib/disk';
import type { ProgramInput } from '../lib/get-input';
import type { Source } from '../lib/get-input/get-wrappers';
import { writeIfChanged } from '../lib/write-if-changed';

export function format(input: ProgramInput, disk: Disk): void {
  const { indent, sortAz, sortFirst, wrappers } = input;

  wrappers.forEach(({ contents, filePath, json }) => {
    const sortedKeys = Object.keys(contents).sort();
    const keys = new Set<string>(sortFirst.concat(sortedKeys));

    const optionalChaining: any = contents;
    const bugsUrl = optionalChaining?.bugs?.url;
    const repoUrl = optionalChaining?.repository?.url;
    const repoDir = optionalChaining?.repository?.directory;

    if (bugsUrl) {
      contents.bugs = bugsUrl;
    }

    if (isNonEmptyString(repoUrl) && !isNonEmptyString(repoDir)) {
      contents.repository = repoUrl.includes('github.com')
        ? repoUrl.replace(/^.+github\.com\//, '')
        : repoUrl;
    }

    sortAz.forEach((key) => sortAlphabetically(contents[key]));
    sortObject(keys, contents);
    writeIfChanged(disk, { contents, filePath, indent, json });
  });

  function sortObject(
    sortedKeys: string[] | Set<string>,
    obj: Source | { [key: string]: string },
  ): void {
    sortedKeys.forEach((key: string) => {
      const value = obj[key];
      delete obj[key];
      obj[key] = value;
    });
  }

  function sortAlphabetically(value: Source[keyof Source]): void {
    if (isArray(value)) {
      value.sort();
    } else if (value && typeof value === 'object') {
      sortObject(Object.keys(value).sort(), value);
    }
  }
}
