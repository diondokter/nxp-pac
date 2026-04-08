The generator when using the extract command will put transformed SVD data into the `raw` folder.
This is not used for anything later on, but serves for debugging purposes.
This is per device:

- A YAML per peripheral, with transforms applied and namespaces stripped. In order to develop transforms, refer to everything in the `raw/original` folder, which are the same files but without the namespaces stripped.
  These files are suitable to base your metapac peripheral definition files on.
- `_addresses.json`: A list of all peripherals and their addresses, taken from the SVD. Handy for adding it to the metadata.
- `_interrupts.json`: A list of all interrupts and their numbers, taken from the SVD. Handy for adding it to the metadata.

The other folders contain the manually curated YAMLs of the peripherals.
The exact names are not prescribed. NXP doesn't give us names, so it's up to us to create some sort of consistency there.
The metadata points to those YAMLs.
