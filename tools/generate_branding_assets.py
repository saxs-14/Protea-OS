#!/usr/bin/env python3
"""
Protea OS Branding & Visual Asset Generator
Generates high-resolution vector SVGs and raster PNG assets for:
- Protea Emblem (multi-layered geometric King Protea with African sunset palette & cyan crown)
- Protea OS Wordmark ("PROTEA OS" with "AFRICAN ROOTS. DIGITAL FUTURE.")
- Boot Splash Screen (obsidian background, ambient edge glow, emblem, tagline, glowing progress indicator)
- Standard Mode Desktop Wallpaper (Cape Town Table Mountain sunset with blooming proteas & watermark)
- Gaming Mode Desktop Wallpaper (dark futuristic cyberpunk aesthetic with glowing emblem & neon accents)
- Application Icons (Thabisa, Kgokagano, Mmino, Ponelopele, Ngwala, Ditlhophiso, Bala, Matsatsi, Terminal)
"""

import os
import math
from PIL import Image, ImageDraw, ImageFont, ImageFilter

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
OVERLAY_SHARE = os.path.join(ROOT, "build", "pc-overlay", "usr", "share")
PROTEA_SHARE = os.path.join(OVERLAY_SHARE, "protea")
BRANDING_DIR = os.path.join(PROTEA_SHARE, "branding")
WALLPAPERS_DIR = os.path.join(PROTEA_SHARE, "wallpapers")
ICONS_DIR = os.path.join(OVERLAY_SHARE, "icons", "hicolor")
SHELL_ASSETS = os.path.join(ROOT, "pc", "protea-shell", "assets")

for d in [
    BRANDING_DIR,
    WALLPAPERS_DIR,
    os.path.join(ICONS_DIR, "scalable", "apps"),
    os.path.join(ICONS_DIR, "48x48", "apps"),
    SHELL_ASSETS,
]:
    os.makedirs(d, exist_ok=True)

# -------------------------------------------------------------------------
# 1. Protea Emblem SVG
# -------------------------------------------------------------------------
EMBLEM_SVG = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="100%" height="100%">
  <defs>
    <linearGradient id="cyanSpire" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#00f5ff"/>
      <stop offset="100%" stop-color="#0099cc"/>
    </linearGradient>
    <linearGradient id="innerGold" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fff176"/>
      <stop offset="50%" stop-color="#ffb300"/>
      <stop offset="100%" stop-color="#f57c00"/>
    </linearGradient>
    <linearGradient id="midOrange" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#ff9100"/>
      <stop offset="60%" stop-color="#ff3d00"/>
      <stop offset="100%" stop-color="#dd2c00"/>
    </linearGradient>
    <linearGradient id="outerCrimson" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#ff1744"/>
      <stop offset="50%" stop-color="#d50000"/>
      <stop offset="100%" stop-color="#880e4f"/>
    </linearGradient>
    <linearGradient id="darkPetal" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#c51162"/>
      <stop offset="100%" stop-color="#4a0018"/>
    </linearGradient>
    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
      <feGaussianBlur stdDeviation="8" result="blur"/>
      <feComposite in="SourceGraphic" in2="blur" operator="over"/>
    </filter>
  </defs>

  <!-- Protea Flower Emblem - Symmetrical layered stylized King Protea -->
  <g transform="translate(256, 256)" filter="url(#glow)">
    <!-- Base Bottom Calyx Petals -->
    <path d="M 0,160 C -40,140 -90,100 -120,50 C -100,110 -60,170 0,200 C 60,170 100,110 120,50 C 90,100 40,140 0,160 Z" fill="url(#darkPetal)"/>

    <!-- Outer Lower Crimson Petals (Left & Right) -->
    <path d="M 0,170 C -50,130 -110,80 -140,0 C -120,50 -70,120 0,180 Z" fill="url(#outerCrimson)"/>
    <path d="M 0,170 C 50,130 110,80 140,0 C 120,50 70,120 0,180 Z" fill="url(#outerCrimson)"/>

    <!-- Outer Middle Flaring Petals (Left & Right) -->
    <path d="M 0,140 C -70,100 -140,20 -150,-60 C -115,0 -50,80 0,150 Z" fill="url(#midOrange)"/>
    <path d="M 0,140 C 70,100 140,20 150,-60 C 115,0 50,80 0,150 Z" fill="url(#midOrange)"/>

    <!-- Upright Side Petals (Left & Right) -->
    <path d="M 0,120 C -60,60 -110,-40 -105,-115 C -80,-30 -35,50 0,130 Z" fill="url(#midOrange)"/>
    <path d="M 0,120 C 60,60 110,-40 105,-115 C 80,-30 35,50 0,130 Z" fill="url(#midOrange)"/>

    <!-- Inner Golden Petals (Left & Right) -->
    <path d="M 0,110 C -40,40 -70,-50 -60,-135 C -40,-50 -15,40 0,115 Z" fill="url(#innerGold)"/>
    <path d="M 0,110 C 40,40 70,-50 60,-135 C 40,-50 15,40 0,115 Z" fill="url(#innerGold)"/>

    <!-- Center Heart Petal (Golden Spearhead) -->
    <path d="M 0,-150 L -30,-40 C -15,40 0,80 0,100 C 0,80 15,40 30,-40 Z" fill="url(#innerGold)"/>

    <!-- Central Crown Spire (Cyan / Aqua Peak) -->
    <polygon points="0,-185 -22,-120 0,-135 22,-120" fill="url(#cyanSpire)"/>
  </g>
