# Brief

Stage 1. The person's own words come first; the rest was written by the agent from them, for the person to read. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

Given on 15 October 2026. The friend is not named here because the repository is public.

> the game description is as follows. my friend wants a game that plays as similarly as possible to the original mod for warcraft3, defense of the ancients, that went on to inspire league of legends and dota. it should support 5v5 online multiplayer in the browser, and should try and play as faithfully as possible in terms of how the game is played. I DO NOT want to copy characters such that we can get in legal trouble ,so they should be distinct from the warcraft champs but with the same abilities. the game should also have a basic arcade missio mode to teach you the basics of the game through increasingly more difficult games you play with 0-4 ai teammates vs 1-5 ai enemies based on complexity, and the difficulty of the AI controlled independently, all done via behavior trees like vagrancy. the online mulpliayer in the browser should be optimized for reducing lag, so it should use rollback netcode, and allowing people to reconnect mid-game if they bug out. build out the mvp for the game with 10 characters. start off by starting a design journal like the long-time coming and galactic survivors where each big change is made into its own tab. create the 3d models for everything in the game, the map for the game, the ui design for the game, and put it in the design journal for me to view, but dont stop working. then build out the working copy of arcade mode, then a working copy of the online mode, with a test specifically to see if a 5v5 game can be played by 5 runners using the online implmenetion that is recorded and analyzed in real time by a subagent. the online menus should look kind of like galactic survivor or ape race from ptonkle.github.io. any art you make for the game for the shop like item art, or ability art, make using the tikz prompt with no text in the diagrams, that should all be hoverable to show what they do. so create the journal, then do a literature review of how to recreate original warcraft 3 dota, looking at all sources like forum posts and reddit and fextra life type pages. then create the mvp accoridng to what i descrived. i give yo uapproval to create a public github repo, and I give you approval to deploy to github pages, and to deploy to the website. name the game protection of the spheres (pots for the browser link). ok good luck dont stop till its done

## The tool

**Protection of the Spheres** (`pots`): a 3D browser game that plays like the Warcraft III map Defense of the Ancients (DotA Allstars, the 6.x era). Two teams of five heroes push three lanes of creeps toward the other side's Sphere, the building whose fall ends the game. Ten heroes, each with original names and looks, carry the abilities of ten DotA heroes. Two modes:

- **Arcade**: a ladder of missions that teach the game, each with 0 to 4 computer teammates against 1 to 5 computer enemies. The size of the match and the skill of the computer players are set separately. Computer players run on behavior trees, as in Vagrancy.
- **Online**: 5v5 between people in browsers, with rollback netcode and rejoining a match in progress.

Played with mouse and keyboard on a desktop or laptop browser.

## Done means

- A journal artifact with one tab per major change, holding the literature review, the 3D models of every hero, creep, building and neutral, the map, and the UI design.
- A literature review in `notes/LITERATURE.md` with sources for each rule the game copies.
- Ten heroes, each with four abilities that work like the DotA hero they stand in for, original names and looks, and hover text for every ability.
- A shop with item art drawn in TikZ, no text in any icon, and hover text for every item.
- Arcade mode: a mission ladder that can be played start to finish against computer players, with team sizes and computer skill chosen separately.
- Online mode: a 5v5 room joined by code, rollback netcode, and rejoining a match after a dropped connection.
- A test in which five headless runners join one online match and play it, recorded, with the recording read by a separate agent as it runs.
- Online menus in the manner of Galactic Survivors and Ape Race.
- `./scripts/check.sh` passes; the game is live at `https://sgilson7.github.io/pots/` and has a tile on sgilson7.github.io.

## Decisions that stay with the person

- The hero names, looks and the names of items (a generated list a reader will see).
- Whether a hero is close enough to its DotA source and still distinct from it (the legal question). The agent changes names, looks and flavour, keeps numbers and behaviour, and flags anything it is unsure of.
- Balance beyond copying DotA's numbers.
- Whether the public signaling service used to connect browsers is acceptable.

## Accessibility

Teams are told apart by shape and outline as well as colour (the two sides' units carry different base rings and banner shapes). Health bars carry numbers on hover. Keyboard shortcuts for every ability and item, with mouse clicks as an alternative. Reduced motion turns off screen shake and flashes.

## Style

A described look: low-poly 3D, readable from a raised camera like Warcraft III, with bright team colours. Menus for online play after Galactic Survivors and Ape Race. Item and ability icons from the TikZ pipeline in `tools/art/`, with no text in any icon.

## Words

Terms from DotA that the game uses in the same sense go into `GLOSSARY.md`: lane, creep, last hit, deny, tower, barracks, rune, neutral camp, gank, buyback, bounty, respawn, mana, cooldown, stun, slow, ultimate, attribute (strength, agility, intelligence), armor, magic resistance. "Ancient" is replaced by "Sphere" (the game's name). "Roshan" becomes the game's own boss name.

## Autonomy

The plan is approved in advance ("dont stop till its done"). Where this brief is silent, the agent decides, records the decision in `SECOND-ORDER.md`, and continues. The agent may not loosen a Builder requirement or a boundary test, and may not add an account, analytics, or any off-origin request except the declared online feature (R7), which loads only when a player opens online play. The person's grant, word for word: "i give yo uapproval to create a public github repo, and I give you approval to deploy to github pages, and to deploy to the website. name the game protection of the spheres (pots for the browser link)." So the agent may create the public repository `sgilson7/pots`, publish it with `./scripts/publish.sh` once `./scripts/check.sh` passes, and add the game to sgilson7.github.io and deploy that site. The agent still runs the browser check against each live address.

## Allowed core dependencies

`serde`, `serde_json` (reference default).
