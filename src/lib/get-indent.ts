import { OPTION_INDENT } from '../constants';
import { CommanderApi } from '../typings';

export type GetIndent = (program: CommanderApi) => string;

export const getIndent: GetIndent = (program) =>
  program.indent || OPTION_INDENT.default;
