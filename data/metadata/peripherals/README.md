The generator will put transformed SVD data into the `raw` folder.
This is not used for anything later on, but serves for debugging purposes.
This is per device:

- A YAML per peripheral, directly from the SVD
- A `_debug_ir.yaml` that represents the non-metapac version of the pac. It's the result of running the SVD through the transforms.
  This is a good source when porting over a pac to the metapac.
- `_addresses.json`: A list of all peripherals and their addresses, taken from the SVD. Handy for adding it to the metadata.
- `_interrupts.json`: A list of all interrupts and their numbers, taken from the SVD. Handy for adding it to the metadata.

The other folders contain the manually curated YAMLs of the peripherals.
The exact names are not prescribed. NXP doesn't give us names, so it's up to us to create some sort of consistency there.
The metadata points to those YAMLs.
