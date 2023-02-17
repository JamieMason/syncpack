import React from 'react';
import css from './styles.module.css';

export default function Pills({ required, optional }) {
  return (
    <p>
      {required && <span className={`${css.pill} ${css.red}`}>Required</span>}
      {optional && <span className={`${css.pill} ${css.gray}`}>Optional</span>}
    </p>
  );
}
