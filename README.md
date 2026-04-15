# DPTrenchcraft



## Usage

```

open cmd in the root of the project (DPTrenchcraft)
cargo run --release --  <input>.schem <output>.map
```

|Argument|Description|
|-|-|
|`input`|Path to a `.schem`, `.schematic` file|
|`output`|Path to write the `.map` file|

### Examples

```bash
# Basic conversion
cargo run --release --  build.schem out.map

Dependencies

`fastnbt` & `flate2` : Handles raw and gzip-compressed NBT schematic formats.
`rustmatica` : Reads `.litematic` Fabric schema designs. (I didn't care much about litematic schems so I do not think they work, they were part of the original broken project)
`mcdata` : Provides block ID context and parsing mapping.
`clap` : Drives the command-line argument interface.

## License

This project is licensed under the MIT License - see the \\\[LICENSE](LICENSE) file for details.

You need to install rust.

Manual work is still required after outputting, you still have to do quality control don't just post it as slop.
\*\*REQUIRED THINGS TO DO AFTER GENERATED\*\*

https://www.youtube.com/watch?v=d5BtADbOsCs (quick overview of how to make a Paintball 2 map with TrenchBroom and how to config TrenchBroom


1. Look at the generated <mapname>\_textures.txt and make sure you have every required textures. (I got most textures from https://codeload.github.com/InventivetalentDev/minecraft-assets/legacy.zip/refs/heads/26.1). Just be careful since they are 16x16 and will look bad in game, you want to open photoshop -> image -> image size -> 64x64 pixels and then save the edited image. By default most blocks are generated a 40x40x40 units size (that was the unit side that allowed players to fit in 1x2 wide gaps such as doors, which required a x scale of 0.625 and a y scale of 0.625 which is what is put on default on most blocks, so some special shaped blocks may need adjusting (like torches) gave torches a simplified texture for now.
(If you followed Jitspoe tutorial you should have your modding resources at C:\\Programs\\paintball2\_mapping\_resources\\quake2\\pball\\textures, I made a subfolder called Minecraft there to put the textures in, put a copy of the textures in a subfolder called Minecraft in your Paintball 2 game files by default : C:\\Games\\Paintball2\\pball\\textures) will be required for testing.


2. Open the map in Trenchbroom Select any transparent brushes (glass, leaves etc) right click on them, reveal in material browser, right click them on material browser, select faces, add trans33 + trans66 flag for them to use the alpha layer of the original png. Select the border brushes, assign them the sky1 or sky3 textures, (if you select sky1 you will want to follow Jitspoe tutorial on how to set up the sky which is in the video) (there is a custom skybox in the texture file, you only need to put it in env in your pball2 folder (by default C:\\Games\\Paintball2\\pball\\env)

3. Double check brushes, Most unseen brushes should be cleaned up automatically such as underground stones, but I did leave a small marge around dirt paths and air in caves, you will want to double check and delete any unused brushes. Stairs should generate properly, except corner stairs you want to use the cut tool (with grid size 2 or something) and select the part of the corner you want to cut, then press ctrl+enter until the part you want to cut becomes transparent then press enter). Wall torches will also need to be rotated, moved to the wall with a y offset. Also will need to assign water to water brushes.

4. Feel free to modify the terrain it is not too hard, although you need to be in a grid 40 which is not an option so grid 8 works well to keep everything aligned, any grass/plants will be culled since they would take too many brushes to remake so feel free to add low poly plants where you feel fit. if it is a team map feel free to decorate in red/blue brushes to help fill the paintball 2 vibe.

5. Manually add every point and brush entities set all of your maps config as you wish.

Known issues / quirks,

1. side of grass block textures repeats a tiny bit at the top because of the 40 unit height.
2. every glass type gets converted to normal glass, this can be changed in src/filter.rs
3. a lot of special blocks just gets deleted, such as doors and plants, this can also be changed in src/filter.rs doors just blocked the players and were unneeded for my projects
4. config.rs contains some const like the block size, it can be changed there and should be automatically calculated for most blocks. bewarn that you may need to change some calculations at the top of optimizer.rs for special blocks.
5. Floating island maps should always have a layer of dirt on top and underneath it to prevent it from being culled (aka make a giant dirt platform above and underneath that covers the entirety of the schematic)


