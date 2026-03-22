# NYT CLInnections

The daily NYT Connections puzzle, right from your command line!

## How to play

- `cargo run` to play in online mode
- `cargo run -- <puzzle>.json` to play in offline mode

## Offline mode

Offline mode is a feature that allows loading your own puzzle from a local
JSON file. The format is outlined below. A sample JSON file is also provided
under `examples/`.

```json
{
    "connections": [
        {
            "color": "Yellow" | "Green" | "Blue" | "Purple",
            "words": [
                "Word1",
                ...
            ],
            "hint": "Hint"
        },
        ...
    ],
    "seed": 42 # Determines the word ordering
}
```
