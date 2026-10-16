## The "feel" of DotA 1 (WC3 DotA Allstars, about 6.48–6.83): mechanics and what players said

**Research note (read first).** Reddit was not reachable: both the search tool and direct fetches were blocked, so this file has no Reddit quotes. The Wayback Machine (archived playdota/getdota posts) timed out, and fandom.com / warcraft.wiki.gg sit behind Cloudflare. The quotes below come from Liquipedia wikitext (pulled through its API), TeamLiquid/LiquidDota threads from the Dota 2 beta in Oct–Dec 2011, Hive Workshop threads, Steam Community, DotA-Blog (2008) and D1Stats. "Verbatim" means I copied the quote from page text I downloaded myself. Quotes marked *(search snippet)* came from a search-engine summary of a page I could not open, so their wording is UNVERIFIED.

Liquipedia's Dota 2 wiki keeps DotA 1 version history: entries numbered 6.xx up to about 6.74 are WC3-only, and later ones were shared by both games. I use those entries as the DotA 1 rule wherever they exist.

---

### 1. Last-hitting

**Rule.** Only the killing blow pays gold. Experience goes to every enemy hero within range. Attack damage is a random range, not a fixed number. That range was inherited from WC3 and kept in Dota 2.

- "Enemies gain no gold and reduced experience from any denied unit." (Liquipedia, Denying) https://liquipedia.net/dota2/Denying
- Damage ranges as a deliberate last-hitting knob: "Chaos Knight, for instance, has a huge range of damage which makes it hard to last hit perfectly; In the same sense, certain heroes like Drow have a very small range". Barbiero, 18 Oct 2011, #59. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=3
- WC3 last-hitting felt harder: "in dota on wc3 it was much much harder to last hit creeps at least for me". Dr. E, 22 Oct 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- "the game feels a lot smoother so i do think its easier to last hit dota2 animations feel and look more natural than wc3 even though they're pretty much the same speeds". rabidch, LiquidDota staff, 16 Oct 2011, #6. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
- A small UI difference: "lasthitting and aiming units is very weird because the pointer doesn't change when you hit 'A"". Barbiero, #2, same thread.
- `-cs` showed your last hits and denies: "Shows how many creeps you have killed, as well as how many you have denied. Added in v6.28". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

### 2. Denying (version-dependent rules)

**Rules.**
- A creep can be denied once it is below 50% HP. Players attack their own unit with A-click.
- Early DotA punished attacking allies: "attempting to do so caused the attacking unit to get hexed for one second". The hex was later replaced with a stop command. After that, denying became allowed below 50% HP. https://liquipedia.net/dota2/Denying (Trivia)
- **Deny XP by version.** These are verbatim Liquipedia version-history lines. https://liquipedia.net/dota2/Denying and https://liquipedia.net/dota2/Archive:Experience

| Version | Rule |
|---|---|
| Before 6.36 | A denied creep gave the enemy **no** XP |
| 6.36 | "Denied units now give off minor experience instead of none" |
| 6.36b | "Lowered the recent XP deny change a little." |
| 6.38 | "Melee heroes get denied less XP than range heroes do." |
| 6.44 | Heroes under deny-able debuffs can only be denied below 25% HP: "You have to be below 25% health for an ally to be able to start denying you" |
| 6.78 | "Deny XP and Bonus XP/gold AoE is now the same as regular XP AoE (1000->1200)." |
| 6.79 | "Ranged heroes now get the same denied experience as melee heroes (instead of less)." |
| 6.82 | "Denied creeps now give less experience (50% experience, instead of a constant 36 XP per unit)." |

- **What this means for 6.48–6.81.** A denied creep gave each nearby enemy a small *flat* amount of XP, not a percentage. The Archive:Experience page quotes a 2011 wiki revision: 36 XP for melee heroes and 18 for ranged. The 6.82 line confirms "constant 36". The split between melee and ranged heroes lasted from 6.38 until 6.79. The exact ranged figure in WC3 is UNVERIFIED; 18 comes from a Dota 2-beta-era wiki revision.
- **A deny marker.** "Every time you, an ally or an opponent makes a deny you see an exclamation mark in the color of the player that denied the creep. Added in v6.39". `-showdeny` / `-hidedeny` toggled it. http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html
- **Towers.** "Towers may be denied once they drop to 10% health and below". This is current Dota 2. The DotA 1 tower deny threshold is UNVERIFIED but believed to be the same.

