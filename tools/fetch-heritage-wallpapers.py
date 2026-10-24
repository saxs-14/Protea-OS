#!/usr/bin/env python3
"""Fetch South African heritage/landscape wallpapers from Wikimedia Commons.

Uses the Wikimedia Commons API (not scraping arbitrary sites) so every image
carries a verifiable free licence and attribution, per Commons' API etiquette
policy: https://www.mediawiki.org/wiki/API:Etiquette

Writes images to assets/wallpapers/<site-slug>/ and a CREDITS.md listing the
title, author, licence and source URL for every downloaded file, since CC-BY
and CC-BY-SA licences require attribution to redistribute.

Usage:
    python tools/fetch-heritage-wallpapers.py [--per-site N] [--max-dimension PX]
"""
import argparse
import json
import pathlib
import re
import sys
import time
import urllib.parse

import requests
from PIL import Image

API = "https://commons.wikimedia.org/w/api.php"
USER_AGENT = (
    "ProteaOS-WallpaperFetcher/1.0 "
    "(https://github.com/saxs-14/Protea-OS; non-commercial OS project)"
)
FREE_LICENCES = (
    "cc0",
    "cc-by",
    "cc-by-sa",
    "public domain",
    "pd-",
)

# South African UNESCO World Heritage Sites plus a few widely recognised
# natural landmarks. Each entry is (slug, Commons category, search fallback).
SITES = [
    ("table-mountain", "Table Mountain", "Table Mountain South Africa"),
    ("robben-island", "Robben Island", "Robben Island South Africa"),
    ("cradle-of-humankind", "Cradle of Humankind", "Cradle of Humankind South Africa"),
    ("drakensberg", "Drakensberg", "Drakensberg mountains South Africa"),
    ("mapungubwe", "Mapungubwe", "Mapungubwe South Africa"),
    ("cape-floral-region", "Cape Floral Region Protected Areas", "Cape Floral Region fynbos"),
    ("isimangaliso-wetland-park", "iSimangaliso Wetland Park", "iSimangaliso Wetland Park"),
    ("richtersveld", "Richtersveld", "Richtersveld South Africa"),
    ("blyde-river-canyon", "Blyde River Canyon", "Blyde River Canyon South Africa"),
    ("kruger-national-park", "Kruger National Park", "Kruger National Park landscape"),
    ("cape-of-good-hope", "Cape of Good Hope", "Cape of Good Hope South Africa"),
    ("garden-route", "Garden Route (South Africa)", "Garden Route South Africa coast"),
]

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT_DIR = REPO_ROOT / "assets" / "wallpapers"


def api_get(params: dict) -> dict:
    params = {**params, "format": "json"}
    resp = requests.get(API, params=params, headers={"User-Agent": USER_AGENT}, timeout=30)
    resp.raise_for_status()
    return resp.json()


def category_files(category: str, limit: int) -> list[str]:
    data = api_get(
        {
            "action": "query",
            "list": "categorymembers",
            "cmtitle": f"Category:{category}",
            "cmtype": "file",
            "cmlimit": str(limit * 3),
        }
    )
    members = data.get("query", {}).get("categorymembers", [])
    return [m["title"] for m in members]


def search_files(term: str, limit: int) -> list[str]:
    data = api_get(
        {
            "action": "query",
            "list": "search",
            "srsearch": term,
            "srnamespace": "6",
            "srlimit": str(limit * 3),
        }
    )
    results = data.get("query", {}).get("search", [])
    return [r["title"] for r in results]


def file_info(title: str) -> dict | None:
    data = api_get(
        {
            "action": "query",
            "titles": title,
            "prop": "imageinfo",
            "iiprop": "url|size|extmetadata|mime",
        }
    )
    pages = data.get("query", {}).get("pages", {})
    for page in pages.values():
        infos = page.get("imageinfo")
        if infos:
            return infos[0]
    return None


def is_free_licence(extmetadata: dict) -> bool:
    licence_short = (extmetadata.get("LicenseShortName", {}).get("value") or "").lower()
    licence = (extmetadata.get("License", {}).get("value") or "").lower()
    text = f"{licence_short} {licence}"
    return any(token in text for token in FREE_LICENCES)


