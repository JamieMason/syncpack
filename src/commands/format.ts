import { SORT_AZ, SORT_FIRST } from '../constants';
import { getWrappers, SourceWrapper, Source } from './lib/get-wrappers';
import { writeIfChanged } from './lib/write-if-changed';

interface FormatConfig {
  sortAz?: string[];
  sortFirst?: string[];
}

interface Options {
  indent: string;
  sources: string[];
}

const sortObject = (sortedKeys: string[] | Set<string>, obj: Source | { [key: string]: string }): void => {
  sortedKeys.forEach((key: string) => {
    const value = obj[key];
    delete obj[key];
    obj[key] = value;
  });
};

const sortAlphabetically = (value: Source[keyof Source]): void => {
  if (Array.isArray(value)) {
    value.sort();
  } else if (value && typeof value === 'object') {
    sortObject(Object.keys(value).sort(), value);
  }
};

export const format = (
  wrapper: SourceWrapper,
  { sortAz = SORT_AZ, sortFirst = SORT_FIRST }: FormatConfig = {},
): Source => {
  const { contents } = wrapper;
  const sortedKeys = Object.keys(contents).sort();
  const keys = new Set<string>(sortFirst.concat(sortedKeys));

  if (contents.bugs && typeof contents.bugs === 'object' && contents.bugs.url) {
    contents.bugs = contents.bugs.url;
  }

  if (contents.repository && typeof contents.repository === 'object' && contents.repository.url) {
    if (contents.repository.url.includes('github.com')) {
      contents.repository = contents.repository.url.replace(/^.+github\.com\//, '');
    } else {
      contents.repository = contents.repository.url;
    }
  }

  sortAz.forEach((key) => sortAlphabetically(contents[key]));
  sortObject(keys, contents);
  return contents;
};

export const formatToDisk = ({ indent, sources: sources }: Options): void => {
  getWrappers({ sources }).forEach((wrapper) => {
    writeIfChanged(indent, wrapper, () => {
      format(wrapper);
    });
  });
};