**How players felt about the later Dota 2 deny changes.** After 7.06, 7.07, 7.20 and 7.26b, the denier gained some XP and then some gold, and the share of XP the denied side still got rose from 25% up to 50%.
- "Now if all your creeps are denied and you stay solo in lane touching nothing you get as much exp as if you were in double lane and lasthitted every single creep." Steam, 22 May 2020. https://steamcommunity.com/app/570/discussions/0/2245553353021133678/
- The same poster on the motive: "they decided to reduce gap between expirienced players and noobs by reducing advantage winning team gets in lane." (same URL)

### 3. Orb walking

**Rule.** WC3 called unique attack modifiers "orb effects". Players manually cast an autocast attack such as Frost Arrows instead of right-clicking. This counts as a spell cast, so it does not draw creep or tower aggro, and the hero can be ordered to move during the cast without losing the attack.
- "In Warcraft III, unique attack modifiers are called orb effects … The term Orb Walking originates therefore from Warcraft III". https://liquipedia.net/dota2/Attack_Modifier
- "casting these attack modifiers manually, also known as Orb Walking, neither draws aggro from lane creeps nor towers." (same page)
- "orb walking and stutter stepping were very commonly used together, so commonly that it is just referred to as orb walking". (same page)
- From a DotA-forum-era guide (BLAOW, mirrored on Eurobattle), *(search snippet)*: "manually casting autocast abilities like Drow's frost arrows avoids aggro". https://forum.eurobattle.net/threads/69659-Line-and-map-control-guide-(written-by-BLAOW-from-dota-allstars-forum) (wording UNVERIFIED)
- "clinkz has great animation if you know how to animation cancel and orbwalk." brokor, 17 Oct 2011, #38. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2

### 4. Animation cancel, attack point and backswing (WC3 engine)

**Rule.** An attack has two phases:
- **Damage point (foreswing).** Cancelling here cancels the attack. For a ranged unit this is the moment the projectile launches.
- **Backswing.** Any new order can cancel it, and cancelling costs nothing. It does not let you attack faster.

Cast point and cast backswing work the same way for spells.
- "Backswing is an interruptible period, so a new attack will cut it short … if you re-issue an order they'll interrupt the backswing and move. This is where animation cancelling comes from in things like DotA." Hive Workshop, 19 Jan 2014. https://www.hiveworkshop.com/threads/dps-and-attack-animation-damage-point.247190/
- "For ranged it is the time between starting the animation and launching the projectile." (same thread)
- "Backswing point is the length of animation after an ability took effect. It can be canceled however via ordering another command." https://www.hiveworkshop.com/threads/cast-point-and-backswing-point.265781/
- "canceling your animation does not allow you to attack faster." https://liquipedia.net/dota2/Attack_Animation
- **Some WC3 heroes had bad animations on purpose, and players learned them as a skill:**
  - "zeus had to finish his attack animation before his projectile comes out … so you couldnt animation cancel like every other hero". Rhaegar99, #16. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
  - "I spent a lot of time learning specific heroes animation and attack points and it gave me a huge edge with certain weird hereos(razor, krob etc.)" Rainmaker5, #42, page 3 of the same thread.
  - "I think CM and other heroes should KEEP their shitty animation. Because the game was balanced around it". FacelessVoid, #31, page 2.
  - "attack points and animation times(front/backswings) are all the same, the animation just feels better." Barbiero, #48, page 3. This says Dota 2 kept the WC3 timings and only redrew the visuals.
  - "a huge portion of the balance is based around some heroes having bad animations. Lina, for example, would be absurd if her spells and autoattack didn't have a long animation." InvalidID, 16 Dec 2011. https://tl.net/blogs/295350-dota-2-interface-concerns
- Liquipedia 6.00: "Updated all of the attack animations so they more fully match the hero models." https://liquipedia.net/dota2/Attack_Animation

### 5. Creep aggro

**Dota 2 rule, as documented.** DotA 1 likely matches it, but that is UNVERIFIED.
- Lane creeps within 500 range of a hero who issues an attack order on an enemy hero switch to that hero. They stay aggroed for up to about 2.3 s, and the trigger has a roughly 3 s cooldown.
- Spells do not trigger it.
- Sources:
  - "Issuing an attack order on an enemy hero will draw the aggro of lane creeps within [500] range of the aggressor hero. An attack order alone is enough". https://liquipedia.net/dota2/Lane_Creeps
  - "The lane creeps' protective behavior only reacts on attacks, they completely ignore ability cast." (same page)

