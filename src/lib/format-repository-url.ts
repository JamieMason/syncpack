export function formatRepositoryUrl(
  input: string | undefined,
): string | undefined {
  if (!input) {
    return undefined;
  }

  const extractedUrl = input.match(/https?:\/\/[^\s]+/)?.[0];
  if (extractedUrl) {
    const withoutSuffix = removeSuffix(extractedUrl);

    return withoutSuffix;
  }

  const isSshProtocol = input.startsWith('git@');
  if (isSshProtocol) {
    const withoutAffix = removeSuffix(removePrefix(input, 'git@'));

    const [origin, path] = withoutAffix.split(':');

    if (!(origin && path)) {
      return undefined;
    }

    return `https://${origin}/${path}`;
  }

  const isShortcut = input.split('/').length === 2;
  if (isShortcut) {
    return `https://github.com/${input}`;
  }

  const isGitProtocol = input.startsWith('git://');
  if (isGitProtocol) {
    const withoutAffix = removeSuffix(removePrefix(input));

    return `https://${withoutAffix}`;
  }
}

function removeSuffix(url: string, suffix = '.git') {
  if (url.endsWith(suffix)) {
    return url.slice(0, url.length - suffix.length);
  }

  return url;
}

function removePrefix(url: string, prefix = 'git://') {
  if (url.startsWith(prefix)) {
    return url.slice(prefix.length, url.length);
  }

  return url;
}
