# The Psycho Project

*Welcome, newcomers! This marks the beginning of my game development journey!*

This game aims to capture the essence of the long-dead duel games, reminiscent of an era I dearly miss. 
Since waiting for a sequel became tiresome, I've taken it upon myself to create one.

I aspire for this game to become both competitive and immensely enjoyable.

## MAIN IDEAS
  * Modular character customization - So I can later add multiple skins and utilze of ragdolls and so on. DONE
  * UI the idea of betting in your dueling skill
  * Extremely dynamic movement - Dashes, parry gun play and sword play 

## CURRENT DESIGN ORDER
1. **Asset Loading**:
   - The game begins by loading all necessary assets using `bevy_asset_loader`.

2. **Main Menu**:
   - After assets are loaded, the player is taken to the main menu.
   - Here, the player can configure settings and navigate between different menu states.

3. **Character Creation**:
   - Once the player opts to start the game, the character creation process begins in the `mod_char` module.
   - This step involves creating the player character, setting up animations, and triggering the player creation module.

4. **Gameplay**:
   - With the player character created, the game transitions into the gameplay(MyAppState::Ingame) phase where the action begins.

Cheerios,
Sirmadeira!
****