**DotA 1 evidence.**
- A DotA-forum-era guide *(search snippet)*: "directly attacking, i.e. right-clicking, an enemy hero near creeps will cause them to attack you, whereas manually casting spells will not". BLAOW guide, Eurobattle mirror (URL above). Wording UNVERIFIED.
- Community DotA 1 patch 6.88 (post-IceFrog): "Non-heroes will not draw creep aggro when attacking enemy hero anymore" and "Improved lane creeps aggro". https://d1stats.ru/88n-betas-changelog/?lang=en
  - This implies that up to 6.83, any unit (summons too) hitting an enemy hero drew creep aggro, as in the WC3 "call for help" behaviour. That reading is INFERENCE.
- Exact DotA 1 radius and duration: UNVERIFIED. The 500 / 2.3 s / 3 s figures are Dota 2 values.
- Players said creep behaviour felt a little different after the port:
  - "Creep AI is basically the same as in WC3". Barbiero, #2. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
  - "i feel like the creep ai has changed a good amount tho, they seem a lot less moody". Dead9, #52, page 3.

### 6. Tower aggro

**Rule (Dota 2, 2014; inherited from DotA, DotA 1 specifics UNVERIFIED).**
- Tower priority: "Closest enemy hero attacking a friendly hero with auto attack" comes first. Then an enemy creep attacking a friendly hero, then a unit attacking the tower, then a unit attacking any friendly unit, then the closest enemy, then siege.
- The tower keeps its target until a "Reseter": the target dies or becomes unattackable, or the tower's current target hero manually attacks a unit.
- "Spells ARE NOT AUTO-ATACK. This means you can spam spells under a tower and it won't switch."
- "The key to this technique is the difference between Tower Aggro Range (500) and Tower Attack Range (700)."
- Source: Steam guide, Jul/Aug 2014. https://steamcommunity.com/sharedfiles/filedetails/?id=288488800
- The guide calls tower and creep aggro "essentially the same" and "the most important step between low level of gameplay and intermediate level". (same URL)

### 7. Juking, trees and tree pathing

**Rules.**
- Trees block ground vision and pathing.
- DotA 1 trees regrew on a **single global 5-minute timer**, not one timer per tree. This is stated in the community 6.85 notes as the behaviour it replaced: "Trees now individually regrow 5 minutes after they were destroyed, instead of trees globally regrowing every 5 minutes." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/ (27 Apr 2017)
- In WC3, trees also changed projectile and ability flight height. Same changelog: "Trees no longer affect the flying height of projectiles and/or abilities (Elune's Arrow, Meat Hook, etc.)"

**What players said.**
- "DOTA 1 more makes us to jukes...play with fog and trees.. self timing etc.." ARTSZ*, 2 Aug 2021. https://steamcommunity.com/app/570/discussions/0/3053989078744434569/
- "DOTA 1 you can easily escape from enemy only by jukes on the tree for example." (same thread)
- A counterpoint in the same thread: "jukes are the same, people got better at reading them".
- Night gives "an easier of escaping by juking and utilizing the trees to obstruct your vision". https://tl.net/blogs/282827-dota-back-to-basics (Nov 2011; describes the Dota 2 beta port of 6.7x)

### 8. Fog of war and vision

**Rules.**
- Lower ground cannot see higher ground.
- Trees cast "shadows" in vision.
- Day and night change vision range.
- Units leaving vision cancel spells that are mid-cast, but not attacks.
- Sources:
  - "Units with ground vision cannot see areas that are on higher elevation than them." https://liquipedia.net/dota2/Vision
  - "If an enemy enters the fog of war during the cast time of an ability, the cast gets canceled. However, attacks do not cancel". (same page)
- Ranged attacks uphill had a miss chance: "for ranged heroes avoids the 25% miss chance of last hitting uphill." https://liquipedia.net/dota2/Creep_control_techniques
  - Whether the 25% figure is WC3's own number is UNVERIFIED. The WC3 gameplay constant was not confirmed.

**What players said.**
- "Fog bugs, much more." This appears in a list of DotA WC3 cons. Hive, 23 Sep 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- "Valve just farmed the original idea … some changes to hero abilities and fog mechanics." (same thread)

