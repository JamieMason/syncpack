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
          : inspect(value, false, null, true),
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

export function semverGroupHeader(semverGroup: SemverGroup, i: number): void {
  logHeader(
    semverGroup.isDefault ? 'Default Semver Group' : `Semver Group ${i + 1}`,
  );
}

export function versionGroupHeader(
  versionGroup: VersionGroup,
  i: number,
): void {
  logHeader(
    versionGroup.isDefault ? 'Default Version Group' : `Version Group ${i + 1}`,
  );
}

function logHeader(label: string) {
  const lead = `= ${label} `;
  console.log(chalk`{blue ${lead}${'='.repeat(80 - lead.length)}}`);
}