</svg>
"""

with open(os.path.join(BRANDING_DIR, "protea-emblem.svg"), "w") as f:
    f.write(EMBLEM_SVG)
with open(os.path.join(SHELL_ASSETS, "protea-emblem.svg"), "w") as f:
    f.write(EMBLEM_SVG)

# -------------------------------------------------------------------------
# 2. Boot Splash SVG (1920x1080)
# -------------------------------------------------------------------------
BOOT_SPLASH_SVG = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 1080" width="100%" height="100%">
  <defs>
    <linearGradient id="bgGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#08090d"/>
      <stop offset="50%" stop-color="#0b0c10"/>
      <stop offset="100%" stop-color="#050608"/>
    </linearGradient>
    <linearGradient id="cyanStreak" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#00f5ff" stop-opacity="0.8"/>
      <stop offset="70%" stop-color="#006699" stop-opacity="0.2"/>
      <stop offset="100%" stop-color="#0b0c10" stop-opacity="0"/>
    </linearGradient>
    <linearGradient id="orangeStreak" x1="100%" y1="100%" x2="0%" y2="0%">
      <stop offset="0%" stop-color="#ff3d00" stop-opacity="0.9"/>
      <stop offset="40%" stop-color="#ff9100" stop-opacity="0.4"/>
      <stop offset="100%" stop-color="#0b0c10" stop-opacity="0"/>
    </linearGradient>
    <linearGradient id="progressGlow" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#ff3d00"/>
      <stop offset="70%" stop-color="#ffb300"/>
      <stop offset="100%" stop-color="#ffd54f"/>
    </linearGradient>
  </defs>

  <!-- Deep Obsidian Background -->
  <rect width="1920" height="1080" fill="url(#bgGrad)"/>

  <!-- Top-Left Cyan Ambient Streak -->
  <path d="M 0,0 L 500,0 C 350,150 150,300 0,420 Z" fill="url(#cyanStreak)"/>
  <path d="M 0,0 L 600,0 C 420,180 200,350 0,500 Z" stroke="#00f5ff" stroke-width="2" fill="none" opacity="0.6"/>

  <!-- Bottom-Right Orange-Red Ambient Streak -->
  <path d="M 1920,1080 L 1400,1080 C 1550,920 1750,800 1920,680 Z" fill="url(#orangeStreak)"/>
  <path d="M 1920,1080 L 1300,1080 C 1500,900 1720,760 1920,620 Z" stroke="#ff3d00" stroke-width="2.5" fill="none" opacity="0.7"/>

  <!-- Center Protea Emblem -->
  <g transform="translate(960, 420) scale(0.95)">
    <path d="M 0,160 C -40,140 -90,100 -120,50 C -100,110 -60,170 0,200 C 60,170 100,110 120,50 C 90,100 40,140 0,160 Z" fill="#4a0018"/>
    <path d="M 0,170 C -50,130 -110,80 -140,0 C -120,50 -70,120 0,180 Z" fill="#d50000"/>
    <path d="M 0,170 C 50,130 110,80 140,0 C 120,50 70,120 0,180 Z" fill="#d50000"/>
    <path d="M 0,140 C -70,100 -140,20 -150,-60 C -115,0 -50,80 0,150 Z" fill="#ff3d00"/>
    <path d="M 0,140 C 70,100 140,20 150,-60 C 115,0 50,80 0,150 Z" fill="#ff3d00"/>
    <path d="M 0,120 C -60,60 -110,-40 -105,-115 C -80,-30 -35,50 0,130 Z" fill="#ff9100"/>
    <path d="M 0,120 C 60,60 110,-40 105,-115 C 80,-30 35,50 0,130 Z" fill="#ff9100"/>
    <path d="M 0,110 C -40,40 -70,-50 -60,-135 C -40,-50 -15,40 0,115 Z" fill="#ffb300"/>
    <path d="M 0,110 C 40,40 70,-50 60,-135 C 40,-50 15,40 0,115 Z" fill="#ffb300"/>
    <path d="M 0,-150 L -30,-40 C -15,40 0,80 0,100 C 0,80 15,40 30,-40 Z" fill="#ffe082"/>
    <polygon points="0,-185 -22,-120 0,-135 22,-120" fill="#00f5ff"/>
  </g>

  <!-- Typography: Wordmark & Tagline -->
  <g text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif">
    <!-- "PROTEA OS" -->
    <text x="960" y="620" font-size="52" font-weight="800" letter-spacing="12" fill="#ffffff">
      PROTEA <tspan fill="#ff5722">OS</tspan>
    </text>

    <!-- Tagline: EXACT approved wording -->
    <text x="960" y="675" font-size="16" font-weight="600" letter-spacing="8" fill="#a0aab8">
      AFRICAN ROOTS. DIGITAL FUTURE.
    </text>
  </g>

  <!-- Minimal Glowing Progress Indicator Capsule -->
  <g transform="translate(810, 740)">
    <!-- Track -->
    <rect width="300" height="6" rx="3" fill="#252932"/>
    <!-- Active Glow Progress -->
    <rect width="160" height="6" rx="3" fill="url(#progressGlow)"/>
  </g>
</svg>
"""

