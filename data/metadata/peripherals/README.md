# Metapac peripherals
This directory contains the register and type definitions for peripheral IPs that are included in NXP chipsets.

NXP does not give us the internal name of these peripheral IPs. (for example: Synopsis USB-OTG rev. 2)
It's up to us to create some sort of consistency there.

These files are manually curated and considered to be the 'source of truth' when generating the metapac Rust source code.
Even though they are assembled by hand, they are derived from the SVD files provided by the vendor.

This derivation ideally happens through chiptool transforms, but manual changes to these files are also allowed.
When adding manual changes, please note them with appropriate comments in the YAML files.



## Updating
The generator when using the `extract` command will put transformed SVD data into the `raw` folder.
Anything in this folder is not used directly, but can be used by you as a start to define metapac peripherals and chips.

This is per device:

- A YAML per peripheral, with transforms applied and namespaces stripped. These files are suitable to base your metapac peripheral definition files on.
- Everything in the `raw/original` folder, which are the same files but without the namespaces stripped. These files are the direct output of the transforms, and thus useful when developing the transforms.
- `_addresses.json`: A list of all peripherals and their addresses, taken from the SVD. Handy for adding it to the metadata.
- `_interrupts.json`: A list of all interrupts and their numbers, taken from the SVD. Handy for adding it to the metadata.

Thus the workflow to add or change a peripheral is as follows: (change MCXA577 to your chipset)
* Run `cargo run -p generator -- extract MCXA577`
* Open `/data/metadata/peripherals/raw/MCXA577/<peripheral>.yaml`
* Check if it is correct, if not change the transforms in `/data/transforms` and re-run `extract` until it is.
* Copy the file over, and check the changes compared to what was already committed when relevant.
* It is also allowed to change the file by hand at this point, but please use comments to denote what and why you changed (and why you didn't use a transform).
* Generate the new nxp-pac code by running `cargo run -p generator -- generate MCXA577`
* Check the code changes and commit both the nxp-pac code and the metadata definitions.