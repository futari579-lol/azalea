# Azalea Protocol

A low-level crate to send and receive Minecraft packets. You should probably use `azalea` or `azalea-client` instead.

The goal is to only support the latest Minecraft version in order to ease development.

This is not yet complete, search for `TODO` in the code for things that need to be done.

Unfortunately, using azalea-protocol requires Rust nightly because [specialization](https://github.com/rust-lang/rust/issues/31844) is not stable yet. Use `rustup default nightly` to enable it.

## Adding a new packet

Adding new packets is usually pretty easy, but you'll want to have Minecraft's decompiled source code which you can obtain with tools such as [DecompilerMC](https://github.com/hube12/DecompilerMC).

1. First, you'll need the packet id. You can get this from azalea-protocol error messages or from [the wiki](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge).
2. Run `python codegen/newpacket.py [packet id] [clientbound or serverbound] \[game/handshake/login/status\]`\
3. Go to the directory where it told you the packet was generated. If there's no comments, you're done. Otherwise, keep going.
4. Find the packet in Minecraft's source code. Minecraft's packets are in the `net/minecraft/network/protocol/<state>` directory. The state for your packet is usually `game`.
5. Add the fields from Minecraft's source code from either the read or write methods. You can look at [the wiki](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol) if you're not sure about how a packet is structured, but be aware that the wiki uses different names for most things.
6. Format the code, submit a pull request, and wait for it to be reviewed.

### Implementing packets

You can manually implement reading and writing functionality for a packet by implementing AzaleaRead and AzaleaWrite, but you can also have this automatically generated for a struct or enum by deriving AzBuf.

Look at other packets as an example.
