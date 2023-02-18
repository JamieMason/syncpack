import chalk from 'chalk';
import { isString } from 'expect-more';
import { inspect } from 'util';
import { ICON } from '../constants';
import type { SemverGroup } from '../get-context/get-groups/semver-group';
import type { VersionGroup } from '../get-context/get-groups/version-group';

export function verbose(...values: unknown[]): void {
  if (process.env.SYNCPACK_VERBOSE) {
    console.info(
      chalk.yellow(ICON.debug),
      ...values.map((value) =>
        isString(value)
          ? chalk.yellow(value)
          : inspect(value, { showHidden: false, colors: true, depth: 20 }),
      ),
    );
  }
}

export function fixed(message: string): void {
  console.log(chalk`{green ${ICON.tick}}`, message);
}

export function skip(message: string): void {
  console.log(chalk.dim(ICON.skip), chalk.dim(message));
}

export function valid(message: string, comment?: string): void {
  if (comment) {
    console.log(chalk`{dim ${ICON.skip}} ${message} {dim ${comment}}`);
  } else {
    console.log(chalk`{dim ${ICON.skip}} ${message}`);
  }
}

export function invalid(message: string, comment?: string): void {
  if (comment) {
    console.log(chalk`{red ${ICON.cross}} ${message} {dim ${comment}}`);
  } else {
    console.log(chalk`{red ${ICON.cross}} ${message}`);
  }
}

export function semverGroupHeader(group: SemverGroup, i: number): void {
  logHeader(group, 'Semver', i);
}

export function versionGroupHeader(group: VersionGroup, i: number): void {
  logHeader(group, 'Version', i);
}

function logHeader(
  group: VersionGroup | SemverGroup,
  type: 'Semver' | 'Version',
  i: number,
) {
  const customLabel = group.groupConfig.label;
  const labelWhenDefault = group.isDefault ? `Default ${type} Group` : '';
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
