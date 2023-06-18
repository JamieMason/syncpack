import chalk from 'chalk';
import type { AnySemverGroup } from '../get-semver-groups';
import type { AnyVersionGroup } from '../get-version-groups';

export const logGroupHeader = {
  semverGroup(group: AnySemverGroup, i: number): void {
    log(group, 'Semver', i);
  },
  versionGroup(group: AnyVersionGroup, i: number): void {
    log(group, 'Version', i);
  },
};

function log(group: AnyVersionGroup | AnySemverGroup, type: 'Semver' | 'Version', i: number) {
  const customLabel = group.config.label;
  const labelWhenDefault = group.isCatchAll === true ? `Default ${type} Group` : '';
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
