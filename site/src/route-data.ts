import { defineRouteMiddleware } from '@astrojs/starlight/route-data';

export const onRequest = defineRouteMiddleware(context => {
  context.locals.starlightRoute.head.some(item => {
    if (item.attrs?.rel === 'sitemap') {
      item.attrs.href = '/syncpack/sitemap.xml';
      return true;
    }
  });
});
