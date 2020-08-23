import { SyncpackConfig } from '../constants';
import { getWrappers, Source, SourceWrapper } from './lib/get-wrappers';
import { writeIfChanged } from './lib/write-if-changed';

type Options = Pick<SyncpackConfig, 'indent' | 'sortAz' | 'sortFirst' | 'source'>;

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

export const format = (wrapper: SourceWrapper, options: Options): Source => {
  const { sortAz, sortFirst } = options;
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

export const formatToDisk = (options: Options): void => {
  getWrappers({ source: options.source }).forEach((wrapper) => {
    writeIfChanged(options.indent, wrapper, () => {
      format(wrapper, options);
    });
  });
};
