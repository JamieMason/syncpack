import { describe, expect, test } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';

describe('supported version formats', () => {
  test.each([
    {
      name: 'alias',
      expected: 'Alias',
      version: 'npm:bar',
    },
    {
      name: 'alias',
      expected: 'Alias',
      version: 'npm:imageoptim-cli@3.1.7',
    },
    {
      name: 'file',
      expected: 'File',
      version: '/path/to/foo.tar',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+https://github.com/user/foo',
    },
    //
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com/user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com/user/foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com:user/foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:foo',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com:user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:foo/bar#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:1234#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:1234/hey#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://mydomain.com:1234/hey',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://username:password@mydomain.com:1234/hey#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@github.com/user/foo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com/user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@notgithub.com:user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@github.com/user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+ssh://git@github.com:user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'user/foo#semver:^1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'user/foo#path:dist',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'user/foo#1234::path:dist',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'user/foo#notimplemented:value',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git+file://path/to/repo#1.2.3',
    },
    {
      name: 'hosted-git',
      expected: 'HostedGit',
      version: 'git://notgithub.com/user/foo',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '>1.0.0',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '>=1.0.0',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '<1.0.0',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '<=1.0.0',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '^1.0.0',
    },
    {
      name: 'range',
      expected: 'Range',
      version: '~1.0.0',
    },
    {
      name: 'latest',
      expected: 'Latest',
      version: '*',
    },
    {
      name: 'tag',
      expected: 'Tag',
      version: 'alpha',
    },
    {
      name: 'unsupported',
      expected: 'Unsupported',
      version: '$typescript',
    },
    {
      name: 'unsupported',
      expected: 'Unsupported',
      version: '',
    },
    {
      name: 'url',
      expected: 'Url',
      version: 'https://server.com/foo.tgz',
    },
    {
      name: 'version',
      expected: 'Exact',
      version: '1.0.0',
    },
    {
      name: 'workspace-protocol',
      expected: 'WorkspaceProtocol',
      version: 'workspace:*',
    },
    {
      name: 'workspace-protocol',
      expected: 'WorkspaceProtocol',
      version: 'workspace:~',
    },
    {
      name: 'workspace-protocol',
      expected: 'WorkspaceProtocol',
      version: 'workspace:^',
    },
  ])('"$version" is a "$expected"', async ({ name, version, expected }) => {
    const getScenario = createScenario({
      'package.json': {
        name,
        version: '0.0.0',
        dependencies: {
          foo: version,
        },
      },
    });
    const reports = await getScenario().getVersionReports();
    const report = reports.find(({ name }) => name === 'foo');
    expect(report).toHaveProperty('reports.0._tag', 'Valid');
    expect(report).toHaveProperty('reports.0.specifier._tag', expected);
    expect(report).toHaveProperty('reports.0.specifier.raw', version);
  });
});
