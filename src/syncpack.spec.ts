import { sync } from 'glob';
import { getMockCommander } from '../test/helpers';
import { run } from './syncpack';

it('registers each command', () => {
  const commands = [
    ...sync('bin-*.ts', { cwd: __dirname }),
    ...sync('!*.spec.ts', { cwd: __dirname }),
  ];
  const program = getMockCommander([]);
  const spy = jest.spyOn(program, 'command');
  const commandNames = commands.map((basename) =>
    basename.replace(/bin\-|\.ts/g, ''),
  );

  run(program);
  expect(commands.length).toBeGreaterThan(0);
  expect(spy).toHaveBeenCalledTimes(commands.length);

  commandNames.forEach((name) => {
    expect(spy.mock.calls).toEqual(
      expect.arrayContaining([
        expect.arrayContaining([name, expect.any(String)]),
      ]),
    );
  });
});
