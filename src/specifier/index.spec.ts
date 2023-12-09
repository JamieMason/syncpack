import { describe, expect, test } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario';

describe('supported version formats', () => {
  test.each([
    {
      name: 'alias',
      expected: 'AliasSpecifier',
      version: 'npm:bar',
    },
    {
      name: 'alias',
      expected: 'AliasSpecifier',
      version: 'npm:imageoptim-cli@3.1.7',
    },
    {
      name: 'file',
      expected: 'FileSpecifier',
      version: '/path/to/foo.tar',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+https://github.com/user/foo',
    },
    //
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com/user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com/user/foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com:user/foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com:user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:foo/bar#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:1234#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:1234/hey#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://mydomain.com:1234/hey',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://username:password@mydomain.com:1234/hey#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@github.com/user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com/user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@notgithub.com:user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@github.com/user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+ssh://git@github.com:user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'user/foo#path:dist',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'user/foo#1234::path:dist',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'user/foo#notimplemented:value',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git+file://path/to/repo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGitSpecifier',
      version: 'git://notgithub.com/user/foo',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '>1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '>=1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '<1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '<=1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '^1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '~1.0.0',
    },
    {
      name: 'range',
      expected: 'RangeSpecifier',
      version: '*',
    },
    {
      name: 'tag',
      expected: 'TagSpecifier',
      version: 'alpha',
    },
    {
      name: 'unsupported',
      expected: 'UnsupportedSpecifier',
      version: '$typescript',
    },
    {
      name: 'unsupported',
      expected: 'UnsupportedSpecifier',
      version: '',
    },
    {
      name: 'url',
      expected: 'UrlSpecifier',
      version: 'https://server.com/foo.tgz',
    },
    {
      name: 'version',
      expected: 'VersionSpecifier',
      version: '1.0.0',
    },
    {
      name: 'workspace-protocol',
      expected: 'WorkspaceProtocolSpecifier',
      version: 'workspace:*',
    },
    {
      name: 'workspace-protocol',
      expected: 'WorkspaceProtocolSpecifier',
      version: 'workspace:~',
    },
  ])('"$version" is a "$expected"', ({ name, version, expected }) => {
    const getScenario = createScenario({
      'package.json': {
        name,
        version: '0.0.0',
        dependencies: {
          foo: version,
        },
      },
    });
    const reports = getScenario().getVersionReports();
    const report = reports.find(({ name }) => name === 'foo');
    expect(report).toHaveProperty('reports.0._tag', 'Valid');
    expect(report).toHaveProperty('reports.0.specifier._tag', expected);
    expect(report).toHaveProperty('reports.0.specifier.raw', version);
  });
});