### 9. Unit collision and creep blocking

**Rule.** Hero bodies physically block creeps. Creep-blocking the first wave shifts where the lanes meet.
- "physically blocking your creeps' path with your hero so they take longer to reach the middle of the lane." https://liquipedia.net/dota2/Creep_control_techniques
- A WC3 habit: "In DotA a goal when blocking was often to let the ranged creep through so it would arrive first and die first". Yurie, 18 Oct 2011, #64. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=4
- "the range creep is supposed to be slightly harder to block". rabidch, #66 (same page)
- A WC3 pathing bug: Sentinel bottom-lane creeps sometimes wandered into the forest. Q: "do creep ever get bugged out on bot lane anymore and run though the sent lower forest?" A: "Nope". #54 and #60, page 3.
- "the bottom creep lane seems way less buggy in its pull behavior … it was a frustrating imbalance for the sentinel". InvalidID, #88, page 5.
- Some unit models were hard to click: "units like bounty hunter don't have that naturally "unclickable" unit model anymore". This is a translated iG report, #46, page 3. A WC3 quirk.

### 10. Item sharing, pooling and dropping

**Rules.**
- In WC3 DotA almost every item could be dropped on the ground and picked up by anyone.
- **6.52** brought in owner-locked ("muted") items for anyone who was not the owner, plus an optional pooling mode: "-poolingmode / -pm … This mode allows pooling (item sharing between players) … Added in v6.52". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html
- A Liquipedia archive page records 6.52: "Items now have muted form when carried by non-owner, and a pooling mode was added as an option" *(search snippet)*. https://liquipedia.net/dota2/Archive:Item_Sharing (wording UNVERIFIED)

**Engine quirks around items** (from the community 6.85 notes, which describe what they fixed). https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
- Items on cooldown could not be moved or dropped: "Using items no longer causes them to be stuck on the hero while they're on cooldown."
- The courier needed items dropped on the ground to combine recipes: "You no longer need to drop items on the ground to get proper item combination."
- Toggling items interrupted orders: "Toggling Radiance, Power Tread … no longer interrupts the hero."
- Inventory rearranging in DotA 1 worked as "right click the item once to select it and right click the new location again to move it". NowLookHere, #23. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2
- Power Treads key presses got "lost": "If you had pressed the key for your power treads a few times in a row, the engine would have lost it one time." Perscienter, #18, page 1.
  - Reply: "It wasn't lag. Items all have CDs." cyberspace, #19.
- Leavers: "All items from the leaver's hero are sold at half price, with the gold distributed evenly to all his teammates." (team-sell) https://warcraft.wiki.gg/wiki/Defense_of_the_Ancients:_Allstars (via fetch summary)
- `-itemswap x y` existed "for dealing with undroppable items or items in cooldown" (v6.55). DotA-Blog URL above.

### 11. Runes and the Bottle

**Rules (late-6.7x, as ported to the Dota 2 beta in 2011).**
- One rune spawns every 2 minutes in one of two river spots.
- Runes in the pool: Double Damage, Invisibility, Haste, Regeneration, Illusion.
- "At the beginning of every game and every second minute (00:00, 02:00, 04:00, etc.), there will spawn one rune in one of the two rune locations." https://tl.net/blogs/282827-dota-back-to-basics
- "Haste runes allow a hero to run at maximum movespeed (522) for 30 seconds". (same URL)

**Bottle quirks in 6.83** (community 6.85 notes, describing what they changed): "Runes are now placed into the bottle automatically on pickup. Using a stored rune no longer interrupts the hero." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
- The direct reading: in DotA 1, bottling took a deliberate action and using a stored rune interrupted the hero. The exact bottling input in WC3 is UNVERIFIED.
- The single-player test command `-powerup` spawns a rune, but "rune won't spawn if there already is one, or was one less than 5 seconds ago". http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

### 12. Ganking and Smoke of Deceit

- **Smoke of Deceit is not in 6.48–6.69.** It was added in **6.70** (cost 100, stock 3) and changed in 6.71. https://liquipedia.net/dota2/Archive:Version_6.70 and https://liquipedia.net/dota2/Archive:Version_6.71 (*search snippet*; both pages were rate-limited, so wording is UNVERIFIED)
  - A faithful 6.48–6.69 remake should leave Smoke out. A 6.70+ remake needs it.
