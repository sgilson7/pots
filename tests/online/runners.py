#!/usr/bin/env python3
"""The five-runner test: five browsers play one online match.

Each runner is its own Chromium process (not a context: separate processes
are separate players, after Floodline's table.py), recorded on video. Runner
1 hosts a room; runners 2 to 5 join by the room link, which is what a player
would paste. Each picks a different hero; the host starts; the five human
seats fill three Sunward and two Moonward places, and computer players fill
the other five, so the match is five against five.

During the match each runner hands its hero to the computer (the pause
menu's "Let the computer play my hero") so the match goes on as a real one,
and also clicks and presses keys every few seconds, so this browser's own
orders travel the network beside the computer's. Midway, one runner's
browser is closed without warning and reopened later on the same address:
the game must hand the seat to the computer and give it back.

Every two seconds each runner's state (the page's data-probe: tick,
checksums, rollbacks, stalls, status) is appended to a JSON-lines log, which
a separate agent reads while the match runs. At the end the checksums the
runners computed for the same ticks are compared.

    .venv/bin/python tests/online/runners.py OUT_DIR [minutes]
"""
import functools, http.server, json, pathlib, random, sys, threading, time
from playwright.sync_api import sync_playwright

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "runs/latest")
MINUTES = float(sys.argv[2]) if len(sys.argv) > 2 else 10
RUNNERS = 5
DROP_RUNNER = 3  # zero-based: the fourth runner drops and returns
OUT.mkdir(parents=True, exist_ok=True)
(OUT / "video").mkdir(exist_ok=True)
(OUT / "shots").mkdir(exist_ok=True)
LOG = open(OUT / "runners.jsonl", "a", buffering=1)


def log(**kw):
    kw["t"] = round(time.time(), 2)
    LOG.write(json.dumps(kw) + "\n")


class Quiet(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def log_message(self, *a):
        pass


srv = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Quiet, directory=str(ROOT / "dist")))
threading.Thread(target=srv.serve_forever, daemon=True).start()
BASE = f"http://127.0.0.1:{srv.server_address[1]}/index.html"
ARGS = [
    "--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist",
    "--disable-features=WebRtcHideLocalIpsWithMdns",
    "--disable-background-timer-throttling", "--disable-backgrounding-occluded-windows", "--disable-renderer-backgrounding",
]
rng = random.Random(7)


def probe(pg):
    try:
        return json.loads(pg.get_attribute("body", "data-probe") or "{}")
    except Exception as e:  # a closed page
        return {"error": str(e)[:80]}


class Runner:
    def __init__(self, p, k):
        self.k = k
        self.p = p
        self.errors = []
        self.open()

    def open(self, url=None):
        self.browser = self.p.chromium.launch(args=ARGS)
        self.ctx = self.browser.new_context(viewport={"width": 1280, "height": 720},
                                            record_video_dir=str(OUT / "video" / f"runner{self.k + 1}"),
                                            record_video_size={"width": 640, "height": 360})
        self.page = self.ctx.new_page()
        self.page.on("console", lambda m: m.type == "error" and self.errors.append(m.text[:200]))
        self.page.on("pageerror", lambda e: self.errors.append(f"pageerror: {str(e)[:200]}"))
        self.page.goto(url or BASE)
        self.page.wait_for_selector("body[data-ready='true']", state="attached", timeout=60000)

    def close(self):
        try:
            self.ctx.close()
            self.browser.close()
        except Exception:
            pass

    def autopilot(self):
        pg = self.page
        pg.keyboard.press("F10")
        pg.wait_for_selector("[data-a='autopilot']", timeout=10000)
        pg.click("[data-a='autopilot']")

    def poke(self):
        """A human-like order: a right-click on the map, an ability, a level-up."""
        pg = self.page
        try:
            r = rng.random()
            if r < 0.4:
                pg.mouse.click(rng.randint(200, 1080), rng.randint(80, 520), button="right")
            elif r < 0.7:
                pg.keyboard.press(rng.choice(["KeyQ", "KeyW", "KeyE", "KeyR"]))
                pg.mouse.click(rng.randint(300, 980), rng.randint(120, 480))
            elif r < 0.85:
                pg.keyboard.press("KeyO")
                pg.keyboard.press(rng.choice(["KeyQ", "KeyW", "KeyE", "KeyR", "KeyT"]))
            else:
                pg.keyboard.press("Space")
        except Exception as e:
            self.errors.append(f"poke: {str(e)[:120]}")


