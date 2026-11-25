# PC Box Wallpapers

Extracted 24 individual wallpaper panels from `background-pc.png`.

- 4 columns × 6 rows
- Zero-based, row-major indices: 0–23
- Each extracted PNG is 154 × 114 pixels
- Crops contain the large wallpaper panel only; small header previews, dark frame/gutters, and bottom attribution are excluded.
- Preserve pixel-art sharpness when scaling (CSS `image-rendering: pixelated` if appropriate).

Example mapping:
- index 0 -> wallpaper_00.png
- index 5 -> wallpaper_05.png
- index 23 -> wallpaper_23.png

See `manifest.json` for the complete index mapping.
