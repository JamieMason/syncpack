import { defineRouteMiddleware } from '@astrojs/starlight/route-data';

// Build custom title with breadcrumb structure
function getCustomTitle(route: any): string {
  if (!route) return 'Syncpack';

  // Homepage special case
  if (route.id === '') {
    return 'Consistent dependency versions in JavaScript Monorepos | Syncpack';
  }

  const pageTitle = route.entry.data.title;

  // Special case: Status Codes overview page (reference/status-codes with slug: status)
  // Title is "Status Codes" so avoid duplicate
  if (route.id === 'reference/status-codes' || (route.slug === 'status' && pageTitle === 'Status Codes')) {
    return `${pageTitle} | Syncpack`;
  }

  // Map directory to sidebar label
  const categoryMap: Record<string, string> = {
    command: 'Commands',
    'version-groups': 'Version Groups',
    'semver-groups': 'Semver Groups',
    config: 'Configuration File',
    reference: 'Reference',
    guide: 'Guides',
    status: 'Status Codes',
  };

  const parts = route.id.split('/');

  if (parts.length >= 1) {
    const category = categoryMap[parts[0]];
    if (category) {
      // Skip category if it matches the page title (for index pages)
      if (category === pageTitle) {
        return `${pageTitle} | Syncpack`;
      }
      return `${pageTitle} | ${category} | Syncpack`;
    }
  }

  return `${pageTitle} | Syncpack`;
}

export const onRequest = defineRouteMiddleware(context => {
  const route = context.locals.starlightRoute;

  // Add custom title tag
  const customTitle = getCustomTitle(route);

  // Remove existing title tag(s) from head
  route.head = route.head.filter(item => item.tag !== 'title');

  // Add our custom title tag
  route.head.push({
    tag: 'title',
    content: customTitle,
  });
});
