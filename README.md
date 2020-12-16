# Minotaur

A tiny project to learn Rust. Is not meant to be flexible or especially well-made; is just a small test-project. Is feature-complete, but is missing many many many layers of polish.

A game about escaping the Minotaur's labyrinth. One player is the Minotaur that chases the hero and the other is the hero who is attempting to escape the labyrinth.

The game ends when the Minotaur touches the hero or the hero touches the exit.

The labyrinth is invisible to both players, who can both only see tiles that are within 10 moves from their current location.

The hero can place torches that reveal the surrounding area to both players, and the Minotaur can reveal whether any nearby tiles are exits. If the minotaur uncovers an exit, it is revealed to both players. 

Some possible improvements for later:
- [ ] messages for whose turn it is
- [ ] graceful disconnect handling
- [ ] actually fun gameplay and better balancing (maybe add a turn limit (like the labyrinth collapses after 100 turns or something) to encourage movement)
- [ ] graceful handling of receiving garbage data over network
- [ ] better command line arguments (i.e. allow ports and IPs to be provided)
