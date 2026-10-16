#!/usr/bin/env python3
"""Screenshots of the built game for the design journal: still scenes the
core sets up (`?tile=<spec>`) and the menus and HUD as a player sees them.
Not a check: it asserts nothing, it saves pictures and prints the console.

    .venv/bin/python tools/shots.py OUT_DIR name=spec [name=spec ...]

A spec is a tile (`lineup:a,b`, `overview`, `match:<seconds>`, `map:<x>,<y>`)
or a screen path (`screen:arcade`, `screen:heroes`, `screen:pick`).
SOFT=1 renders without the graphics card.
"""
import functools, http.server, os, pathlib, sys, threading
from playwright.sync_api import sync_playwright

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = pathlib.Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)
SHOTS = [a.split("=", 1) for a in sys.argv[2:]]
W, H = [int(v) for v in os.environ.get("SIZE", "1600x900").split("x")]


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
    logs = []
    pg.on("console", lambda m: logs.append(f"{m.type}: {m.text}"))
    pg.on("pageerror", lambda e: logs.append(f"pageerror: {e}"))
    for name, spec in SHOTS:
        if spec.startswith("screen:"):
            pg.goto(base)
            pg.wait_for_selector("body[data-ready='true']", state="attached", timeout=60000)
            for step in spec.split(":", 1)[1].split("/"):
                kind, _, value = step.partition("=")
                if kind == "click":
                    pg.click(value)
                elif kind == "wait":
                    pg.wait_for_timeout(int(value))
                elif kind == "key":
                    pg.keyboard.press(value)
                elif kind == "hover":
                    pg.hover(value)
                else:
                    pg.click(f"[data-a='screen'][data-v='{kind}']")
                pg.wait_for_timeout(250)
        else:
            pg.goto(f"{base}?tile={spec}")
            pg.wait_for_selector("body[data-tile='drawn']", state="attached", timeout=180000)
        pg.wait_for_timeout(int(os.environ.get("SETTLE", "900")))
        pg.screenshot(path=str(OUT / f"{name}.png"))
        print("saved", name)
    for line in logs:
        print(line)
    b.close()