def sanitize(name: str) -> str:
    name = re.sub(r"^File:", "", name)
    name = re.sub(r"[^A-Za-z0-9._-]+", "_", name)
    return name[:120]


def resize_in_place(path: pathlib.Path, max_dimension: int) -> None:
    try:
        with Image.open(path) as img:
            if max(img.size) <= max_dimension:
                return
            img.thumbnail((max_dimension, max_dimension), Image.LANCZOS)
            img = img.convert("RGB") if img.mode in ("P", "RGBA") else img
            img.save(path, quality=88, optimize=True)
    except Exception as exc:  # noqa: BLE001 - keep the raw file rather than crash
        print(f"  ! could not resize {path.name}: {exc}", file=sys.stderr)


def fetch_site(slug: str, category: str, fallback_search: str, per_site: int, max_dimension: int, credits: list[str]) -> int:
    site_dir = OUT_DIR / slug
    site_dir.mkdir(parents=True, exist_ok=True)

    titles = category_files(category, per_site)
    source = f"Category:{category}"
    if len(titles) < per_site:
        titles += [t for t in search_files(fallback_search, per_site) if t not in titles]
        source = f"{source} + search:{fallback_search}"

    saved = 0
    for title in titles:
        if saved >= per_site:
            break
        time.sleep(0.3)  # be polite to the API
        info = file_info(title)
        if not info or "url" not in info:
            continue
        mime = info.get("mime", "")
        # Restrict to photographic formats - Commons also returns diagrams,
        # maps and logos (gif/svg) under the same categories/search terms,
        # which make poor desktop wallpapers.
        if mime not in ("image/jpeg", "image/png"):
            continue
        extmetadata = info.get("extmetadata", {})
        if not is_free_licence(extmetadata):
            continue

        url = info["url"]
        filename = sanitize(title)
        dest = site_dir / filename
        if dest.exists():
            saved += 1
            continue

        try:
            resp = requests.get(url, headers={"User-Agent": USER_AGENT}, timeout=60)
            resp.raise_for_status()
        except requests.RequestException as exc:
            print(f"  ! failed to download {title}: {exc}", file=sys.stderr)
            continue

        dest.write_bytes(resp.content)
        resize_in_place(dest, max_dimension)
        saved += 1

        author = extmetadata.get("Artist", {}).get("value") or "Unknown"
        author = re.sub(r"<[^>]+>", "", author).strip()
        licence_name = extmetadata.get("LicenseShortName", {}).get("value") or "Unknown licence"
        credits.append(
            f"- **{filename}** ({slug}) - {title} - "
            f"author: {author} - licence: {licence_name} - "
            f"source: https://commons.wikimedia.org/wiki/{urllib.parse.quote(title)}"
        )
        print(f"  + {filename}  ({licence_name})")

    print(f"{slug}: saved {saved}/{per_site} from {source}")
    return saved


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--per-site", type=int, default=3, help="max images per site (default: 3)")
    parser.add_argument("--max-dimension", type=int, default=2560, help="max width/height in px (default: 2560)")
    args = parser.parse_args()

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    credits: list[str] = []
    total = 0

    for slug, category, fallback in SITES:
        print(f"\n== {category} ==")
        total += fetch_site(slug, category, fallback, args.per_site, args.max_dimension, credits)

    credits_path = OUT_DIR / "CREDITS.md"
    with credits_path.open("w", encoding="utf-8") as f:
        f.write("# Wallpaper credits\n\n")
        f.write(
            "Every image below was fetched from Wikimedia Commons via its public API "
            "and carries a free licence (CC0, CC-BY, CC-BY-SA, or public domain). "
            "Attribution is required for CC-BY/CC-BY-SA images - keep this file "
            "alongside the images.\n\n"
        )
        f.write("\n".join(credits) + "\n")

    print(f"\nTotal images saved: {total}")
    print(f"Credits written to {credits_path}")


if __name__ == "__main__":
    main()
