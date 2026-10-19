# Decisions

Dated entries. Each quotes the person's request word for word, then says what the agent decided and where it is recorded. Grants of autonomy go here too ("you may publish"), with their scope: this run only, or until withdrawn.

## 2026-10-15

> i give yo uapproval to create a public github repo, and I give you approval to deploy to github pages, and to deploy to the website. name the game protection of the spheres (pots for the browser link). ok good luck dont stop till its done

The agent decided:

- The game is named Protection of the Spheres. The public repository is `sgilson7/pots`, served at `https://sgilson7.github.io/pots/`.
- The plan is approved in advance ("dont stop till its done"); stage 7 (the person plays a match) is the gate for later rounds, not for this first publish.
- The agent may publish with `./scripts/publish.sh` once `./scripts/check.sh` passes, and may add the game to the scholar site (sgilson7.github.io) and deploy it. Scope: this build and its later triage rounds, until withdrawn.
- The working folder stays `OGDota`, the folder the person made; the repository and every reader-visible name use `pots`.

> I DO NOT want to copy characters such that we can get in legal trouble ,so they should be distinct from the warcraft champs but with the same abilities.

The agent decided: every hero, item, creep, building and boss gets an original name, look and flavour. Ability behaviour and numbers follow DotA. No Blizzard or IceFrog name, model, icon, sound or text is used. The names are a generated list the person reads (a worklist row in `SECOND-ORDER.md`).

## 2026-10-18

> then, deploy all of that to main, i approve any make deploys.

The agent decided: the round-7 changes (T20 to T22) go to `main` and are published with `./scripts/publish.sh` once `./scripts/check.sh` passes; "make deploys" is read as the scholar site's `make deploy` too, if its page needs a change. Scope: this round.
