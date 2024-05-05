import { EOL } from 'os';
import { getIndent } from '../config/get-indent.js';
import type { Ctx } from '../get-context/index.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';

type Ending = '\n' | '\r' | '\r\n' | string;

const CR = '\r';
const CRLF = '\r\n';
const LF = '\n';

export const newlines = {
  detect(source: string): Ending {
    const cr = source.split(CR).length;
    const lf = source.split(LF).length;
    const crlf = source.split(CRLF).length;
    if (cr + lf === 0) return EOL;
    if (crlf === cr && crlf === lf) return CRLF;
    if (cr > lf) return CR;
    return LF;
  },
  fix(source: string, lineEnding: Ending): string {
    return source.replace(/\r\n|\n|\r/g, lineEnding);
  },
};

export function toJson(ctx: Ctx, file: PackageJsonFile): string {
  const contents = file.jsonFile.contents;
  const indent = getIndent(ctx.config);
  const EOL = newlines.detect(file.jsonFile.json);
  const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
  return newlines.fix(source, EOL);
}
