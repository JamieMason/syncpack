export function sectionWrapper() {
  // Recursively wrap headings at a given depth
  function wrapHeadingsAtDepth(children, depth) {
    const newChildren = [];
    let i = 0;

    while (i < children.length) {
      const node = children[i];

      // When we hit a heading at this depth, collect it + following nodes until next heading at same/higher level
      if (node.type === 'heading' && node.depth === depth) {
        const sectionChildren = [node];
        i++;

        while (i < children.length) {
          const next = children[i];
          if (next.type === 'heading' && next.depth <= depth) {
            break;
          }
          sectionChildren.push(next);
          i++;
        }

        // Extract heading text for aria-labelledby (including text from inline code)
        const headingText = node.children
          .map(child => {
            if (child.type === 'text') return child.value;
            if (child.type === 'inlineCode') return child.value;
            return '';
          })
          .join('');

        // Create slug from heading text (match Starlight's slugification)
        const slug = headingText
          .toLowerCase()
          .trim()
          .replace(/\s+/g, '-')
          .replace(/[^\w-]/g, '');

        // Recursively wrap child headings if this is not h4
        const wrappedChildren = depth < 4 ? wrapHeadingsAtDepth(sectionChildren, depth + 1) : sectionChildren;

        // Wrap heading + siblings in containerDirective that becomes <section>
        newChildren.push({
          type: 'containerDirective',
          name: 'section',
          data: {
            hName: 'section',
            hProperties: {
              'aria-labelledby': slug,
            },
          },
          children: wrappedChildren,
        });
      } else {
        newChildren.push(node);
        i++;
      }
    }

    return newChildren;
  }

  return tree => {
    tree.children = wrapHeadingsAtDepth(tree.children, 2);
  };
}
