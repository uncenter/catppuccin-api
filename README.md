# catppuccin-api

Soothing pastel REST API for querying Catpppuccin ports and metadata. Written in Rust, using the [Axum](https://github.com/tokio-rs/axum) framework, and deployed on [Fly.io](https://fly.io/). Data is sourced from the two YAML metadata files, [`ports.yml`](https://github.com/catppuccin/catppuccin/blob/main/resources/ports.yml) and [`userstyles.yml`](https://github.com/catppuccin/userstyles/blob/main/scripts/userstyles.yml), and somewhat restructured to combine the two sources.

## Routes

### `/ports`

> [!NOTE]
> List all ports and userstyles, _combined_. Use the `is-userstyle` field to differentiate between the two.

Returns an object, where the keys are the port identifiers and the values are arrays of ports matching the identifier (userstyles and ports might have duplicate identifiers, e.g. `mdbook`).

#### `/ports/:identifier`

Returns an array of ports matching the identifier.

### `/collaborators`

Returns an object, where the keys are the usernames and the values are objects containing collaborator information.

> [!NOTE]
> Duplicate usernames between ports/userstyles are resolved by picking one to use, simply assuming they are identical.

#### `/collaborators/:username`

Returns an object of collaborator information.

### `/categories`

Returns an object of category keys and values.

#### `/categories/:key`

Returns an object of category information.

### `/showcases`

Returns an array of showcases.

### `/palette`

Returns the color palette.

## License

[Apache-2.0](LICENSE)
