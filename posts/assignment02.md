---
title: "Assignment 02: 3D design with movable parts."
author: "Fredrik Eide Fluge"
timestamp: "2023-09-12T20:37:00"
---

Our second assignment consist of creating a 3d model in fusion 360. The model should ideally be as fabricated as possible, without needing extra components like screws and bearings(not a requirement though).

I settled on creating an small toy car consisting of a hull and 4 wheels. The goal is to make a model that won't require any extra parts other than what's printed. For this I'll need to design a joint that will lock the wheels in place and still allow them to spin around. 

### Modelling the car
I Started the project by constructing the hull in Fusion 360. This consisted of making a cubiod with the dimensions 10cm*5cm*5cm. This cube was then sculpted into an car shape by cutting away parts on the sides.

<img src="images/assignment02/car-sketch-1.png" alt="sketch of car base" class="image"/>
<img src="images/assignment02/car-sketch-2.png" alt="extruded cuboid of car" class="image"/>

With the wonderful cuboid in place is was time to sculpt the car out. This involved creating a sketch along the side of the cuboid and using extrude with the cut method to make rough shapes in the car

For the first extrusion the car looked like this:

<img src="images/assignment02/car-sketch-3.png" alt="sketch for first sculpt" class="image">
<img src="images/assignment02/car-sketch-4.png" alt="first sculpt finished" class="image">

For the second molding on the front, It was necessary to do an intersection between the extrude of the mesh and the car hull. This was due to the sketch only bein bound to one of the faces on the hull.

The sketch and final sculpt looked like this:

<img src="images/assignment02/car-sketch-5.png" alt="sketch for second sculpt" class="image">
<img src="images/assignment02/car-sketch-6.png" alt="second sculpt finished" class="image">

With the rough shape in place. It was time to add more details to the hull. This was first done by applying a filet operation to some of the corner edges in the model. The operation made the hull look a little smoother.

<img src="images/assignment02/car-sketch-7.png" alt="smoothed out model" class="image">

I made small indents in the windows to make the model more details. Furthermore I added decals like lights and small blobs that acts like door handles.
This resulted in a car looking like this: (skipped sketches)

<img src="images/assignment02/car-sketch-8.png" alt="detailed car" class="image">

### Printing

#### Slicing and dicing

I used Bambu studio for slicing the model. The model was sliced by using a layer height of .2 mm and a start height of .2 mm. It printed the model in two steps. The wheels an axles, and the hull.
This was due to my inexperience with 3d printing, and Starting with the smallest parts was an easy way to test out the functionality of the Bambu P