- Day and night shaped ganks: "Ganking in the different hours of day is very different because the enemy will be able to see you later in the gank". https://tl.net/blogs/282827-dota-back-to-basics

### 13. WC3 engine quirks: selection, hotkeys, orders, randomness

- **Selection.** Clicking another unit selects it. DotA 1 players worried about "accidentally deselecting my hero at horrible times. It makes the interface feel clunky." Ryhn, #27. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=2
  - "failing to cast a skill because its on cooldown never deselected your hero in wc3". rabidch, #32 (same page)
- **Stop and hold.** Players spammed Stop (S) or Hold (H) to keep from auto-attacking while waiting to last-hit: "I don't enjoy spamming the shit out of S until I want to last hit." Zlasher, #50, page 3.
  - "usually people just walk their heroes around instead of spamming S or hitting H". Dead9, #52.
- **Auto-acquire** stayed in Dota 2: "default is wc3 behavior, you can turn off auto attacking if you want it to". rabidch, #36, page 2.
- **Hotkeys.** Items sat on the numpad by default, and many players used third-party tools to remap them. "By default they are bound to the Numpad". #84, page 5.
- **Information hiding.** To see an enemy's items or mana you had to click the enemy: "showing opponents items/mana/buffs will never be done because it wasnt in dota1 … you have to click on your opponent to know what items they have". Post #16. https://tl.net/blogs/295350-dota-2-interface-concerns
  - Always-on mana bars needed "dll injection and therefor were considered hacks in most competitive settings". InvalidID, #89. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=5
- **Order latency.** "The input latency thing is required for balance, and was changed slightly in DOTA2." InvalidID. https://tl.net/blogs/295350-dota-2-interface-concerns
  - The community 6.85 notes claim: "Every Warcraft 3 map has "Random delay" option, which randomly delays every given attack order by 0-0.25 seconds. Originally DotA didn't change it". https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - As an engine fact this is UNVERIFIED. Battle.net's roughly 250 ms default turn latency is widely reported but not sourced here.
- **Pseudo-random distribution.** Applies to Critical Strike, Bash, Pulverize, Hardened Skin, Orb of Slow and Demolish, but not to the damage roll. Playdota article (archived; not fetched), cited in thread #77, page 4: http://www.playdota.com/forums/7993/article-pseudo-random-distribution/
  - The damage roll is "dicerolls. Some heroes have more dices then others (creating a distribution rather than a linear range)". 5-s, #82, page 5.
- **Turn rate.** WC3 units already had turn rates, and Dota 2 kept them; Liquipedia gives the Dota 2 facing rule (11.5°). https://liquipedia.net/dota2/Turn_Rate
  - A 6.85 changelog line, "Increased turn rate of Black Drake and Black Dragon from 0.1 to 1.0", shows that some WC3 units had very slow turn rates. https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - DotA 1 hero turn-rate values: UNVERIFIED. They were not fetched.
- **"Paused" units.** The 6.85 notes say "All sources of "paused" unit state were removed, as this method is outdated and buggy." (same URL)
- **Neutral AI under observers.** Post: "Only when there are spectators watching (DotA uses player 11 and 12 for neutral creep AI when they are available)". Barbiero, #22, page 2.

### 14. Chat commands (DotA-Blog guide, 2008–2010; versions as stated)

URL for this section: http://www.dota-blog.com/2008/01/dota-allstars-game-command-mode-guides.html

**Modes** are typed by the host as one string, for example "-dm -ar -id -sc". The main modes:
- `-ap` (All Pick)
- `-ar` (All Random)
- `-rd` (Random Draft)
- `-sd` (Single Draft)
- `-cm` (Captains Mode)
- `-lm` (League Mode)
- `-id` (Item Drop)
- `-sc` (Super Creeps)
- `-ns` (No Swap)
- `-nr` (No Repick; "Added in v6.48")
- `-pm` (Pooling, 6.52)
- `-oi` (Observer Info, 6.52)

