#!/usr/bin/env python3
"""Animated pictures of abilities, from the built game itself (round 7,
journal tab 12): the cast stage (`?tile=cast:<ability>,<ticks>`) shot at
every few ticks and put together as a GIF, with a still of the most
readable frame. Not a check: it asserts nothing about the pictures.

    .venv/bin/python tools/castgif.py OUT_DIR ability[:frames[:step[:still]]] ...

frames defaults to 30, step (ticks between frames) to 2, still (the frame
saved as the JPG) to the middle one. SOFT=1 renders without the graphics
card.
"""
import functools, http.server, io, os, pathlib, sys, threading
from playwright.sync_api import sync_playwright
from PIL import Image

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = pathlib.Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)
W, H = 1280, 720


class Quiet(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def log_message(self, *a):
        pass


srv = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Quiet, directory=str(ROOT / "dist")))
threading.Thread(target=srv.serve_forever, daemon=True).start()
base = f"http://127.0.0.1:{srv.server_address[1]}/index.html"

with sync_playwright() as p:
    soft = ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"]
    gpu = ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"]
    b = p.chromium.launch(args=soft if os.environ.get("SOFT") else gpu)
    pg = b.new_page(viewport={"width": W, "height": H})
    for arg in sys.argv[2:]:
        parts = arg.split(":")
        aid = parts[0]
        frames = int(parts[1]) if len(parts) > 1 else 30
        step = int(parts[2]) if len(parts) > 2 else 2
        still = int(parts[3]) if len(parts) > 3 else frames // 2
        shots = []
        for i in range(frames):
            pg.goto(f"{base}?tile=cast:{aid},{1 + i * step},close")
            pg.wait_for_selector("body[data-tile='drawn']", state="attached", timeout=60000)
            pg.wait_for_timeout(120)
            shots.append(Image.open(io.BytesIO(pg.screenshot())).convert("RGB"))
        shots[min(still, frames - 1)].save(OUT / f"{aid}.jpg", quality=86)
        small = [s.resize((480, 270), Image.LANCZOS) for s in shots]
        strip = Image.new("RGB", (480, 270 * len(small)))
        for k, s in enumerate(small):
            strip.paste(s, (0, 270 * k))
        pal = strip.quantize(colors=255, method=Image.Quantize.MEDIANCUT)
        q = [s.quantize(palette=pal, dither=Image.Dither.NONE) for s in small]
        q[0].save(OUT / f"{aid}.gif", save_all=True, append_images=q[1:], duration=int(1000 * step / 30), loop=0, optimize=True)
        print("saved", aid, frames, "frames")
    b.close()
