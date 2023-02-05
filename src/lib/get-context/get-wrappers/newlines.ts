import { EOL } from 'os';

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
