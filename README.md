# rogue-legion
Following [bracket's rust roguelike tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html), using legion instead of specs for ECS.

## differences:
| Chapter        | Comment  |
| ------------- |:----------------:|
|  1.2 | I've decided to implement movement as a System using a Velocity component instead of a separate function for the player. Since moving the player will involve a query anyway, maybe I'll move inputs to a system aswell. |
