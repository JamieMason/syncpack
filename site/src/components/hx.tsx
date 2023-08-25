import type { ReactNode } from 'react';
import React from 'react';

interface Props {
  children: ReactNode;
  level: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';
}

/** Set a custom header level */
export function Hx({ children, level: Heading }: Props) {
  return <Heading>{children}</Heading>;
}