def main():
    t_start = time.time()
    with sync_playwright() as p:
        runners = [Runner(p, k) for k in range(RUNNERS)]
        host = runners[0]
        host.page.click("[data-a='screen'][data-v='online']")
        host.page.fill("input[data-a='online.name']", "Runner 1")
        host.page.click("[data-a='online.host']")
        host.page.wait_for_selector(".room-code .code")
        code = host.page.inner_text(".room-code .code").strip()
        log(event="room", code=code)
        for r in runners[1:]:
            # a fresh page on the room link, as a player opening a shared link
            r.close()
            r.open(f"{BASE}#room={code}")
            r.page.wait_for_selector(".hero-mini", timeout=60000)
        # wait for everyone in the room
        deadline = time.time() + 120
        while time.time() < deadline and probe(host.page).get("net", {}).get("peers", 0) < RUNNERS - 1:
            time.sleep(1)
        peers = probe(host.page).get("net", {}).get("peers", 0)
        log(event="joined", peers=peers, seconds=round(time.time() - t_start, 1))
        for k, r in enumerate(runners):
            r.page.click(f".hero-mini[data-v='{k * 2}']")
            time.sleep(0.5)
        time.sleep(3)
        host.page.click("[data-a='online.start']")
        for r in runners:
            r.page.wait_for_selector("body[data-screen='match']", timeout=60000)
        log(event="started", seconds=round(time.time() - t_start, 1))
        for r in runners:
            r.autopilot()
        end = time.time() + MINUTES * 60
        drop_at = time.time() + MINUTES * 60 * 0.4
        back_at = drop_at + 30
        dropped_url = None
        last_shot = 0
        while time.time() < end:
            now = time.time()
            for r in runners:
                if r.k == DROP_RUNNER and dropped_url and r.page is None:
                    continue
                if r.page is not None and rng.random() < 0.25:
                    r.poke()
            if dropped_url is None and now >= drop_at:
                victim = runners[DROP_RUNNER]
                dropped_url = victim.page.url
                log(event="drop", runner=DROP_RUNNER + 1, url=dropped_url)
                victim.close()
                victim.page = None
            if dropped_url and runners[DROP_RUNNER].page is None and now >= back_at:
                victim = runners[DROP_RUNNER]
                victim.open(dropped_url)
                log(event="rejoin", runner=DROP_RUNNER + 1)
                # once caught up, the runner hands its hero to the computer again
                victim.rejoined_at = now
            v = runners[DROP_RUNNER]
            if getattr(v, "rejoined_at", None) and now - v.rejoined_at > 20 and not getattr(v, "piloted", False):
                pr = probe(v.page)
                if pr.get("screen") == "match" and not pr.get("net", {}).get("catching", True):
                    try:
                        v.autopilot()
                        v.piloted = True
                        log(event="autopilot_again", runner=DROP_RUNNER + 1)
                    except Exception as e:
                        log(event="autopilot_failed", runner=DROP_RUNNER + 1, error=str(e)[:120])
            for r in runners:
                if r.page is None:
                    log(runner=r.k + 1, probe={"gone": True})
                    continue
                log(runner=r.k + 1, probe=probe(r.page), errors=len(r.errors))
            if now - last_shot > 60:
                last_shot = now
                for r in runners:
                    if r.page is not None:
                        try:
                            r.page.screenshot(path=str(OUT / "shots" / f"r{r.k + 1}-{int(now - t_start):04d}.png"))
                        except Exception:
                            pass
            time.sleep(2)
        # the verdict: the checksums every runner computed for the same ticks
        finals = [probe(r.page) if r.page else {} for r in runners]
        # compare the browsers that are in the match; one that is not is reported apart
        logs = [dict(map(tuple, f.get("net", {}).get("sumlog", []))) for f in finals]
        playing = [l for l in logs if l]
        common = set(playing[0]) if playing else set()
        for l in playing[1:]:
            common &= set(l)
        agree = bool(common) and all(len({l[t] for l in playing}) == 1 for t in common)
        summary = {
            "event": "end",
            "minutes": MINUTES,
            "runners": RUNNERS,
            "ticks": [f.get("tick") for f in finals],
            "common_checksum_ticks": sorted(common),
            "checksums_agree": agree,
            "in_match": [bool(l) for l in logs],
            "desync": [f.get("net", {}).get("desync") for f in finals],
            "rollbacks": [f.get("net", {}).get("rollbacks") for f in finals],
            "deepest": [f.get("net", {}).get("deepest") for f in finals],
            "stalls": [f.get("net", {}).get("stalls") for f in finals],
            "errors": {r.k + 1: r.errors[-10:] for r in runners},
        }
        log(**summary)
        (OUT / "summary.json").write_text(json.dumps(summary, indent=1))
        print(json.dumps(summary, indent=1))
        for r in runners:
            r.close()


if __name__ == "__main__":
    main()
