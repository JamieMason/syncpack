import { fromHtml } from 'hast-util-from-html';
import { SKIP, visit } from 'unist-util-visit';

// https://discord.com/channels/830184174198718474/1396232862142824649
export function headingAnchorLinks() {
  return function transformer(tree) {
    visit(tree, 'element', (node, index, parent) => {
      if (!headingRank(node) || !node.properties.id || typeof index !== 'number' || !parent) {
        return;
      }

      const tagName = node.tagName;
      const id = String(node.properties.id);

      const html = `<${tagName} id="${id}"></${tagName}>`;
      const fragment = fromHtml(html, { fragment: true });
      const heading = fragment.children[0];

      // Restore the original children directly — avoids serializing to HTML
      // which breaks on MDX node types such as mdxJsxTextElement.
      heading.children = node.children;

      // Merge any extra properties added by earlier plugins (e.g. class names),
      // but keep the id we already set via the template.
      const { id: _id, ...extraProps } = node.properties;
      Object.assign(heading.properties, extraProps);

      parent.children[index] = heading;

      return SKIP;
    });
  };

  function headingRank(node) {
    const name = node.type === 'element' ? node.tagName.toLowerCase() : '';
    const code = name.length === 2 && name.charCodeAt(0) === 104 ? name.charCodeAt(1) : 0;
    return code > 48 && code < 55 ? code - 48 : undefined;
  }
}
