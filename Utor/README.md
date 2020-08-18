# UTor

This is a utility for generating the world in which the game takes place.


## Terminology

 - **World Space** The actual Eucilidian (x, y, z) space occupied by the game world. Easy for compiler to understand, hard to use for generation purposes
 - **Ring Space** A unit of measurment based on anglular projection. Takes the format (x, y, arc), where arc is angle of the segment's sweep. used in module generation
 - **Module Space** The most practical space to use when building modules. Units of measurment are all in meters, and treats the module as if it were a flat polygon. 