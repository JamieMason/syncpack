import Layout from '@theme/Layout';
import React from 'react';
import { HomepageHeader } from '@site/src/components/homepage/header';
import css from './index.module.css';

export default function Home(): JSX.Element {
  return (
    <Layout
      title="Consistent dependency versions in large JavaScript Monorepos"
      description="A CLI tool to find and fix duplicate dependencies"
    >
      <HomepageHeader />
      <main>
        <iframe
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
          allowFullScreen
          className={css.video}
          src="https://www.youtube.com/embed/peJNp8BZ_dE"
          title="Fixing duplicate production dependencies in vercel/turborepo"
        ></iframe>
      </main>
    </Layout>
  );
}
