import Link from '@docusaurus/Link';
import { filterDocCardListItems, useCurrentSidebarCategory } from '@docusaurus/theme-common';
import type { Props as CardProps } from '@theme/DocCard';
import DocCard from '@theme/DocCard';
import type { Props } from '@theme/DocCardList';
import React from 'react';

function DocCardListForCurrentSidebarCategory({ className }: Props) {
  const category = useCurrentSidebarCategory();
  return <DocCardList items={category.items} className={className} />;
}

export default function DocCardList(props: Props): JSX.Element {
  const { items, className } = props;
  if (!items) {
    return <DocCardListForCurrentSidebarCategory {...props} />;
  }
  const filteredItems = filterDocCardListItems(items);
  return (
    <section className={className}>
      {filteredItems.map((item, index) => (
        <article key={index}>
          <Item item={item} />
        </article>
      ))}
    </section>
  );
}

function Item({ item }: CardProps) {
  switch (item.type) {
    case 'link':
      return (
        <Link to={item.docId}>
          <h2>{item.label}</h2>
        </Link>
      );
    case 'category':
      return <DocCard item={item} />;
    default:
      throw new Error(`unknown item type ${JSON.stringify(item)}`);
  }
}
