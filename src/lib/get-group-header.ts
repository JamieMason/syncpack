import chalk from 'chalk';
import type { SemverGroup } from '../semver-group/index.js';
import type { VersionGroup } from '../version-group/index.js';

export function getSemverGroupHeader(input: { group: SemverGroup.Any; index: number }) {
  return getGroupHeader(input.group.config.label || `Semver Group ${input.index + 1}`);
}

export function getVersionGroupHeader(input: { group: VersionGroup.Any; index: number }) {
  return getGroupHeader(input.group.config.label || `Version Group ${input.index + 1}`);
}

function getGroupHeader(label: string) {
  const trimmed = label.trim();
  const hasNewLines = trimmed.search(/[\r\n]/) !== -1;
  const header = hasNewLines ? formatMultiLine(trimmed) : formatSingleLine(trimmed);
  return chalk.blue(header);
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
