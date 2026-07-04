# Novus Writer - Brand Guidelines

## Overview

**Novus Writer** is a modern, professional desktop writing application for Linux. The name "Novus" comes from Latin meaning "new", "fresh", or "innovative", representing a modern approach to document writing while remaining professional.

---

## Logo Assets

### Primary Logo
- **File:** `novus-writer-logo.svg`
- **Usage:** Main application icon, website header, marketing materials
- **Colors:** Royal Blue (#1E4FBF), Navy (#163A8C), White, Light Gray, Silver
- **Background:** Soft white gradient

### Dark Theme Logo
- **File:** `novus-writer-logo-dark.svg`
- **Usage:** Dark mode interfaces, dark backgrounds
- **Colors:** Adapted for dark backgrounds with lighter blue tones

### Monochrome Logo
- **File:** `novus-writer-logo-mono.svg`
- **Usage:** Single-color printing, embossing, engraving, fax documents
- **Note:** Uses `currentColor` for easy theming

### Symbol-Only Logo
- **File:** `novus-writer-symbol.svg`
- **Usage:** Favicon, app launcher, small sizes (16x16 to 64x64)
- **Note:** Simplified version without the "N" letter for better scalability

---

## Color Palette

### Primary Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Royal Blue | `#1E4FBF` | rgb(30, 79, 191) | Primary brand color, "N" letter |
| Navy | `#163A8C` | rgb(22, 58, 140) | Pen body, accents |

### Secondary Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| White | `#FFFFFF` | rgb(255, 255, 255) | Document background |
| Light Gray | `#F3F4F6` | rgb(243, 244, 246) | Background gradient |
| Silver | `#C9CED6` | rgb(201, 206, 214) | Pen accents, metallic elements |
| Dark Gray | `#3A3A3A` | rgb(58, 58, 58) | Text lines, details |

### Dark Theme Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Dark BG 1 | `#2D3748` | rgb(45, 55, 72) | Dark background gradient |
| Dark BG 2 | `#1A202C` | rgb(26, 32, 44) | Dark background gradient |
| Light Blue | `#4299E1` | rgb(66, 153, 225) | "N" letter in dark mode |
| Medium Gray | `#4A5568` | rgb(74, 85, 104) | Text lines in dark mode |

---

## Typography

### Logo Font
- **Primary:** Georgia
- **Fallback:** Times New Roman, serif
- **Weight:** 700 (Bold)
- **Style:** Classic, professional serif

### Application UI Fonts (Recommended)
- **Primary:** Inter, system-ui
- **Headings:** Georgia, serif
- **Code:** JetBrains Mono, Fira Code, monospace

---

## Logo Usage Guidelines

### Clear Space
Maintain a minimum clear space around the logo equal to 10% of the logo height on all sides.

### Minimum Size
- **Full logo:** 32×32 pixels
- **Symbol only:** 16×16 pixels
- **Print:** 1 inch (2.54 cm) minimum width

### Do Not
- ❌ Stretch or distort the logo
- ❌ Change the colors outside the brand palette
- ❌ Add effects like drop shadows or glows
- ❌ Rotate the logo
- ❌ Place on busy backgrounds
- ❌ Use low-resolution versions
- ❌ Modify the proportions

### Acceptable Variations
- ✅ Full-color version (primary)
- ✅ Dark theme version
- ✅ Monochrome version
- ✅ Symbol-only version
- ✅ Reversed (light on dark)

---

## Icon Scalability

The logo is designed to remain recognizable at all standard sizes:

| Size | Usage | Version Recommended |
|------|-------|---------------------|
| 16×16 | Favicon, toolbar | Symbol only |
| 32×32 | App launcher, taskbar | Symbol only |
| 64×64 | Desktop shortcut | Symbol only |
| 128×128 | Settings, about dialog | Full logo |
| 256×256 | Application icon | Full logo |
| 512×512 | Marketing, website | Full logo |
| 1024×1024 | App stores, banners | Full logo |

---

## Brand Personality

### Core Values
1. **Professional** - Serious tool for serious writers
2. **Modern** - Contemporary design and technology
3. **Reliable** - Stable, offline-first, no telemetry
4. **Creative** - Empowers writers and authors
5. **Simple** - Intuitive interface, easy to use
6. **Fast** - Optimized performance, low resource usage

### Tone of Voice
- Clear and direct
- Professional but approachable
- Technical when needed, simple when possible
- Respectful of users' intelligence

---

## Application in UI

### Title Bar
Use the symbol-only logo at 24×24 or 32×32 pixels next to the application title.

### About Dialog
Display the full-color logo prominently with version information.

### Documentation
Use the full-color logo on covers and the monochrome version inside documents.

### Website
- Header: Full-color logo
- Favicon: Symbol-only
- Social media: Full-color logo
- Print materials: Monochrome or full-color depending on context

---

## File Formats

All logo files are provided in SVG format for infinite scalability. For raster formats:

- Export PNG files at 2× the target size for Retina/HiDPI displays
- Use lossless compression
- Maintain transparency where applicable

### Generating PNGs
```bash
# Example using Inkscape
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-512.png -w 512 -h 512
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-256.png -w 256 -h 256
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-128.png -w 128 -h 128
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-64.png -w 64 -h 64
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-32.png -w 32 -h 32
inkscape novus-writer-logo.svg --export-filename=novus-writer-logo-16.png -w 16 -h 16
```

---

## Trademark Notice

Novus Writer™ is a trademark of the Novus Writer Project.

The name "Novus Writer" and the logo are protected trademarks. When using these assets:

- Always use the correct spelling: "Novus Writer" (capital N, capital W)
- Do not use as a verb or in plural form
- Include trademark notice in official documentation when appropriate

---

## Contact

For questions about brand usage, licensing, or modifications, please refer to the project repository or contact the maintainers.

---

## License

The Novus Writer logo and brand assets are licensed under [LICENSE TO BE DETERMINED].

© 2024 Novus Writer Project. All rights reserved.
