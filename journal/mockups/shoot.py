"""Pictures for the round-2 art tab of the design journal.

    ../../.venv/bin/python shoot.py models [id ...]     stills of each model
    ../../.venv/bin/python shoot.py abilities [id ...]  a still and a GIF of each ability
    ../../.venv/bin/python shoot.py lineups             the hero lineups
    ../../.venv/bin/python shoot.py scene <name>        a whole scene
    ../../.venv/bin/python shoot.py export              every model's parts, into tools/art2_models.json
    ../../.venv/bin/python shoot.py notes               every NOTES table, into ../art2/notes.json

Serves this folder on a free port, opens index.html in Chromium on the
graphics card, steps window.frame(t) itself and saves into ../art2/.
Fails on a console error or a page error. Not a check: it asserts nothing
about the pictures.
"""
import base64, functools, http.server, io, pathlib, sys, threading
from playwright.sync_api import sync_playwright
from PIL import Image

HERE = pathlib.Path(__file__).resolve().parent
import os
# ART=art3 writes a later round's pictures beside round 2's
OUT = HERE.parent / os.environ.get("ART", "art2")
# LOOK=ape draws in the new look (kit.js, journal tab 10)
LOOK = os.environ.get("LOOK", "")


class Quiet(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".js": "text/javascript"}

    def log_message(self, *a):
        pass


def grab(page):
    url = page.evaluate("document.getElementById('gl').toDataURL('image/png')")
    return Image.open(io.BytesIO(base64.b64decode(url.split(",", 1)[1]))).convert("RGB")


def main(kind, names):
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Quiet, directory=str(HERE)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    base = f"http://127.0.0.1:{server.server_address[1]}/index.html"
    errors = []
    with sync_playwright() as p:
        browser = p.chromium.launch(args=["--enable-gpu", "--ignore-gpu-blocklist", "--use-angle=metal"])
        page = browser.new_page(viewport={"width": 1400, "height": 900})
        page.on("console", lambda m: errors.append(m.text) if m.type == "error" and "Failed to load resource" not in m.text else None)
        page.on("pageerror", lambda e: errors.append(str(e)))

        def open_(qs):
            page.goto(f"{base}?{qs}" + (f"&look={LOOK}" if LOOK else ""))
            page.wait_for_selector("body[data-drawn='1'], body[data-error]", state="attached", timeout=60000)
            err = page.evaluate("document.body.dataset.error || ''")
            if err:
                errors.append(f"{qs}: {err}")
                return False
            return True

        if not names and kind in ("models", "abilities"):
            open_("lineup=hero_ironvow")
            names = page.evaluate("window.MODEL_IDS" if kind == "models" else "window.ABILITY_IDS")
        if kind == "models":
            (OUT / "models").mkdir(parents=True, exist_ok=True)
            for n in names:
                if open_(f"model={n}&w=1000&h=750&t=0.6"):
                    grab(page).save(OUT / "models" / f"{n}.jpg", quality=88)
                    print("saved", n)
        elif kind == "lineups":
            (OUT / "models").mkdir(parents=True, exist_ok=True)
            for n, ids in [("heroes_1", "hero_ironvow,hero_rime,hero_cinder,hero_thistle,hero_gristle"), ("heroes_2", "hero_tectus,hero_kestrel,hero_hexwright,hero_longwick,hero_tempest")] + [tuple(a.split("=", 1)) for a in names]:
                if open_(f"lineup={ids}&w=1600&h=700&t=0.6"):
                    grab(page).save(OUT / "models" / f"{n}.jpg", quality=88)
                    print("saved", n)
        elif kind == "abilities":
            (OUT / "abilities").mkdir(parents=True, exist_ok=True)
            for n in names:
                if not open_(f"ability={n}&w=1280&h=720&t=0"):
                    continue
                A = page.evaluate("window.ABILITY")
                page.evaluate(f"window.frame({A['still']})")
                grab(page).save(OUT / "abilities" / f"{n}.jpg", quality=88)
                fps = A["fps"]
                frames = []
                k = int(round(A["T"] * fps))
                for i in range(k):
                    page.evaluate(f"window.frame({i / fps})")
                    frames.append(grab(page).resize((480, 270), Image.LANCZOS))
                # one palette from a strip of every frame, so no frame's colours are lost
                strip = Image.new("RGB", (480, 270 * len(frames)))
                for i, f in enumerate(frames):
                    strip.paste(f, (0, 270 * i))
                pal = strip.quantize(colors=255, method=Image.Quantize.MEDIANCUT)
                q = [f.quantize(palette=pal, dither=Image.Dither.NONE) for f in frames]
                q[0].save(OUT / "abilities" / f"{n}.gif", save_all=True, append_images=q[1:], duration=int(1000 / fps), loop=0, optimize=True)
                print("saved", n, k, "frames")
        elif kind == "notes":
            open_("lineup=hero_ironvow")
            notes = page.evaluate("""async () => { const out = { models: {}, abilities: {} };
              for (const f of ['./heroes.js', './world.js']) { try { Object.assign(out.models, (await import(f)).NOTES || {}); } catch (e) {} }
              for (const f of ['./fx_a.js', './fx_b.js']) { try { const m = await import(f); Object.assign(out.abilities, m.NOTES || {});
                for (const [k, v] of Object.entries(m.ABILITIES)) { out.abilities[k] = { ...(out.abilities[k] || {}), T: v.T }; } } catch (e) {} }
              return out; }""")
            import json
            (OUT / "notes.json").write_text(json.dumps(notes, indent=1, ensure_ascii=False))
            print("saved notes:", len(notes["models"]), "models,", len(notes["abilities"]), "abilities")
        elif kind == "export":
            import json
            open_("lineup=hero_ironvow")
            data = page.evaluate("""async () => { const m = await import('./export.js'); return m.exportAll(window.build, window.MODEL_IDS); }""")
            dest = HERE.parents[1] / "tools" / "art2_models.json"
            dest.write_text(json.dumps(data, separators=(",", ":")))
            print("exported", len(data), "models to", dest)
        elif kind == "scene":
            (OUT / "scenes").mkdir(parents=True, exist_ok=True)
            for n in names:
                if open_(f"scene={n}&w=1600&h=900&fov=34&t=1"):
                    grab(page).save(OUT / "scenes" / f"{n}.jpg", quality=88)
                    print("saved", n)
        browser.close()
    server.shutdown()
    if errors:
        sys.exit("errors:\n" + "\n".join(errors))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2:])
