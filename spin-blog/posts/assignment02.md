---
title: "Assignment 02: 3D design with movable parts."
author: "Fredrik Eide Fluge"
timestamp: "2023-09-17T19:25:00"
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

The first models I printed was in the original size specified in the model. This caused problems due to the model not adhering to the print bed correctly, resulting in a failed print. After scaling the model up to 150% size The parts adhered correctly.
The first batch consisted of the wheels and axles for the car, as seen here:

<img src="images/assignment02/car-wheel-print-attempt-1.JPG" alt="First print of car" class="image"/>

The original diameter of the axle and wheel was not big enough, resulting in the wheel being unable to rotate correctly. This was corrected by increasing the diameter of the hole on the wheel, and decreasing the axle's diameter.

I only printed one wheel and axle the second time to test out if the new dimensions would work as expected. This only took about 40 minutes and the results where promising. I forgot to photograph the print process, but here is an image of the final wheel. The model was configured to use a layer height and base height of .2 mm with an infill of 10%.

<img src="images/assignment02/test-wheel-print.png" alt="The sliced test wheel" class="image" />

<img src="images/assignment02/car-wheel-test-print.JPG" alt="The test print of the wheel" class="image" />

The car would take a whopping 14 hours to print. I therefore printed one of the back wheel mounts to test if the wheel would fit and spin freely. For this i used a lower layer height of .1 mm to try and make the model look a little smoother. Other than that the configuration remained the same

<img src="images/assignment02/wheel-test-mount-print.png" alt="The sliced test mount" class="image">

I forgot to photograph the print process, but the final result worked quite well as seen in this clip:

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment02/car-wheel-in-test-mount.MOV"/>
</video>

When i was done fidgeting with the test prints, I started printing out the remaining three wheels.
I kept the same settings as with the mount, and the print took about 1 hour and 40 minutes. In hindsight i do regret not printing 4 wheels instead of 3, due to them looking 10 times nicer than the first test print.

<img src="images/assignment02/remaining-wheels-print.png" alt="slice of remaining wheels" class="image"/>

<img src="images/assignment02/remaining-wheels-in-progress.JPG" alt="remaining wheels in print" class="image"/>

<img src="images/assignment02/all-wheels-ready.JPG" alt="all wheels." class="image" />

Now that the wheels are ready it was time to print the hull itself. The original plan was to print the car upside down. This was due to the wheel mounts having potruding parts that was meant ot hold the axle in place. This resulted in a lot of overhang, which i tried to fix by using tree supports as seen here:

<img src="images/assignment02/hull-bad-print.png" alt="bad slice of hull" class="image" />

<img src="images/assignment02/bad-print-in-progress.JPG" alt="bad print of hull" class="image" />

This hull kept coming loose from the build plate, and multiple times the supports would brake due to being to fragile. In the end, I ended up with cutting of the potruding parts under the car and putting it flat at its bottom. I also desided to use a raft to keep it at place and it appeared that this would stick troughout the night. 

<img src="images/assignment02/hull-good-print.png" alt="good slice of hull" class="image"/>

<img src="images/assignment02/hull-soon-finished.JPG" alt="hull soon finished" class="image" />

<img src="images/assignment02/hull-finished-print.JPG" alt="hull finished" class="image" />

Now that the car was finished printing, it was time to put it all together.

<img src="images/assignment02/car-pre-assembled.JPG" alt="car parts" class="image" />

<img src="images/assignment02/car-finished.JPG" alt="car finished" class="image" />


There was a small error in my measurement with the wheel mount on the front, so the wheels are 1-2 mm to far back, meaning the do scrape a little against the hull, but it seems to work fine while driving forwards.

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment02/car-driving-test.MOV"/>
</video>

All in all, this has been a fun and entertaining project to train on using Fusion 360 and Bambu studio to print out functional components that fit together. There are aspects of the car that i would have done differently if i where to do this again, e. g. structuring the project better and desiging the parts with 3d printing in mind. 

-- Fredrik Eide Fluge