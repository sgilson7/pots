"""Hero portraits: web/art/heroes/<hero id>.png, a head-and-shoulders bust on
a round badge in the hero's element (tools/art/icons.py)."""
from icons import M, hero_fig


def ironvow(rng):
    fig = hero_fig("ironvow")
    with M.clipped(fig):
        fig.ellipse(1.0, 0.25, 0.7, 0.45, "steel")
    M.helm(fig, 1.0, 1.1, 0.95)
    return fig


PICTURES = {"heroes/ironvow": ironvow}
