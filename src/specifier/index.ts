import type { Instance } from '../get-instances/instance';
import { DELETE, type Delete } from '../version-group/lib/delete';
import { AliasSpecifier } from './alias';
import { DeleteSpecifier } from './delete';
import { ExactSpecifier } from './exact';
import { FileSpecifier } from './file';
import { HostedGitSpecifier } from './hosted-git';
import { LatestSpecifier } from './latest';
import { parseSpecifier } from './lib/parse-specifier';
import { RangeSpecifier } from './range';
import { TagSpecifier } from './tag';
import { UnsupportedSpecifier } from './unsupported';
import { UrlSpecifier } from './url';
import { WorkspaceProtocolSpecifier } from './workspace-protocol';

export namespace Specifier {
  export const Alias = AliasSpecifier;
  export const Delete = DeleteSpecifier;
  export const Exact = ExactSpecifier;
  export const File = FileSpecifier;
  export const HostedGit = HostedGitSpecifier;
  export const Latest = LatestSpecifier;
  export const Range = RangeSpecifier;
  export const Tag = TagSpecifier;
  export const Unsupported = UnsupportedSpecifier;
  export const Url = UrlSpecifier;
  export const WorkspaceProtocol = WorkspaceProtocolSpecifier;

  export type Any =
    | AliasSpecifier
    | DeleteSpecifier
    | ExactSpecifier
    | FileSpecifier
    | HostedGitSpecifier
    | LatestSpecifier
    | RangeSpecifier
    | TagSpecifier
    | UnsupportedSpecifier
    | UrlSpecifier
    | WorkspaceProtocolSpecifier;

  export function create(instance: Instance, raw: string | Delete): Specifier.Any {
    if (raw === DELETE) return new Specifier.Delete({ instance, raw });
    if (!raw) return new Specifier.Unsupported({ instance, raw });
    try {
      if (raw === 'latest') raw = '*';
      const parsed = parseSpecifier(instance.name, raw, instance.packageJsonFile);
      const type = parsed.type;
      const data = { instance, raw };
      if (raw === '*') return new Specifier.Latest(data);
      if (type === 'version') return new Specifier.Exact(data);
      if (type === 'range') return new Specifier.Range(data);
      if (type === 'workspaceProtocol') return new Specifier.WorkspaceProtocol(data);
      if (type === 'alias') return new Specifier.Alias(data);
      if (type === 'file' || type === 'directory') return new Specifier.File(data);
      if (type === 'remote') return new Specifier.Url(data);
      if (type === 'git') return new Specifier.HostedGit(data);
      if (type === 'tag') return new Specifier.Tag(data);
      return new Specifier.Unsupported(data);
    } catch {
      return new Specifier.Unsupported({ instance, raw });
    }
  }
}
