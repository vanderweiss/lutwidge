# lutwidge

`lutwidge` is a command-line utility made for patching games from the Black Souls franchise, authored by [@toro_yori_ebi](https://twitter.com/toro_yori_ebi). 
These patches provide fixes for dependencies and assets to run the the franchise through Steam's proton on Linux natively.

If you want to support the original work, please buy the games from https://www.dlsite.com/maniax/circle/profile/=/maker_id/RG33488.html.

## Overview

For all the franchise's games to work properly, they require dynamically loading RPG Maker's RTP dependencies on startup. These dependencies,
packed together as the RPG Maker VX Ace RTP, serve to provide generic assets and portions of the engine essential for any game made with it to run.

On Linux, as the framework for handling dynamic dependencies is vastly different, it is required to make some adjustments. In this case, packing the
game with the dependencies themselves altogether, and merging them into a single workspace. This removes the issues that would arise from any attempts to locate them. 

## Dependencies
- un/zip 
- p7zip