## UW UPL / Phoenix K COVID-19 Minecraft Server

This server uses the Enigmatica 2 1.62 mod over MC 1.12.2, and 
supports password-based offline sessions for friends without MC
accounts.

#### Instructions

1. Install OpenJDK-8 and OpenJDK-8-JRE. This is not the latest Java 
   version, but it's what modded minecraft needs to run.
2. Download and install MultiMC ([link](https://multimc.org/#Download)).
   We'll use this as a modpack launcher. 
    - When it prompts you for the
      Java installation you'd like it to use, select Java 8. If it 
      doesn't prompt you, or you miss the prompt, go into 
      `MultiMC > Settings > Java > Auto Detect` and select Java 8.
    - Recommend that you configure it to allocate as much JVM memory
      as you can. Modded MC requires massive amount of RAM. Try 10 or 
      12GB or more if you have enough. If you missed the prompt,
      you can change it at 
      `MultiMC > Settings > Java > Minimum/Maximum memory allocation`.
3. Download the modpack zip file in this repo, `Enigmatica2-1.62.zip`.
   ([original link](https://www.curseforge.com/minecraft/modpacks/enigmatica2/files/2888191))
4. Add the modpack as a launch config by going 
   `MultiMC > Add Instance > Import From Zip > Browse` and select the 
   `Enigmatica2-1.62.zip` file. Name the config if you want.
5. Launch the game, by selecting the config, and clicking `Launch`. 
   This may take a very long time to load, especially on the first 
   time. 

#### Recommendations

1. Adjust your graphical settings from the main menu, not while in a 
   world. Adjusting settings in a world triggers a regeneration of 
   the world's GPU resources, which can overwhelm your system and 
   freeze or even crash minecraft.
2. In minecraft, go to `Options > Controls` and set `Auto-Jump` to `Off`.
3. In minecraft, go to `Options > Video Settings` and set `Mipmap Levels` 
   to 2 or blocks will look like crap from far away.
4. In minecraft, go to `Options > Video Settings` and adjust `GUI Scale`
   to your preference.

#### Optional: GLSL Shaders

If you have a sicko-mode GPU, minecraft has modded GLSL shaders. To 
install them on top of all this:

1. Download the `OptiFine_1.12.2_HD_U_F5.jar` in this repo.
2. Go to `MultiMC > (select config) > Edit Instance > Loader Mods > Add`,
   and add the `OptiFine_1.12.2_HD_U_F5.jar`.
3. MC's graphics settings will now have a Shaders menu, where you 
   can open the shader folder, and copy in shaders. This repo includes
   some shaders in the `shaders` directory.
