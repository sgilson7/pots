from icons import M, hero_fig, item_fig, ability_fig
from artlib import Fig
def mk(draw, rim="frost", shape="round"):
    def f(rng):
        fig = Fig("icon"); M.badge(fig, rim, shape); draw(fig); return fig
    return f
T = {
 "star": lambda f: (M.stun_stars(f,1,1.3,1.0), M.burst(f,1,0.8,0.4)),
 "flame": lambda f: M.flame(f,1,0.4,1.1),
 "bolt": lambda f: M.bolt(f,1,1,1.2),
 "snow": lambda f: M.snowflake(f,1,1,0.55),
 "crystal": lambda f: M.crystal(f,1,1,1.1,rot=-20),
 "drop": lambda f: M.drop(f,1,1,1.0),
 "cloud": lambda f: M.cloud(f,1,0.8,1.1),
 "swirl": lambda f: M.swirl(f,1,1,0.5),
 "heart": lambda f: M.heart(f,1,1,1.1),
 "plus": lambda f: M.plus(f,1,1,1.2),
 "eye": lambda f: M.eye(f,1,1,1.2),
 "gem": lambda f: M.gem(f,1,1,1.1),
 "orb": lambda f: M.orb(f,1,1,0.45),
 "ring": lambda f: M.ring(f,1,0.9,1.2),
 "potion": lambda f: M.potion(f,1,1,1.3),
 "sword": lambda f: M.sword(f,1,1,1.0,rot=-45),
 "dagger": lambda f: M.dagger(f,1,1,1.2,rot=-30),
 "axe": lambda f: M.axe(f,1,1,1.2),
 "hammer": lambda f: M.hammer(f,1,1,1.1,rot=30),
 "mace": lambda f: M.mace(f,1,1,1.1),
 "arrow": lambda f: M.arrow(f,0.45,0.45,1.6,1.6),
 "bow": lambda f: M.bow(f,1,1,1.1),
 "shield": lambda f: M.shield(f,1,1,1.2),
 "rshield": lambda f: M.round_shield(f,1,1,0.55),
 "helm": lambda f: M.helm(f,1,0.95,1.2),
 "boot": lambda f: (M.boot(f,1,1,1.1), M.wing(f,0.8,1.3,0.7)),
 "staff": lambda f: M.staff(f,1,1,1.2,rot=-30),
 "scroll": lambda f: M.scroll(f,1,1,1.2),
 "chainhook": lambda f: (M.chain(f,[(0.5,1.6),(1.0,1.2)]), M.hook(f,1.1,0.9,0.9)),
 "skull": lambda f: M.skull(f,1,1,1.2),
 "rock": lambda f: M.rock(f,1,1,1.1),
 "leaf": lambda f: M.leaf(f,1,1,1.2,rot=-20),
 "band": lambda f: M.band(f,1,1,1.2),
 "trail": lambda f: (M.trail(f,[(0.4,0.6),(0.7,0.9),(1.1,1.0),(1.4,1.3)],"violet"),),
}
rims = ["frost","fire","earth","steelrim","blood","arcane","storm","rot","consumables","basics","attributes","armaments","arcanetab","secret","common","support","protect","weapons","caster","relictab"]
PICTURES = {f"zztest/{i:02d}{k}": mk(v, rims[i % len(rims)], "round" if i%2 else "square") for i,(k,v) in enumerate(T.items())}
