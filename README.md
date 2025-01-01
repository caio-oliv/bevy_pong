# Pong

This project is an implementation of the game [Pong](https://en.wikipedia.org/wiki/Pong) using the [Bevy game engine](https://bevyengine.org/).

Checkout the [video demo](<TODO VIDEO URL HERE>).

## Features

- Game menu
  - Selection between game mode
  - Play
  - Exit
- Pause menu
  - Pause game
  - Resume game
  - Go back to main menu
- Game score
  - Register the players score
- Gamepad support
  - Support 2 gamepads
  - Menu actions
- 2 Players
  - Two players in the same instance
- AI
  - AI Player

## Build

Use the command `scripts/build-release.sh` to build the game for release. The final binary should be available in the `bin` directory.

To execute the game, run the command `scripts/run.sh`.

## Usage

The following describes the game controls and how to open and navigate the available menus.

### Main menu

The first menu to be shown, in the main menu it's possible to:

- Change between 2 players or AI by clicking in the second button;
- Start the game using the **"Play"** button;
- Exit the game in the **"Exit"** button;

### Game movement

The game movement is controlled by the keyboard, using the `W` and `S` for the main player (left of the screen) and `ArrowUp` and `ArrowDown` for the second player (right of the screen) in case the **2 Players mode** is used.

When the **AI mode** is used, only the main player is able to control the paddle.

### Gamepad

This game has gamepad support, although not all buttons support interaction with the gamepad.

On the main menu, it's possible to change the game mode using the `DPad` left and right. Its also possible to use the `Start` button to start the game.

With the game running, press `Start` to pause and resume the game. The paddle is controlled by the `DPadUp` and `DPadDown` button.

When running in the **2 Players mode** both players have gamepad support, with the first connected device assigned to the main player and the second device to the second player. In case the first device is disconnected, the second player controller is assigned to the first.

## Design

#### The Language

For this project I decided to use the [Rust](https://www.rust-lang.org/) programming language, appling the low level concepts learned in the Problem Sets using the C language during the course, but in this case, using a more modern language with a more robust Type System, pattern matching and the [Ownership and Borrowing](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) rules preventing the memory bugs that can commonly occur using C.

#### The Engine

[Bevy](https://bevyengine.org/) is a simple Game Engine built with Rust. It's data-driven, using a custom **ECS (Entity Component System)** to organize the game and engine logic.

Bevy is built around a few key concepts:

- **Entities**: These are unique identifiers representing game objects. Entities themselves don't contain any data; they are just handles or references to a group of components.
- **Components**: Components are simple data structures that store the state or attributes of an entity.
- **Systems**: Systems are functions that perform logic on the components of entities. Systems are responsible for processing the data in components and updating the state of the game world.

With this design, the engine can schedule the systems based on the components usage, enabling [parallel execution](https://en.wikipedia.org/wiki/Task_parallelism) of systems and [Cache-Friendly](https://www.baeldung.com/cs/cache-friendly-code) access of data.

Although the game build for this project is very simple, the engine supported every requirement to make this game, being a great option for games of different scopes.

### Window

The main window created for the game has a fixed size of 1280x720. This constraint is due to the required scale applied to the camera, to make the window always show the pong arena. This is a current limitation of the game, and can be solved by recalculating the scale of the camera based on the size of the arena.

Although the window cannot be resized, it's possible to set it to full-screen normally by the OS.

### UI

All the Game UI is made with the Bevy support for UI Nodes. A [UI Node](https://docs.rs/bevy/latest/bevy/ui/struct.Node.html) is presented differently from the normal game entities and can be stylized using a model similar to CSS.

Currently, button elements do not support focus or other state properties, neither keyboard or Gamepad navigation (like in *Big Picture*).

Most of the code that builds the Game UI is in the [UI module](src/ui.rs), divided by [reusable components](src/ui/component.rs), [in game UI](src/ui/in_game.rs), [Pause Menu](src/ui/pause_menu.rs) and the [Main Menu](src/ui/main_menu.rs).

### Arena

The arena in which the ball moves has the size of 100x50. It is composed of 3 entities, the parent entity, marked by the `Arena` component and 2 child entities as the top and bottom `Wall`. Both of those entities have a collider component to allow the ball to bounce when hitting the walls.

Another requirement of the arena is to delimit the area that the player can collide with the ball, preventing the point for its opponent.

### Ball

The ball entity is composed mostly by the `Ball` marker component and the `LinearVelocity` component, which defines the current ball velocity as a vector of 2 dimensions.

To make the ball collide with the arena and the paddles, a simple [AABB collision detection](https://developer.mozilla.org/en-US/docs/Games/Techniques/3D_collision_detection) was used, checking the ball circle collider with the other entities bounding boxes, and reacting in case of a hit.

Due to the simplistic approach, one common bug that can be encountered happens when the ball is moving fast and goes through the paddle, even when the paddle is perfectly positioned to collide with the ball.

### Paddle

The paddles are entities composed by the `Player` component, holding which player controls the paddle, the `Paddle` marker component and a `Collider` component. In case of an AI player, the paddle receives the `PlayerAI` component to facilitate the system queries for the AI movement.

To make the paddle movement, a system is implemented requesting the keyboard input and connected Gamepads. In case the required button is being pressed, the paddle moves based on its default velocity (50 units per second).

### Game score

Game score is updated each time a player marks a point, in this implementation, the system responsible to check if the ball leaves the arena broadcasts a `PointMarked` event.

Another system is responsible for updating the game state when the `PointMarked` event is received. After updating the game score, a new `GameDataUpdated` event is sent.

The in-game UI receives the `GameDataUpdated` event and displays the available game score.

This level of indirection is required, since the system that updates the game score and the system that displays the new game score could be out of sync if both were receiving events from the same source.

### AI

The AI movement can be described as an algorithm that predicts the future position of the ball and moves the paddle to intersect that position.

It uses the direction of the ball to make a ray cast into the end of the arena, taking as result the distance required for the hit, if possible. With the required distance, it's possible to calculate the position the paddle must move relative to the ball current position.