**In-game commands:**
- `-ma`: "Displays which opponent controls which hero, and it's level. Also shows the time the player left the game".
- `-ms`: movement speed.
- `-cs` / `-cson`: creep stats.
- `-disablehelp`: blocks allied Chen, Furion and Harbinger teleports or sprouts.
- `-unstuck`: "Channels for 60 seconds, then moves your hero back to your base".
- `-roll #`: random number 0–#, "Added in v6.44". `-rollhero` was added in 6.60.
- `-random`: gives "250 bonus gold".
- `-repick`: costs 100 gold, or 400 in -ar/-tr; "only in the first minute".
- `-afk` and `-kickafk #`: 6.50.
- `-apm`: 6.50.
- `-courier`: 6.50.
- `-clear`: 6.50.
- `-invokelist`: 6.50.
- `-swap 1-5`, `-swapcancel`, `-swapall`: "only before 90 seconds mark". `-swap` was added in 6.46.
- `-ah` (Anti-MapHack): "Must be entered … between 15 and 60 seconds of game time. Added in v6.50".
- `-test`: 6.48.
- Single-player commands: `-lvlup`, `-refresh`, `-spawncreeps`, `-powerup`, `-neutrals`.
- `-ii`: not found in this list, UNVERIFIED. It may come from a later version.
- `-gg` (concede) did not exist in IceFrog's DotA. It was added in the community 6.85: "It can be used in order to fast finish the game … only available after 15 minutes." https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/

### 15. Creep pulling and stacking (neutral respawn)

**Rules.**
- At each minute mark a camp respawns if its spawn box is empty. Pulling the creeps out of the box just before the minute (about xx:52–:55) stacks the camp.
- Standing in the box, having vision of it, or placing a ward there blocks the spawn.
- "If you're too close to the spawn or have vision of the actual spawn, you will block the spawn which means no creeps will spawn … you can actually block spawns from spawning with a ward." https://tl.net/blogs/282827-dota-back-to-basics
- That blog shows stacks timed "between :51-53" and says "you can stack a camp up three times". That is Dota 2-beta behaviour; the DotA 1 stack cap is UNVERIFIED.
- Allstars-era guide (Dota-Utilities, Jan 2009) *(search snippet)*: "creep respawning is based on an invisible rectangle around each camp. To make new creeps spawn, you have to get the original creeps to follow you out of the rectangle." http://www.dota-utilities.com/2009/01/pulling-and-stacking-neutral-creeps.html (wording UNVERIFIED)
- Pull timings were reported identical in the port: Q: "What are the jungle pull timings like? … Can you still stack creeps with pulls?" A: "Same as in dota." #85–86, page 5. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota?page=5
  - A different poster: "the leftmost creep camp on scourge side can't be pulled" in HoN, implying it could be in DotA. #87.

### 16. Fountain, couriers, Roshan

- **Fountain.** The community 6.85 notes "fixed" both of these, so in 6.83 the fountain was attackable and healing started after a delay:
  - "Fountains are now invincible."
  - "Fountain zone now provides healing as soon as unit enters it, without any kind of delay."
  - URL: https://www.hiveworkshop.com/threads/dota-all-stars-gets-a-comeback.293401/
  - What the delay was in 6.83: UNVERIFIED.
- **Couriers.** In DotA 1 couriers were shared units that anyone could control. When a courier dropped items outside base, no notice was given; the 6.85 notes added one: "If the courier drops any of your items on the ground outside of the fountain, you'll be notified about it." (same URL)
  - Late-6.83 era: "Can now unshare leaver so it's courier won't be used for transfering your items". https://d1stats.ru/88n-betas-changelog/?lang=en
- **Roshan (as ported, 2011).** Aegis lasts 10 minutes, Roshan respawns 10 minutes later, and cheese drops from the third kill on. https://tl.net/blogs/282827-dota-back-to-basics

### 17. No matchmaking, leavers, hosting

- No matchmaking. Games were hosted lobbies on Battle.net, Garena and so on. Leavers were managed with third-party banlists: "A free 3rd party program that keeps track of leavers and attempts to prevent them from joining games." https://warcraft.wiki.gg/wiki/Defense_of_the_Ancients:_Allstars (via fetch summary)
  - "Third party programs also are against the battle.net terms of use." (same URL)
- "when a person leaves the game, it creates a favorable position for the opposing team." (same URL)
- Player view: "I also hated the idea of banlists in the original dota even though I never got banned, thanks god that was changed in DotA 2 to a less biased system." Hive. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
  - Reply: "DotA 2 really needs a surrender option." (same thread)
