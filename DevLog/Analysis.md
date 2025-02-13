# Analysis

## Introduction

The problem to be solved is: an online multiplayer game that is real time and allows you to create civilisations and go to war with other players. What is this for? Entertainment.

The game will be called "MonarchyTiles" (or "MT" for short. This will be how the project will be referred to from now on) as it will be a 2D tile based system and players can own certain tiles and be a "monarch" to them.

The requirements to build this project are:

+ A game engine to be the main backbone of the project and allow the drawing to a window.
+ A way of connecting two machines either on a local network or potentialy a global one. Using a global network will require port forwarding which can cost money where a local network does not. However, a local network will only working for two machines connected to the same LAN.
+ Software to create art for the game.
+ Software to create music and sound effects for the game.

![Demo of tiling game](images/demo_1.png)

TM is inspired by a web game called "atWar", however atWar is based off a real map of earth and not randomly generated terrain. It allows you to take turns to buy and move units and rounds only move on once players have all agreed or done the maximum actions they can. MT will not be like this as players can move their "units" at any time. TM will also have the ability to expand empires peacefully and allow for trading between players. There should also be some automation involved as not everyone can be constantly on their computers playing a game. Automation would involve resource gathering, resource processing and resource transport. Allowing automation for anything else may cause the game to perform unwanted actions and reduce the need to play the game.

TM is written in rust because ~rust is cool~ it is a memory safe language which is important when handling large amounts of data.

The game engine it uses is a cargo library called macroquad. Cargo is a popular library manager used with rust and macroquad can be added by using `cargo add macroquad` in the project directory. It can there be used in the main program.

Another library it uses is called noise which includes the perlin noise function so that natural terrain can be procedurally generated.

## Stakeholders

The stakeholders for this project are mainly my friends as one of them said they'd be really interested in this game and would play it and also some people in a programming server. This is because it is quite easy to organise play testing sessions amongst these groups. Their roles in these sessions is to comment on the game and give any feedback for improvement that I can implement.


## Survey

Firstly people were asked if they like chess. Why? Because chess is a very popular strategy game and although it is a much smaller scale to MT it will attract similar people so people who do like chess should be taken more seriously as they are more likely to play MT. The survey was sent to some friends and also to programming discord servers so as no surprise most people answered "Yes" for this question meaning most other answers can properly treat the major answer as the most important.

The second question asked if people like pixel art games which is just to help decide if the art assets should be pixel art or not. It will be kept 2D for simplicity and due to its large scale as a 3D game of this scale would be much harder to optimise.

The third question asked if people would like a single player game mode. The answers to this had the option to vote for NPC monarchs which also was also chosen as the majority. This means that I will have to develop a more complex automisation which can have parts taken and given to players to give them an easier time playing the game especially if they have large cities to manage.

The fourth question asked if people would like to create their own flags for their kingdoms which was answered with 100% yes meaning I will need to create a small art program or allow importing of images. As the game is online this does create a higher risk for younger players seeing things they should not and therefore some sort of moderation options should be added or the age rating of the game would be raised to prevent younger audiences from playing and restricts the accessability of the game. Because of this I will likely just make an art editor that has a small area so that anything nsfw will not be obvious and younger audiences can still play and test the game.

The fifth question asked if their should be limits to servers such as player count and map size. Most people responded to this with "limit both" which is likely a good thing as lots of people connecting to a server with an unlimited map size could cause the map to exceed the memory of the server and suck up the bandwidth when requesting sectors of the map to draw.

The sixth question asked if the game should continue running even if not all players are online. For example playeer 1 is online but player 2 is not connected to the server. The reason this was asked is because it could create an unfair advantage for people who do not have to spend time on other aspects of life to gaming. Restricting this could also cause issues of players being able to hold servers hostage by not joining them and then preventing everyone else from playing the game. Due to this and also most people answering with "yes", the games will run even if not all players are online at any one time.

The seventh question of this servey asked if players should receive automation options for trading, transport and resource management as these tasks are very repetative and automating them would increase the quality of play time in the game. Likely due to this large benefit the question was answered 100% with "yes".

The eight question asked if people used windows or linux. This was asked as if enough people use both then the game will be compiled for both and there is an almost even split between the use of the operating systems so the game will have to be compiled for both.

The last question was open ended and allowed users to suggest something to add to the game. My favourite answer is "ability to create alliances" which would be a very cool and useful tool to have. Another answer was "a good management system" which I hope to succeed in making.

## Problems

### Networking

As the game is online multiplayer and online there needs to be some way of connecting to other players. As running my own servers would be expensive and also have risks because I'm opening up my network to the world, I will likely rent servers or use free ones. A common option is AWS but I have no idea how to use that so I will likely use render, which is a website another student in my class used to make a chat website. The server doesn't need to go under much stress because players aren't constantly moving, however, if tile updates are sent by the player and the server just accepts it then this is an opening for hacks and creates an unfair advantage in the game. We also have to consider the risk of malicous attacks to the server which seek to shut it down. This means we have to rate limit users from sending data and also the size of packets players send to the server have to be as small as they can get.


## Algorithms

### Chunking

With a tile based game that will have very large map sizes, the storing and rendering of tiles must be very optimised so the game runs as smooth as possible. The first prototype for this is to use chunking. The first time I wrote a chunking algorithm I stored the x and y position of the chunk and the tile.

```rs
struct Chunk {
    x: isize,
    y: isize,
    tiles: Vec<Tile>,
}
struct Tile {
   x: f32,
   y: f32,
}
```

Which was extremely inefficient because all chunks held the same number of tiles so the `tiles: Vec<Tile>` could be replaced with a static array and then the index of the tile in that array could be used to get the coordinates instead of unnecessarily taking up 64 bits per tile.

This was later refined to be:

```rs 
const CHUNK_SIZE: usize = 16; // Isn't necessary to be 16 but it is a common size for chunks in games

struct Chunk {
    x: isize,
    y: isize,
    tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE],
}

struct Tile();
```

This is much better and to get the position of the tiles you just enumerate through the list to get the index and the tile and get the coordinates through `(i % CHUNK_SIZE, i / CHUNK_SIZE)`. As both `i` and `CHUNK_SIZE` are of the type `usize` no type conversion has to be done and the values are rounded appropriately.
This can be taken a step further as the chunks don't need to store their position. If we have a terrain struct that contains an array, it can act like the tiles array in the chunks. All we need is the dimensions of the map so that the x and y positions can be extracted properly and then fill the array with chunks. This results in the following:

```rs
const CHUNK_SIZE: usize = 16;

struct Tile();

struct Chunk {
    tiles: [Tile; CHUNK_SIZE*CHUNK_SIZE], // Square area
}

struct Terrain {
    position: Vec2,
    width: usize,
    height: usize,
    chunks: Vec<Chunk>,
}
```

The macroquad library includes a 2D vector data type which stores an `x: f32` and a `y: f32`. We use this datatype to store a root position of `Terrain` because otherwise the terrain would always be based off (0,0) and would only expand in the positive directions. An option could be to center the terrain but if we want other tiles to overlap and they are not properply sized then they won't be in the correct position with the tilemap`


But what information does the tile need to store? Having a tilemap of nothing would be quite pointless so the tile struct needs to store something to signify what it represents. At first I stored the value generated by the perlin noise function but that is an `f32` which is 32 bits to store a value that hardly ever changes so clearly an enum value would be better.

What tiles does the player need? Firstly, water is an incredibly important resource and forcing players to manage that could make the game quite interesting. It could also effect ground stability. A way of implementing this could be through perlin noise to determine the water density of the land.

