import chalk from 'chalk';
import { isString } from 'tightrope/guard/is-string';
import { inspect } from 'util';
import { ICON } from '../constants';
import type { AnySemverGroup } from '../get-semver-groups';
import type { AnyVersionGroup } from '../get-version-groups';

export function verbose(...values: unknown[]): void {
  /* istanbul ignore if */
  if (process.env.SYNCPACK_VERBOSE) {
    console.info(
      chalk.yellow(ICON.debug),
      ...values.map((value) =>
        isString(value)
          ? chalk.yellow(value)
          : inspect(value, {
              colors: true,
              compact: true,
              depth: 20,
              showHidden: false,
            }),
      ),
    );
  }
}

export function semverGroupHeader(group: AnySemverGroup, i: number): void {
  logHeader(group, 'Semver', i);
}

export function versionGroupHeader(group: AnyVersionGroup, i: number): void {
  logHeader(group, 'Version', i);
}

function logHeader(
  group: AnyVersionGroup | AnySemverGroup,
  type: 'Semver' | 'Version',
  i: number,
) {
  const customLabel = group.config.label;
  const labelWhenDefault =
    group._tag === 'CatchAll' ? `Default ${type} Group` : '';
  const anonymousLabel = `${type} Group ${i + 1}`;
  const label = (customLabel || labelWhenDefault || anonymousLabel).trim();
  const hasNewLines = label.search(/[\r\n]/) !== -1;
  const header = hasNewLines ? formatMultiLine(label) : formatSingleLine(label);
  console.log(chalk.blue(header));
}

function formatSingleLine(label: string): string {
  const leftSide = `= ${label} `;
  const dividerWidth = 80 - leftSide.length;
  const rightSide = dividerWidth > 0 ? '='.repeat(dividerWidth) : '';
  return `${leftSide}${rightSide}`;
}

function formatMultiLine(label: string): string {
  const reindented = label.replace(/^\s+/gm, '  ');
  return `= ${reindented}`;
}
