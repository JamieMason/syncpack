import { defineCollection, z } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';
import { glob } from 'astro/loaders'; // Not available with legacy API

const oldStartPage = 'https://jamiemason.github.io/syncpack/guide/getting-started/';
const v13Docs = `https://web.archive.org/web/20250217193908/${oldStartPage}`;
const v14 = 'https://github.com/JamieMason/syncpack?tab=readme-ov-file#syncpack';
const banner = `This documentation is for <a href="${v14}">v14</a>, the <a href="${v13Docs}" rel="external nofollow">docs for v13</a> are archived.`;

const schema = docsSchema({
  extend: z.object({
    banner: z.object({ content: z.string() }).default({
      content: banner,
    }),
    schemaOverride: z.enum(['article', 'faq', 'howto']).optional(),
  }),
});

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    schema,
  }),
  faq: defineCollection({
    loader: glob({ pattern: '**/*.mdx', base: './src/faq' }),
    schema,
  }),
};