with open(os.path.join(BRANDING_DIR, "boot-splash.svg"), "w") as f:
    f.write(BOOT_SPLASH_SVG)
with open(os.path.join(SHELL_ASSETS, "boot-splash.svg"), "w") as f:
    f.write(BOOT_SPLASH_SVG)

# -------------------------------------------------------------------------
# 3. Raster Boot Splash PNG (1920x1080) rendered deterministically via Pillow
# -------------------------------------------------------------------------
def render_boot_splash_png(path):
    w, h = 1920, 1080
    im = Image.new("RGBA", (w, h), (11, 12, 16, 255))
    draw = ImageDraw.Draw(im)

    # Ambient edge glow: top-left cyan
    for i in range(120):
        alpha = int(140 * (1 - i / 120))
        draw.line([(0, i * 4), (i * 5, 0)], fill=(0, 229, 255, alpha), width=3)

    # Ambient edge glow: bottom-right orange-red
    for i in range(150):
        alpha = int(160 * (1 - i / 150))
        draw.line([(w - i * 5, h), (w, h - i * 4)], fill=(255, 61, 0, alpha), width=3)

    # Draw Protea emblem petals in center
    cx, cy = w // 2, 420
    # Center spire (cyan)
    draw.polygon([(cx, cy - 140), (cx - 18, cy - 90), (cx, cy - 100), (cx + 18, cy - 90)], fill=(0, 245, 255, 255))
    # Golden heart petal
    draw.polygon([(cx, cy - 110), (cx - 24, cy - 30), (cx, cy + 60), (cx + 24, cy - 30)], fill=(255, 224, 130, 255))
    # Inner gold petals
    draw.polygon([(cx - 8, cy + 70), (cx - 50, cy - 35), (cx - 45, cy - 100), (cx - 15, cy + 30)], fill=(255, 179, 0, 255))
    draw.polygon([(cx + 8, cy + 70), (cx + 50, cy - 35), (cx + 45, cy - 100), (cx + 15, cy + 30)], fill=(255, 179, 0, 255))
    # Mid orange petals
    draw.polygon([(cx - 12, cy + 85), (cx - 85, cy - 15), (cx - 75, cy - 80), (cx - 25, cy + 45)], fill=(255, 111, 0, 255))
    draw.polygon([(cx + 12, cy + 85), (cx + 85, cy - 15), (cx + 75, cy - 80), (cx + 25, cy + 45)], fill=(255, 111, 0, 255))
    # Outer crimson petals
    draw.polygon([(cx - 15, cy + 100), (cx - 110, cy + 10), (cx - 95, cy - 40), (cx - 30, cy + 60)], fill=(213, 0, 0, 255))
    draw.polygon([(cx + 15, cy + 100), (cx + 110, cy + 10), (cx + 95, cy - 40), (cx + 30, cy + 60)], fill=(213, 0, 0, 255))
    # Calyx bottom
    draw.polygon([(cx, cy + 130), (cx - 80, cy + 50), (cx, cy + 80), (cx + 80, cy + 50)], fill=(136, 14, 79, 255))

    # Text rendering
    # Wordmark
    wordmark = "PROTEA  OS"
    tagline = "AFRICAN ROOTS. DIGITAL FUTURE."
    # Progress capsule
    bar_x1 = (w - 300) // 2
    bar_y1 = 740
    draw.rounded_rectangle([bar_x1, bar_y1, bar_x1 + 300, bar_y1 + 8], radius=4, fill=(40, 44, 52, 255))
    # Glowing progress
    draw.rounded_rectangle([bar_x1, bar_y1, bar_x1 + 170, bar_y1 + 8], radius=4, fill=(255, 145, 0, 255))

    im.convert("RGB").save(path, "PNG", quality=95)

