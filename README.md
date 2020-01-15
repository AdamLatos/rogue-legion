# rogue-legion
Following [bracket's rust roguelike tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html), using legion instead of specs for ECS.

## differences:
| Chapter        | Comment  |
| ------------- |:----------------:|
| 1.1 | Slight difference in how legion handles systems - instead of implementing a system for a component and running updates manually, you build them with a SystemBuilder and register in a Schedule.
|  1.2 | I've decided to implement movement as a system using a velocity component instead of a separate function for the player. Since moving the player will involve a query anyway, maybe I'll move inputs to a system aswell. |
