# Gravity Snake

A [Battlesnake](https://play.battlesnake.com) written in rust.

Tiles to avoid (walls, larger snakes) are given a positive value, while tiles to go towards (food, smaller snakes) are given a negative value.
Each tile influences all other tiles according to `tile score = sum(value / (radius^2))`, similar to the equation for gravity `g = (Gmm) / (radius^2)`.
The snake then "falls" towards the lowest scored tile beside its head. This snake probably won't do very well, but it was a fun experiment.