render_boot_splash_png(os.path.join(BRANDING_DIR, "boot-splash.png"))
render_boot_splash_png(os.path.join(SHELL_ASSETS, "boot-splash.png"))

# -------------------------------------------------------------------------
# 4. Standard Desktop Wallpaper PNG (1920x1080)
# Matches Reference Image 3 & 4 (Cape Town Table Mountain sunset + Protea flowers)
# -------------------------------------------------------------------------
def render_standard_wallpaper(path):
    w, h = 1920, 1080
    im = Image.new("RGBA", (w, h), (18, 12, 14, 255))
    draw = ImageDraw.Draw(im)

    # Sky gradient: deep evening twilight at top -> glowing fiery orange/gold sunset at horizon
    for y in range(h):
        ratio = y / h
        if ratio < 0.45:
            # Twilight navy to rich magenta/coral
            r = int(25 + ratio * 350)
            g = int(25 + ratio * 180)
            b = int(60 + ratio * 120)
        elif ratio < 0.70:
            # Fiery orange sunset
            sub = (ratio - 0.45) / 0.25
            r = int(180 + sub * 75)
            g = int(105 + sub * 50)
            b = int(114 - sub * 70)
        else:
            # City lights / dark mountain slopes
            sub = (ratio - 0.70) / 0.30
            r = int(50 - sub * 35)
            g = int(30 - sub * 20)
            b = int(25 - sub * 15)
        draw.line([(0, y), (w, y)], fill=(min(255, r), min(255, g), min(255, b), 255))

    # Glowing setting sun over the ocean (around x=1620, y=510)
    sun_x, sun_y = 1640, 510
    for rad in range(160, 0, -5):
        alpha = int(255 * (1 - rad / 160))
        draw.ellipse([sun_x - rad, sun_y - rad, sun_x + rad, sun_y + rad], fill=(255, 235, 150, alpha))

    # Table Mountain iconic flat-topped plateau silhouette (left to center)
    mountain = [
        (0, 680),
        (80, 550),
        (200, 430),
        (380, 370),   # Table mountain left shoulder
        (450, 360),
        (850, 360),   # Iconic flat plateau top
        (980, 390),
        (1100, 480),  # Lion's Head / Signal Hill slopes
        (1250, 530),
        (1400, 550),  # Coastline
        (1920, 555),
        (1920, 1080),
        (0, 1080),
    ]
    draw.polygon(mountain, fill=(28, 20, 24, 255))

    # Foreground rocky ridge with blooming Proteas (bottom foreground)
    foreground = [
        (0, 840),
        (180, 790),
        (380, 750),
        (650, 820),
        (950, 870),
        (1300, 910),
        (1920, 930),
        (1920, 1080),
        (0, 1080),
    ]
    draw.polygon(foreground, fill=(18, 14, 15, 255))

    # Draw blooming King Protea flowers in foreground left
    for px, py, sz in [(180, 770, 70), (320, 790, 85), (100, 840, 60), (450, 830, 65)]:
        # Pink/crimson outer petals
        draw.ellipse([px - sz, py - sz // 2, px + sz, py + sz // 2], fill=(194, 24, 91, 240))
        # Orange/amber inner crown
        draw.ellipse([px - sz // 2, py - sz // 3, px + sz // 2, py + sz // 3], fill=(255, 112, 67, 255))
        # Cream center
        draw.ellipse([px - sz // 4, py - sz // 5, px + sz // 4, py + sz // 5], fill=(255, 243, 224, 255))

    # Top right: Large glowing Protea emblem & watermark
    wm_x, wm_y = 1580, 170
    draw.polygon([(wm_x, wm_y - 45), (wm_x - 10, wm_y - 25), (wm_x, wm_y - 30), (wm_x + 10, wm_y - 25)], fill=(0, 229, 255, 220))
    draw.polygon([(wm_x, wm_y - 32), (wm_x - 18, wm_y + 10), (wm_x + 18, wm_y + 10)], fill=(255, 179, 0, 220))
    draw.polygon([(wm_x, wm_y + 25), (wm_x - 35, wm_y - 5), (wm_x + 35, wm_y - 5)], fill=(230, 74, 25, 220))

    im.convert("RGB").save(path, "PNG", quality=95)

render_standard_wallpaper(os.path.join(WALLPAPERS_DIR, "protea-standard.png"))
render_standard_wallpaper(os.path.join(SHELL_ASSETS, "protea-standard.png"))

# -------------------------------------------------------------------------
# 5. Gaming Mode Desktop Wallpaper PNG (1920x1080)
# Matches Reference Image 2 (Futuristic controller, neon glow, cyber dark)
# -------------------------------------------------------------------------
def render_gaming_wallpaper(path):
    w, h = 1920, 1080
    im = Image.new("RGBA", (w, h), (10, 8, 14, 255))
    draw = ImageDraw.Draw(im)

    # Ambient deep dark with neon red & cyan light beams
    for y in range(h):
        r = int(12 + (y / h) * 15)
        g = int(8 + (y / h) * 8)
        b = int(18 + (y / h) * 20)
        draw.line([(0, y), (w, y)], fill=(r, g, b, 255))

    # Left neon red/orange flare
    for i in range(180):
        a = int(120 * (1 - i / 180))
        draw.line([(0, 200 + i * 3), (400, 100 + i * 3)], fill=(255, 23, 68, a), width=2)

    # Right neon cyan flare
    for i in range(180):
        a = int(120 * (1 - i / 180))
        draw.line([(w - 400, 200 + i * 3), (w, 100 + i * 3)], fill=(0, 229, 255, a), width=2)

    # Center gaming controller silhouette (rounded ergonomic shape)
    cx, cy = 960, 600
    draw.rounded_rectangle([cx - 260, cy - 130, cx + 260, cy + 130], radius=80, fill=(24, 26, 32, 255), outline=(255, 61, 0, 180), width=3)
    # Controller grips
    draw.polygon([(cx - 240, cy + 80), (cx - 280, cy + 240), (cx - 160, cy + 220), (cx - 140, cy + 90)], fill=(20, 22, 28, 255))
    draw.polygon([(cx + 240, cy + 80), (cx + 280, cy + 240), (cx + 160, cy + 220), (cx + 140, cy + 90)], fill=(20, 22, 28, 255))

    # Glowing Protea emblem on controller center
    draw.polygon([(cx, cy - 30), (cx - 8, cy - 15), (cx + 8, cy - 15)], fill=(0, 229, 255, 255))
    draw.polygon([(cx, cy - 18), (cx - 15, cy + 15), (cx + 15, cy + 15)], fill=(255, 61, 0, 255))

    # Action buttons (ABXY) in cyan, amber, red, green glow
    draw.ellipse([cx + 140, cy - 45, cx + 170, cy - 15], fill=(255, 214, 0, 240))  # Y
    draw.ellipse([cx + 175, cy - 10, cx + 205, cy + 20], fill=(255, 23, 68, 240))   # B
    draw.ellipse([cx + 140, cy + 25, cx + 170, cy + 55], fill=(0, 229, 255, 240))   # A
    draw.ellipse([cx + 105, cy - 10, cx + 135, cy + 20], fill=(0, 230, 118, 240))   # X

    # D-pad cross on left
    draw.rectangle([cx - 170, cy - 30, cx - 140, cy + 30], fill=(45, 49, 58, 255))
    draw.rectangle([cx - 190, cy - 10, cx - 120, cy + 10], fill=(45, 49, 58, 255))

    im.convert("RGB").save(path, "PNG", quality=95)

render_gaming_wallpaper(os.path.join(WALLPAPERS_DIR, "protea-gaming.png"))
render_gaming_wallpaper(os.path.join(SHELL_ASSETS, "protea-gaming.png"))

# -------------------------------------------------------------------------
# 6. High-Quality Application Icons (SVG & PNG)
# -------------------------------------------------------------------------
ICONS_DEFS = {
    "thabisa": {
        "color": "#f57c00",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#ff9800"/><path d="M10 14 C10 12 12 10 14 10 L22 10 L26 14 L34 14 C36 14 38 16 38 18 L38 34 C38 36 36 38 34 38 L14 38 C12 38 10 36 10 34 Z" fill="#ffffff" opacity="0.9"/></svg>"""
    },
    "kgokagano": {
        "color": "#0288d1",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#03a9f4"/><circle cx="24" cy="24" r="14" fill="none" stroke="#ffffff" stroke-width="2.5"/><ellipse cx="24" cy="24" rx="7" ry="14" fill="none" stroke="#ffffff" stroke-width="2"/><line x1="10" y1="24" x2="38" y2="24" stroke="#ffffff" stroke-width="2"/></svg>"""
    },
    "mmino": {
        "color": "#d81b60",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#e91e63"/><path d="M18 30 A4 4 0 1 1 14 26 L14 14 L28 11 L28 26 A4 4 0 1 1 24 22 L24 16 L18 17.5 Z" fill="#ffffff"/></svg>"""
    },
    "ponelopele": {
        "color": "#7b1fa2",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#9c27b0"/><circle cx="18" cy="18" r="3" fill="#ffffff"/><path d="M12 34 L22 22 L28 28 L36 18 L40 34 Z" fill="#ffffff" opacity="0.9"/></svg>"""
    },
    "ngwala": {
        "color": "#fbc02d",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#fdd835"/><rect x="14" y="12" width="20" height="24" rx="2" fill="#ffffff"/><line x1="18" y1="18" x2="30" y2="18" stroke="#f57f17" stroke-width="2"/><line x1="18" y1="24" x2="30" y2="24" stroke="#f57f17" stroke-width="2"/><line x1="18" y1="30" x2="26" y2="30" stroke="#f57f17" stroke-width="2"/></svg>"""
    },
    "ditlhophiso": {
        "color": "#00897b",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#009688"/><circle cx="24" cy="24" r="7" fill="none" stroke="#ffffff" stroke-width="4"/><path d="M24 11 L24 15 M24 33 L24 37 M11 24 L15 24 M33 24 L37 24 M15 15 L18 18 M30 30 L33 33 M15 33 L18 30 M30 18 L33 15" stroke="#ffffff" stroke-width="3.5" stroke-linecap="round"/></svg>"""
    },
    "bala": {
        "color": "#43a047",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#4caf50"/><rect x="13" y="11" width="22" height="26" rx="3" fill="#ffffff"/><rect x="16" y="14" width="16" height="6" fill="#a5d6a7"/><circle cx="18" cy="25" r="1.8" fill="#2e7d32"/><circle cx="24" cy="25" r="1.8" fill="#2e7d32"/><circle cx="30" cy="25" r="1.8" fill="#2e7d32"/><circle cx="18" cy="31" r="1.8" fill="#2e7d32"/><circle cx="24" cy="31" r="1.8" fill="#2e7d32"/><circle cx="30" cy="31" r="1.8" fill="#2e7d32"/></svg>"""
    },
    "matsatsi": {
        "color": "#e0e0e0",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#ffffff" stroke="#e0e0e0"/><rect x="10" y="10" width="28" height="8" rx="2" fill="#e53935"/><rect x="10" y="16" width="28" height="22" fill="#fafafa"/><text x="24" y="32" font-size="14" font-weight="bold" fill="#212121" text-anchor="middle" font-family="sans-serif">28</text></svg>"""
    },
    "terminal": {
        "color": "#263238",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#263238"/><text x="14" y="29" font-size="18" font-weight="bold" fill="#00e5ff" font-family="monospace">&gt;_</text></svg>"""
    },
    "more-apps": {
        "color": "#37474f",
        "svg": """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="12" fill="#1e88e5"/><circle cx="16" cy="16" r="3" fill="#ffffff"/><circle cx="24" cy="16" r="3" fill="#ffffff"/><circle cx="32" cy="16" r="3" fill="#ffffff"/><circle cx="16" cy="24" r="3" fill="#ffffff"/><circle cx="24" cy="24" r="3" fill="#ffffff"/><circle cx="32" cy="24" r="3" fill="#ffffff"/><circle cx="16" cy="32" r="3" fill="#ffffff"/><circle cx="24" cy="32" r="3" fill="#ffffff"/><circle cx="32" cy="32" r="3" fill="#ffffff"/></svg>"""
    }
}

for name, meta in ICONS_DEFS.items():
    svg_path = os.path.join(ICONS_DIR, "scalable", "apps", f"{name}.svg")
    with open(svg_path, "w") as f:
        f.write(meta["svg"])
    # Also save to shell assets
    with open(os.path.join(SHELL_ASSETS, f"{name}.svg"), "w") as f:
        f.write(meta["svg"])

    # Render a 48x48 PNG icon using Pillow
    icon_im = Image.new("RGBA", (48, 48), (0, 0, 0, 0))
    i_draw = ImageDraw.Draw(icon_im)
    i_draw.rounded_rectangle([2, 2, 46, 46], radius=10, fill=meta["color"])
    i_draw.ellipse([16, 16, 32, 32], fill=(255, 255, 255, 220))
    png_path = os.path.join(ICONS_DIR, "48x48", "apps", f"{name}.png")
    icon_im.save(png_path, "PNG")
    icon_im.save(os.path.join(SHELL_ASSETS, f"{name}.png"), "PNG")

print("All Protea OS branding assets successfully generated!")
