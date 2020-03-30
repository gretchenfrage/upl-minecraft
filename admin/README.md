## Server Administration README

Currently, this document is only intended for myself (Phoenix).

#### Explanation of technology used

This server must:

1. Run MC 1.12.2
2. Run Forge Mod Loader (FML)
3. Support password-passed offline sessions, since some friends
   don't have MC accounts. This means serverside plugins.

We're using the [SpongeForge](spongepowered.org) MC server
because it supports Forge mods and serverside plugins simultaneously, 
and is not abandoned. 

([link to sponge docs for setting up server](https://docs.spongepowered.org/stable/en/server/index.html))

We're using SpongeForge build `1.12.2-2838-7.1.11-RC4007`, the Download
of which has been included in this repo as `spongeforge-1.12.2-2838-7.1.11-RC4007.jar`.
This SpongeForge build requires precisely Forge version `14.23.5.2838`,
the download of which has been included in this repo as 
`forge-1.12.2-14.23.5.2838-installer.jar`.

The latest version of Enigmatica 2 is version `1.77`. However, that version
requires at least forge `2847`. The latest version of `SpongeForge` is 
version `7.1.11-RC4007`, which requires exactly forge version `2838`.
For this reason, we are instead using Enigmatica 2 version `1.62`,
which is okay with forge version `2838`.

#### List of quirks

- Our server configuration must enable fly mode, because anti-fly-cheat 
   can trigger false positives with some forge mods.
- The default configuration of the FoamFix mod (which adds some optimizations)
   conflicts with SpongeForge. However, when SpongeForge errors, it points
   out the exact ways necessary to alter the file.
- Just like the modded minecraft client, the server requires JDK 8 
   (which is considerably outdated) to run correctly, due to general reliance 
   on `java.lang` implementation details.
- SpongeForge needs to run before some other mod we're using, or it will
   cause some sort of weird critical conflict relating to the loading of 
   different versions of a shared 3rd party library.

#### Cloud storage of large assets

The server generation procedure accesses some assets which are too big to 
store on GitHub (and to practically store in git in general). These assets
are instead stored in the public `mcupl-lfs` bucket on GCS (google cloud
storage), and consequentially can be accessed with a REST API. The 
`./serverfactory/download-assets.sh` script downloads any missing assets.

#### Explanation of server generation procedure

The directory `./serverfactory` contains a script, `make-server.sh`, which
generates the files necessary to run a server into `./serverfactory/target`,
which itself contains a `run.sh` script as an entrypoint. 

The procedure implemented by the `make-server.sh` script is as follows:

1. Call `./download-assets.sh` to download missing large assets from GCS.
2. Un-archive the enigmatica server zipfile. Although the server setup
   is generally useful, the setup's existing generator/launcher script 
   will not actually be utilized.
3. Use the forge installer to patch minecraft's server jarfile.
4. Install SpongeForge, by simply moving the SpongeForge jarfile into the
   `mods` directory.
5. Agree to minecraft's EULA by creating a `eula.txt` file containing 
   `eula=true`.
6. Copy over our pre-configured server settings (including disabling PVP,
   increasing world load distance, enabling RCON, enabling offline mode,
   and maybe some other stuff).
7. Copy over our `run.sh` entrypoint (which simply runs the patched 
   forge server jar with the appropriate java development kit).
8. Copy over our altered version of FoamFix's configuration to resolve
   a critical conflict with SpongeForge.
9. Copy over our trixtalogin config (that's our offline-login plugin)
   to give a really long login timeout duration.
10. Copy over our serverside plugins.

#### Server Supervisor

We do a lot of stuff here which I "will" write up an explanation for later.
