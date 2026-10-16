"""Frame rate of a custom 5v5 against computer players, with the camera on
the middle of the map once the waves meet. A probe, not a check: it prints
frames a second and asserts nothing. MEASUREMENTS.md records its output.

    .venv/bin/python tests/browser/fps.py [seconds_in]
"""
import functools, http.server, pathlib, sys, threading
from playwright.sync_api import sync_playwright

ROOT = pathlib.Path(__file__).resolve().parents[2]


class Q(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def log_message(self, *a):
        pass


s = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Q, directory=str(ROOT / "dist")))
threading.Thread(target=s.serve_forever, daemon=True).start()
wait = int(sys.argv[1]) if len(sys.argv) > 1 else 90
with sync_playwright() as p:
    b = p.chromium.launch(args=["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"])
    pg = b.new_page(viewport={"width": 1600, "height": 900})
    pg.goto(f"http://127.0.0.1:{s.server_address[1]}/index.html")
    pg.wait_for_selector("body[data-ready='true']", state="attached")
    pg.click("[data-a='screen'][data-v='arcade']")
    for side in ("allies", "enemies"):
        for _ in range(5):
            pg.click(f"[data-a='setup.{side}'][data-v='+']")
    pg.click("[data-a='custom']")
    pg.click(".hero-card")
    pg.click("[data-a='start']")
    pg.wait_for_selector("body[data-screen='match']", state="attached")
    for t in (5, wait):
        pg.wait_for_timeout((t - (5 if t == wait else 0)) * 1000)
        box = pg.locator("#minimap").bounding_box()
        pg.mouse.click(box["x"] + box["width"] / 2, box["y"] + box["height"] / 2)
        pg.wait_for_timeout(1000)
        fps = pg.evaluate("""() => new Promise(r => { let n = 0; const t0 = performance.now();
            const f = () => { n++; if (performance.now() - t0 < 4000) requestAnimationFrame(f); else r(n * 1000 / (performance.now() - t0)); };
            requestAnimationFrame(f); })""")
        print(f"{t} s in, camera on the middle: {fps:.0f} frames a second")
        pg.screenshot(path=str(ROOT / "runs" / f"fps_{t}.png"))
    b.close()
