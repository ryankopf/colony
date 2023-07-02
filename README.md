# COLONY

COLONY is a Rust game, built with Bevy, that plays like any other colony simulator game. The current version is very basic, there are basic food items that your units can consume, your units have basic stats like hunger, and there is some basic map generation.

What are the various systems?
* "Seasons" represents the movement of time, and causes plants to grow and die.
* "Movetoward" is an A* algorithm that lets objects move in the most direct path, avoiding obstacles along the way.
* Spacebar will pause the game
* Escape will close the game
* Some of the menus don't work yet, but you can farm berries and trees, and a unit will go plant these
* Zones, walls, building, and crafting are not yet implemented
* The red box in the current version is meant to be a monster simulator, in the future there can be monsters like wild dogs that attack
* You can give Orders > Chop to chop down trees
* Units automatically sleep when they're tired.

## Next Steps

1. Some kind of UI box should show up when you click one of your units, displaying their health/hunger/sleepiness/stats/etc
2. Implementing more variety in foods, trees, etc, would be a good place to start
3. (Harder) Implement building things like walls, beds, etc.


## Getting Started

Generally ```cargo build``` and then ```cargo run``` works fine on Windows 11.

## Contributing

Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Art

The art may fall under different licensing conditions. Please see the specific licenses regarding any artwork.

Main tailset is from https://opengameart.org/content/dungeon-crawl-32x32-tiles and it is shown as CC-0.

## Contribution License

By contributing to this project, you agree to license your contributions under the terms of the MIT license. Additionally, you grant the project owner, Ryan Kopf, an unlimited, irrevocable, perpetual, universe-wide license to use your contributions for any purpose, including but not limited to commercial purposes, without any additional restrictions or obligations.
