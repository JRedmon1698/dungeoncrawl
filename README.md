Just a little dungeon crawler game written in rust based on Herbert Wolverson's excellent *Hands-on Rust* 
(https://www.amazon.com/Hands-Rust-Effective-Learning-Development/dp/1680508164)

### Installation
* Clone the repo `git clone https://github.com/JRedmon1698/dungeoncrawl.git`
* Install rust and cargo (the rust package manager)
   * `curl https://sh.rustup.rs -sSf | sh` (macOS & Linux)
   * for Windows: https://win.rustup.rs/
 * Navigate to the root of the project and run `cargo run --release`


### Gameplay
Make your way through each level of the dungeon as you search for the venerable Amulet of Yala. 
Along the way you will face increasingly dangerous monsters, but also useful items to help keep you alive.

In this roguelike dungeon crawler, each time you take action, monsters will also take action. 
If you stand still, everyone else will, too. 

Use the arrow keys to move around. 
To attack monsters simply move toward them, but be careful: they attack back. 
Monsters will come after you if they see you, so use your corners and corridors effectively to evade them.

To pick up an item, step over it and press `g`. To use a consumable item like health potions and maps, 
press the corresponding number represented for that item in the top left HUD.

Make it to the final level and find the Amulet of Yala and you win!
