"""Two browsers meet through the real relays and play for a minute."""
import functools, http.server, json, pathlib, sys, threading, time
from playwright.sync_api import sync_playwright
ROOT = pathlib.Path(__file__).resolve().parents[2]
class Q(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}
    def log_message(self, *a): pass
srv = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Q, directory=str(ROOT / "dist")))
threading.Thread(target=srv.serve_forever, daemon=True).start()
base = f"http://127.0.0.1:{srv.server_address[1]}/index.html"
ARGS = ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist", "--disable-features=WebRtcHideLocalIpsWithMdns",
        "--disable-background-timer-throttling", "--disable-backgrounding-occluded-windows", "--disable-renderer-backgrounding"]
def probe(pg):
    return json.loads(pg.get_attribute("body", "data-probe") or "{}")
with sync_playwright() as p:
    logs = {0: [], 1: []}
    bs = [p.chromium.launch(args=ARGS) for _ in range(2)]
    pgs = [b.new_page(viewport={"width": 1280, "height": 720}) for b in bs]
    for k, pg in enumerate(pgs):
        pg.on("console", lambda m, k=k: logs[k].append(f"{m.type}: {m.text}"))
        pg.on("pageerror", lambda e, k=k: logs[k].append(f"pageerror: {e}"))
    host = pgs[0]
    host.goto(base); host.wait_for_selector("body[data-ready='true']", state="attached")
    host.click("[data-a='screen'][data-v='online']"); host.click("[data-a='online.host']")
    host.wait_for_selector(".room-code .code")
    code = host.inner_text(".room-code .code").strip()
    print("room", code)
    pgs[1].goto(f"{base}#room={code}"); pgs[1].wait_for_selector("body[data-ready='true']", state="attached")
    t0 = time.time()
    while time.time() - t0 < 60 and probe(host)["net"]["peers"] < 1:
        time.sleep(1)
    print("peers after", round(time.time() - t0), "s:", probe(host)["net"]["peers"], probe(pgs[1])["net"])
    host.click(".hero-mini[data-v='0']"); pgs[1].click(".hero-mini[data-v='1']")
    time.sleep(2)
    host.click("[data-a='online.start']")
    time.sleep(40)
    for k, pg in enumerate(pgs):
        print(k, json.dumps(probe(pg))[:600])
        pg.screenshot(path=f"/tmp/smoke{k}.png")
    for k in logs:
        for l in logs[k][-8:]:
            print(k, l[:200])
    for b in bs: b.close()
