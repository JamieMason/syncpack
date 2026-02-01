import { siteConfig } from "./siteConfig";

type StarlightRouteData = any;

/**
 * Build all applicable schemas for a given Starlight route.
 *
 * This is the main entry point called from Head.astro.
 */
export function buildSchemas(route: StarlightRouteData) {
  const schemas: Record<string, any>[] = [];

  const isHomepage = route.id === "";
  const isDocPage = !isHomepage;

  if (isHomepage) {
    schemas.push(buildPersonSchema());
    schemas.push(buildSoftwareApplicationSchema());
  }

  if (isDocPage) {
    schemas.push(buildBreadcrumbSchema(route));
    schemas.push(buildArticleSchema(route));
  }

  return schemas;
}

/**
 * Person schema for homepage (Jamie Mason as author).
 * https://schema.org/Person
 */
function buildPersonSchema() {
  return {
    "@context": "https://schema.org",
    "@type": "Person",
    name: siteConfig.authorName,
    url: siteConfig.authorUrl,
    image: siteConfig.authorImage,
    sameAs: siteConfig.authorSocial,
    funding: {
      "@type": "MonetaryGrant",
      url: siteConfig.sponsorUrl,
    },
  };
}

/**
 * SoftwareApplication schema for homepage.
 * https://schema.org/SoftwareApplication
 */
function buildSoftwareApplicationSchema() {
  return {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    name: siteConfig.siteName,
    description: siteConfig.softwareDescription,
    url: siteConfig.siteUrl,
    image: siteConfig.logoUrl,
    applicationCategory: "DeveloperApplication",
    operatingSystem: ["Linux", "macOS", "Windows"],
    author: {
      "@type": "Person",
      name: siteConfig.authorName,
      url: siteConfig.authorUrl,
    },
    funding: siteConfig.sponsorUrl,
    downloadUrl: siteConfig.npmUrl,
  };
}

/**
 * BreadcrumbList schema for documentation pages.
 * https://schema.org/BreadcrumbList
 */
function buildBreadcrumbSchema(route: StarlightRouteData) {
  const breadcrumbs = generateBreadcrumbs(route.slug);

  return {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: breadcrumbs.map((item, i) => ({
      "@type": "ListItem",
      position: i + 1,
      name: item.name,
      item: `${siteConfig.siteUrl}${item.url}`,
    })),
  };
}

/**
 * Article schema for documentation pages.
 * https://schema.org/Article
 */
function buildArticleSchema(route: StarlightRouteData) {
  const { data } = route.entry;
  const title = data.title || "Syncpack Documentation";
  const description = data.description || siteConfig.description;

  // Starlight provides lastUpdated from Git when lastUpdated: true in config
  const dateModified =
    data.lastUpdated?.toISOString() || new Date().toISOString();

  // Use the OG image for this specific page
  const ogImageUrl = `${siteConfig.siteUrl}/og/${route.id}.png`;

  return {
    "@context": "https://schema.org",
    "@type": "Article",
    headline: title,
    description: description,
    datePublished: dateModified, // Use same as modified for docs
    dateModified: dateModified,
    author: {
      "@type": "Person",
      name: siteConfig.authorName,
      url: siteConfig.authorUrl,
    },
    publisher: {
      "@type": "Person",
      name: siteConfig.authorName,
      url: siteConfig.authorUrl,
      image: siteConfig.authorImage,
    },
    image: ogImageUrl,
    inLanguage: "en",
  };
}

/**
 * Generate breadcrumb trail from slug.
 *
 * Example: "guide/getting-started" becomes:
 * - Docs → /syncpack/
 * - Guide → /syncpack/guide/
 * - Getting Started → /syncpack/guide/getting-started/
 */
function generateBreadcrumbs(slug: string) {
  const breadcrumbs = [{ name: "Docs", url: "/" }];

  const parts = slug.split("/").filter(Boolean);
  let currentPath = "";

  parts.forEach((part) => {
    currentPath += `/${part}`;
    breadcrumbs.push({
      name: titleCase(part),
      url: `${currentPath}/`,
    });
  });

  return breadcrumbs;
}

/**
 * Convert kebab-case to Title Case.
 *
 * Example: "getting-started" → "Getting Started"
 */
function titleCase(str: string) {
  return str
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}
