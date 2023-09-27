---
title: "Assignment 03: Laser Cutting"
author: "Fredrik Eide Fluge"
timestamp: "2023-09-027T20:08:00"
---

Our third assignment consist of creating a 3D box by using CAD and a Laser cutter. The box should consist of panels held together by finger joints and be able to to be assembled without glue. Ideally not bigger than 10cm at each side.

When I started designing the model. I began by modelling the box directly to get the final measurements of each component. The project started by defining a set of parameters:

|  name  |  value  | usage  |
| --- | --- | --- |
| Box width | 100 mm | The width of a side. |
| Finger width | 5 mm | The width of a single finger. |
| Material width | 4 mm | The thickness of the material. Based of the listed thickness of plywood. |

(The kerf will come later.)

With these parameters in place I could design the first sketch of my box. 
The sketch was very simple and cosisted of a square with fingers on each side. The fingers where constructed by making a rectangle with the specified finger width and the material width as the height. I could then use a repeat modifier to extend them across the entire side. To spread the faces across the rest of the square I used the circular repeat modifier with the quantity set to four and the distribution to full. The center point was placed in the middle of the model by finding the diagonals with construct lines. By using 4 as the quantity each row of fingers where placed with 90 degrees difference, meaning they would line nicely up with each side.

<img src="images/assignment03/first-sketch-of-side.png" class="image"/>

After I was complete with my first sketch I used an extrude command to make a 3d volume. The height of the extrude was set to the material width, and my first piece was complete.

<img src="images/assignment03/completed-piece-first-it.png" class="image"/>

Now that I had completed a single piece in the box, I could start to test out if they would fit together. To do this i used a rectangle repeat modifier to first make one more copy of the side. Wich i colored blue to better differenciate between them.

<img src="images/assignment03/first-set-of-copies.png" class="image"/>

The reason i only made two copies was to modify one of the instances so that they would include the corners. This was done by creating a new sketch at each of the long rectangular corners on the pieces and modeling out a finger. This process was quite tedious, and If I'm doing it again some other time, I'll rather create a new Sketch with the same parameters and add the extra fingers on there. I think that would have saved me some time in the design process. But with the four extra fingers ready I could start to assemble the model.

<img src="images/assignment03/side-with-extra-fingers.png" class="image"/>

To make the remaining sides a made three copies of the side with no extra finger and one copy of the other. Each side was then turned into a sepparate component. To assemble the box I used the Joint tool to connect different pieces. The joins where placed at the side of the fingers and the model snapped together quite easily.

<img src="images/assignment03/sides-ready-for-assembly.png" class="image">
<img src="images/assignment03/placement-of-joint.png" class="image">
<img src="images/assignment03/three-sides-assembled.png" class="image">

With the cube complete it looked like this:

<img src="completed-cube.png" class="image">

It was about now that I realized if forgot to add skerf to my model. This lead to the entire model being redesign to make it easier to add the skerf parameter.

The redesign conisted of doing making the first sketch the same way as before, but this time as a construct. After the initial construct was done I made the actual model, but included the skerf offset at each side. The underlying construct allowed me to position and verify that the model with skerf aligned with the expected model. The resulting sketch can be viewed below(skerf has been increased for better visualizing).

<img src="images/assignment03/sketch-with-skerf.png" class="image">

When placing the fingers with kerf, it was necessary to figure out the new spacing between them, since this would be affected by the kerf, or so I thought. Since Fusion 360 calculated the distance between two repeated elements as the distance between the same edge on each instance, the distance would remain the same as for the non kerfed finger. This is due to the fact that The width of a finger is f+kerf while the distance between t2o fingers is f-kerf, so if we combine the two we get d = f+kerf + f-kerf = f+f = 2f. Meaning the distance is the same no matter the kerf.

