
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from urllib.request import Request, urlopen
from urllib.error import HTTPError, URLError

BASE_DIR = Path(__file__).resolve().parent
SPRITE_DIR = BASE_DIR / "assets" / "pokemon"

BASE_URL = (
    "https://raw.githubusercontent.com/PokeAPI/sprites/master/"
    "sprites/pokemon/versions/generation-iii/emerald"
)

PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def download_sprite(dex_number, shiny=False):
    folder = SPRITE_DIR / "shiny" if shiny else SPRITE_DIR
    folder.mkdir(parents=True, exist_ok=True)

    destination = folder / f"{dex_number}.png"

    # Keep existing valid sprites.
    if destination.exists():
        try:
            if destination.read_bytes().startswith(PNG_SIGNATURE):
                return "skipped"
        except OSError:
            pass

    variant = "shiny/" if shiny else ""
    url = f"{BASE_URL}/{variant}{dex_number}.png"

    request = Request(
        url,
        headers={"User-Agent": "EmeraldIVScanner/1.0"}
    )

    for attempt in range(3):
        try:
            with urlopen(request, timeout=20) as response:
                data = response.read()

            if not data.startswith(PNG_SIGNATURE):
                return "invalid"

            temporary = destination.with_suffix(".tmp")
            temporary.write_bytes(data)
            temporary.replace(destination)

            return "downloaded"

        except HTTPError as error:
            if error.code == 404:
                return "missing"
            if attempt == 2:
                return "failed"

        except (URLError, TimeoutError, OSError):
            if attempt == 2:
                return "failed"

    return "failed"


def main():
    SPRITE_DIR.mkdir(parents=True, exist_ok=True)
    (SPRITE_DIR / "shiny").mkdir(exist_ok=True)

    jobs = [
        (dex, shiny)
        for dex in range(1, 387)
        for shiny in (False, True)
    ]

    results = {
        "downloaded": 0,
        "skipped": 0,
        "missing": 0,
        "invalid": 0,
        "failed": 0,
    }

    print("Downloading Pokémon Emerald sprites...")
    print("National Dex range: 1–386")
    print("Variants: Normal + Shiny")
    print()

    with ThreadPoolExecutor(max_workers=6) as executor:
        futures = {
            executor.submit(download_sprite, dex, shiny): (dex, shiny)
            for dex, shiny in jobs
        }

        for index, future in enumerate(as_completed(futures), start=1):
            dex, shiny = futures[future]

            try:
                result = future.result()
            except Exception:
                result = "failed"

            results[result] += 1

            if index % 25 == 0 or index == len(jobs):
                print(
                    f"Progress: {index}/{len(jobs)} "
                    f"| Downloaded: {results['downloaded']} "
                    f"| Skipped: {results['skipped']} "
                    f"| Missing: {results['missing']} "
                    f"| Failed: {results['failed']}"
                )

    print("\n========== DOWNLOAD SUMMARY ==========")

    for name, count in results.items():
        print(f"{name.capitalize():12}: {count}")

    print(f"\nSprite folder: {SPRITE_DIR}")


if __name__ == "__main__":
    main()
