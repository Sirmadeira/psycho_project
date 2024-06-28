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
  1. We start with asset_loader_plugin. He loads all of our assets. He will run in parallel with the ui_plugin.
  2. Than we run the mod_char_plugin. He basically structures the base skeleton for our modular characters.
  3. After that we create our main physical rigidbody he controls movement and so on and after that we create the 
  Player in the player_effects_plugin. Basically an entity with a bunch of details
  4. Than we run the whole form_hitbox_plugin, the one who will handle the ragdoll state and the hitboxes. That well controls it.
  5. Thant we run the camera_plugin basically the guy who will follow our player. 
  6. All the other plugins are stateless meaning the run, right in the first frame.

Cheerios,
Sirmadeira!
****
