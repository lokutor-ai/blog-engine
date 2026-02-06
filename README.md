# web-blog

A high-performance, SEO-focused static site generator for bloggers, written in Rust. It transforms Markdown files into a complete, deployment-ready website with support for taxonomies, pagination, and live reloading.

## Features

- **Fast Builds**: Parallelized processing using Rayon.
- **SEO Ready**: Automatic generation of `sitemap.xml` and `rss.xml`.
- **Search**: Generates a `search.json` index for client-side search implementation.
- **Taxonomies**: Built-in support for tags and categories.
- **Pagination**: Configurable post-per-page limits for index pages.
- **Live Reload**: Development server that rebuilds on file changes.
- **Theming**: Flexible HTML templating using the Tera engine.

## Installation

Ensure you have the Rust toolchain installed. Clone the repository and build the binary:

```bash
cargo build --release
```

The binary will be available at `./target/release/web-blog`.

## Quick Start

1. **Initialize a new project**:
   ```bash
   web-blog new my-awesome-blog
   cd my-awesome-blog
   ```

2. **Start the development server**:
   ```bash
   web-blog serve --drafts
   ```
   Open `http://localhost:3000` in your browser.

3. **Build for production**:
   ```bash
   web-blog build --output ./dist
   ```

## Project Structure

```text
.
├── config.toml          # Site-wide configuration
├── content/             # Markdown source files
│   └── posts/           # Your blog posts
├── static/              # Assets (CSS, JS, Images) copied directly to output
└── themes/
    └── default/         # Tera HTML templates
        ├── index.html   # Homepage and pagination
        ├── post.html    # Individual post layout
        └── taxonomy.html # Tag and Category archives
```

## Configuration (`config.toml`)

```toml
title = "My Rust Blog"
base_url = "https://example.com"
description = "Thoughts on systems programming"
posts_per_page = 5
```

## Writing Posts

Create `.md` files in `content/posts/`. Each file requires YAML frontmatter:

```markdown
---
title: "Optimizing Rust Builds"
date: 2026-02-06
slug: "optimizing-rust-builds"
tags: ["rust", "performance"]
categories: ["engineering"]
draft: false
---

# Your Content Here
```

## CLI Usage

- `new <path>`: Scaffolds a new project directory.
- `build`: Compiles the site to the output folder.
  - `-i, --input`: Input directory (default: `.`)
  - `-o, --output`: Output directory (default: `public`)
  - `-d, --drafts`: Include posts marked as `draft: true`.
- `serve`: Starts a local server with auto-reloading.
  - `-p, --port`: Port to listen on (default: `3000`)
  - `-d, --drafts`: Include drafts during development.

## License

MIT / Apache-2.0