- The leaver's gold is split among the team that remains (from the v3.0a changelog, an early pre-IceFrog version, per search). https://dota2.fandom.com/wiki/Version_3.0a (UNVERIFIED wording)

### 18. Day and night

- Day and night switch vision ranges. Night Stalker's passive turns on at night. https://tl.net/blogs/282827-dota-back-to-basics
- The cycle length in DotA 1 (WC3 day length) is UNVERIFIED. It was not fetched.

### 19. DotA 1 vs Dota 2: other differences players reported

- Camera: "The camera movement feels a little odd on the first try". #2. https://tl.net/forum/dota-2-general/454320-little-differences-in-dota2-beta-vs-dota
- Projectiles: "Razor's lightning actually damages now when it lands". 5-s, #33, page 2.
- Visual clarity, cutting both ways:
  - "heroes such as CM, lina, zeus etc even though they have the animation timing carryied over, the graphic has been made much more clear and noob friendly". NB, #69, page 4.
  - "in dota 1 there were bright teamcolors atleast so it was easy to" spot heroes. Hive, 2014. https://www.hiveworkshop.com/threads/dota-vs-dota-2.257595/
- Ward pricing in the port: "wards you buy them in bunches of two for 200 gold". #60, page 3.
- Wanting things kept as they were: "I guess those DotA purists have got to have everything EXACTLY THE SAME." PSIDefenseUp, #58, page 3.
- The original thread opener's framing: "There were a lot of subtle things that the WC3 engine caused, and whether the developers meant for them to happen or they just came about, they became part of the game." Eeeegor, 16 Oct 2011, #1.

---

### Summary lists

**Things players say must not be simplified (they carry skill or balance)**
1. **Last-hit gold only, plus denying below 50% HP.**
   - Denied creeps should give the enemy only a little XP. Use the version-correct rule from the table in section 2.
   - Keep the deny "!" marker.
   - Sources: Liquipedia Denying; Hive post "The lack of creep denial was a letdown for me" (about LoL).
2. **Per-hero attack point and backswing, plus animation cancelling.**
   - Bad animations were balance tools (Lina, CM, Zeus, Razor), and players liked learning them.
   - Sources: TL #31, #42; InvalidID on the TL blog.
3. **Damage ranges per hero.** These set how hard a hero is to last-hit (CK vs Drow). TL #59, #73.
4. **Orb walking and the attack-vs-spell split for creep and tower aggro.**
   - Manual orb casts and spells must not draw aggro; right-clicking a hero must.
   - Sources: Liquipedia Attack Modifier; Steam tower guide.
5. **Deterministic tower targeting**, with the "A-click your own creep" reset. Steam tower guide.
6. **Trees as juke terrain and vision blockers**, with fog and night making escapes possible. Steam 2021 thread; TL blog.
7. **Body-blocking creeps**, including the ranged creep that slips past. TL #64, #66.
8. **Pull and stack timing at the minute mark**, and blocking a camp with a ward or by standing in it. TL blog; TL #85–86.
9. **High-ground vision asymmetry.** Liquipedia Vision.
10. **PRD for bash and crit procs.** "Some things are really affected by it, so it would be a big change." SKC, #77.

**Engine quirks people don't miss (OK to smooth, or offer as an option)**
1. Accidentally deselecting your hero, "clunky" selection. TL #27.
2. Spamming S/H to keep from auto-attacking. TL #50. An auto-attack toggle is fine.
3. Visuals that do not match the damage moment (Zeus's hammer, Razor's lightning hitting before the visual). TL #48, #33, #68. Keep the *timing* and fix the *visual*.
4. Global tree regrowth, trees changing projectile flight height, unclickable models. Hive 6.85 notes; TL #46.
5. Items stuck while on cooldown, courier recipe combining needing items dropped on the ground, toggling items interrupting orders, lost key presses. Hive 6.85; TL #18.
6. Fog and spell bugs, "paused"-state bugs, observers changing neutral AI, Sentinel bottom-lane creeps wandering into the forest. Hive 2014; TL #22, #54, #88.
7. Random order delay or input latency, if the "Random delay" claim holds. UNVERIFIED. Hive 6.85 notes.
8. Banlists and having no concede. Hive 2014. People wanted `-gg` and matchmaking.
9. Hidden mana bars and hidden enemy items. This is disputed: some treated clicking to inspect as skill (TL blog #16), while others called third-party mana bars "hacks" (TL #89).
