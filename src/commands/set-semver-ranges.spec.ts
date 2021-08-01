import 'expect-more-jest';
import * as mock from '../../test/mock';
import { DEFAULT_CONFIG } from '../constants';
import { setSemverRanges } from './set-semver-ranges';

describe('setSemverRanges', () => {
  it('sets all versions to use the supplied range', () => {
    const wrapper = mock.wrapper('a', ['foo@0.1.0', 'bar@2.0.0']);
    setSemverRanges(wrapper, { ...DEFAULT_CONFIG, dev: false, peer: false, prod: true, semverRange: '~' });
    expect(wrapper).toEqual(mock.wrapper('a', ['foo@~0.1.0', 'bar@~2.0.0']));
  });
});
