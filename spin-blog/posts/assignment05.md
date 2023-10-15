---
title: "Assignment 05: Prototype 1."
author: "Fredrik Eide Fluge"
timestamp: "2023-09-12T20:37:00"
---


## The big flop idea.
When I first started this project, I had no idea how to attack this weave. I realized quite early that I could not directly follow the same tactic as traditional veawes due to the spools needing to move around. This lead to the first iterations of ideas wich resolved around arms being moved around.

<img src="images/assignment05/weave-iteration-1.png" />

The principle was that the blue arms would lift up the thread spool while the black arms would slide them across, meaning that the threads would be woven together in the correct pattern. After sketching out this thought and thinking about it more I realized this would become a very messy solution. It required a lot of moving parts and there was no easy way to fit it all together without making the weave large. The threads would also be spaced out quite far, leading to looser masks in the woven material. I did try to work around the problems with the thread being to far apart and the arms requiring a lot of space, but in the end I scrapped the idea and moved onto a rail based system.

## The sligthly better idea, rails.

For my second iteration of ideas I tried to make a railsystem for the spools. The idea was that each spool would follow a designated track around the weave. Each set of spools would go in oposite directions and follow zig zag rail that would cross each other. My inital challenge with this desing was to make sure that each spool would go down the correct path when they would reach the intersection. I first thought about using a flipper that would alter between the two potensial paths. This could either be controlled by the motors or by the spool itself, making it close after a spool passes. The intersection would look something like this:

<img src="images/assignment05/latched-intersection.jpeg" />

I tried to make a test design for this mechanism, but it fell short due me not realizing how to make the latch work, I then started exploring with using a sliding rail that would function like a train junction and slide in the new railway after a spool has passed.

<img src="images/assignment05/weave-iteration-2-5.png" />

This design did seem promising at first, but the timing mechanism required to make it work seemed like a lot of uneccesary complexity, so I scrapped the idea and went back to the drawing board on the rails.

## The slightly better better idea, boats and rails.

I searched a lot around while designing the rails to see if there existed something similar. I then found the holy grail to having multiple parts move in zigzag motion. While looking at lego weaves on youtube, I stumbled upon a lego __braiding__ machine. This machine moved 3 spools around in a figure eight pattern, and did exatly what my machine needed to do, only with a few more wheels and intersections included. This amazing discovery lead me do a deep dive in the world of braiding and braiding machines.

One of the best displays I could find on my mechanism was a video by thang010146 on Youtube named "Drawing eight-shaped line 3". This video displayed a model being moved around in an eight-shape, only using gears for motion. The mechanism behind this movement was the boat shaped guide that followed the track. This meant that when the boat reached the intersection it could be forced into the correct rail without a second mechanism directing it, this was a very genius method, and ultimately what I decided to implement for my weave (or braider). <a href="https://www.youtube.com/watch?v=2dVaRpPl9Vg">Link to the video.</a>

After some more digging I found out that what I was trying to make is called a may pole braider, and is used to create hollow circular braids, and the mechanism itself is a horn gear braider, and is a trusted and robust mechanism for making braids. <a href="https://en.wikipedia.org/wiki/Braiding_machine">This was read here at Wikipedia</a>. 
I did also find a 3d printable version of the braider, but I decided to make my own version of it without stealing someone elses design.

## The best idea, a proper braider.

Now that I was corrected in terms of calling it a weave rather than a braid, I could start to design the braider(I'll from now on refer to it as a braider rather than a weave).
When I designed this version of the braider I wanted to make sure That the placement of the gears and the grooves was correct. Here is the final version of the _first_ draft of the braider, this version was not properly parameterized and functioned as a guide for the actual product.

<img src="images/assignment05/weave-iteration-3.png"/>

### The gears and grooves.

The way a horn gear braider moves the spools around is by using gears with grooves on top. The grooves move the spool around and on each intersection between the gears the grooves align up to allow the spool to pass over to the next gear. This movement is forced by a bump on the rail that guides the boat(If you look at the center piece that lies just above the gear and grooves it looks like small horns). 
<img src="images/assignment05/horned-mechanism.png" />

The first thing I designed was a base for the braider. I wanted to have eight gears, so I made a gear axle and rotated it around with the circular pattern. The dimension of this base was about 30cm,

<img src = "images/assignment05/first-base.png" />

The gears and grooves where designed as a single component, with the grooves directly connected to the gear body. The gears had 24 theeth and there where 3 grooves per gear. This relation meant that each groove would be placed correctly in relation to the theeth underneath, meaning they would align with the other gears. 
I created to distinct gears, one where the grooves align exactly with a thooth underneath, and one that was offset by 360/24 deegrees(7.5). This meant that when the gears interlocked the grooves would align nicely, and the model could be built by alternating between the two gear types. I colored the aligned gears orange and the offset gears lime to make them easier to differenciate.

<img src = "images/assignment05/aligned-gear.png"/>
<img src = "images/assignment05/offset-gear.png" />

With the gears ready for action I could repeat them around the base and add rotational joints. These joints where then rotated to prevent overlap in the model, and each joint was connected with a motion link to make them move like actual gears. This allowed me to see if everything was aligned correctly.

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment05/animated-gears.mp4"/>
</video>

With the gears in place I could start to work on the horned plate that would guide the boat in the intersection. I did struggle a little bit with the scaling on the horns, but in the end, the horns where designed so that they would form a straight line down to the other rail. On the first draft it was guesswork that went into the design, but I changed it in the final prototype. More on that later. The design I went with in the first draft consisted of having a dedicated "bump height" that define how far out the bump would go. I would then have edges that would define an edge in the rail that would intersect the center rail piece of one gear and the outer rail piece sorounding the neighbouring gear. It might be easier to view in the below drawing

<img src="images/assignment05/explanation-mechanism.jpeg" />

The final sketch looked like the one below. It had a bump height of 4mm with a expected 10mm rail gap. In the final model a new approach to the horns where used. I'll discuss it more in detail later.

<img src="images/assignment05/first-draft-horns.png"/>

With the horns and gear in place it was time to design something printable. The currenct gear design and braider design was ill-fitted for 3d printing due to a lot of overhangs and pieces being large and complex. For my final design I wanted a system that consisted of a lot of assemblies that was easy to print and easy to assembled. For this challenge I decided to start with designing the gears and grooves, since they represented a printing challenge, and having them in place would make designing the rest of the braider easier.

For the gears I defined the parameters:

|  name  |  value  | usage  |
| --- | --- | --- |
| BraiderDimension | 300 mm | Defines the outer dimensions of the braider |
| GearOffset | 220 mm | defines the dimension of the circle where the gears are placed |
| GearAxleDimension | 30 mm | The dimension of the gear axle |
| NumGears | 8 | How many gears to use in the model |
| GearSpacing | 360 / NumGears | The number of degrees between each gear |
| GearDimension | sqrt(2\*GearOffset^2\*(1- cos(GearSpacing))) | Slightlty misleading name, but it's the distance between the center of two gears, the actual dimensions is half this value |
| GearHeight | 10 mm | The height of the gear |
| GrooveDimension | 15 mm | The dimension of the grooves |
| NumGrooves | 4 | Number of grooves to place on top |
| GearGrooveConnectorDistance | 40 mm | The distance from the center to place the connection pins for the grooves and gears.

The first gears had a wrong GearDimension due to a miscalculation from my side, and I originally wanted 4 grooves rather than three. This first design lead to the following models.

<img src="images/assignment05/first-iteration-parameterized-gears.png"/>

The above gear and groove had a wrong dimension and holes, but in the second iteration this problem was solved. The reason I wanted to have pins on the gears and holes on the groove plate was to print them as 2 flat pieces and just click them together. The benefit of doing this was that I didn't need to alter the gears if there was an error with the grooves, and I could reuse the same gear model rather than having 2 types.

Each of the groove plates uses the same sketch for the actual grooves while the offset needed to align them on the gears are added to the pin holes. The reason behind this decision is to make it easier to make modifications to the grooves without having to keep 2 set of sketches in sync.

The final sketch of the grooves can be viewed here:

<img src="images/assignment05/final-iteration-groove-plates.png" />

The two set of pinholes where designed by first adding a circle to the mesh with an angle of 0, and then adding the same amount of holes as there are grooves, the reasoning behind this decision was to make it easier to add more grooves without adjusting the holes, but I have realized that the placement would not work. For the offseted piece the inital hole is added with an angle of 7.5, to offset the grooves accordingly.
The pins on the gear are placed at an angle of 7.5 also, meaning that the aligned grooves gets the offset and the offseted groove is aligned. This was due to an error I made while creating the original sketch meaning that the grooves would always be offset by an small amount. I tried to counteract this error by offseting the pins on the model, and it was left in due to the added offset not breaking anything. The gear sketch also adds the hole for the axle. This axle goes trough both the gear and the grooves. This is to allow for connecting the top plate that will act as a lock and the rail for the boat.

<img src="images/assignment05/final-sketch-groove-align.png" />
<img src="images/assignment05/final-sketch-groove-offset.png" />
<img src="images/assignment05/final-sketch-gear-pins.png" />

### Pizza slices of the base

With the gears in place, I could start making the base. The braider I imagined would be about 30 cm in diameter meaning that I would need to print it in smaller pieces. For this I decided that I would make the base in 8 slices whith each slice containing one gear. This allowed me to focus on getting the rail and gear to work correctly, while also being sure that the base would repeat around without problems. The initial sketch of the base consisted only of the gear axle and gave a nice starting point to figure out how the remaining model needed to be added, the axled had a dimension of _GearDimension - 2mm_, this was to allow for the gear to spin around freely. I also added a few colors in this step to differentiate each model. 

<img src="images/assignment05/colorized-pieces.png"/>

The next step was to add the edges around the gear. These edges was designed to be slightly higher than the gear and groove combined. This was to prevent friction from the top rails that would be added later and the gear system beneath. Originally I had a extra height of 1mm, but this prooved to be too tight after printing so the final height became 2 mm. I started with the edge at the center of the model, since this would be a single extrude and it would be easy to see how it would fit. The edge was designed by making a circle with the gear dimension and a offset of 3 mm, this offset is due to the gears absolute offset being larger than the specified, since they are to intersect. When I assembled the model with joints I discovered that the 3 mm spacing I added was way to small, so in the final prototype this was increased to 10 mm. I think that I'm going to explore slightly to figure out a better way to calculate this extra spacing, so that I can change the gears without adding to much guesswork.

<img src="images/assignment05/sketch-base-edges.png" />

With the first part of the base in place it was time to design the horned top. In comparison to the version from the first draft, this model did not include a defined bump height, but rather defined the bump by making multiple lines that intersected the inner and outher spheres that defined the rails. Further on to place the bump on the opposite side I used the GearSpacing/2 to find the center line between the horns and a mirror. The base diameter of the model was _GearDimension/2 - GrooveDimension*2_

<img src="images/assignment05/initial-sketch-correct-top.png" />

After sketching and extruding out the top piece I added a connector hole to the base and a pin on the top. The diameter of the hole and pin was 10 mm and had a square shape. The reason for this was to make a good lock that could not be rotated, meaning that the angle of the top piece would be correct in realtion to the base and the gears. The pin on the top was rotated around by the GearAngle so that the horns would align with the intersection points of the gears.

Later in the design proccess I added a lip size to the top pieces. this was to lock the boat in place. The sketch above does not display the printed version, but I'll provide sketches an images of the final top pieces.

### Cleaning and improvement

When I had designed the top piece I realized that I needed to add a lip to act as a barrier to keep the boat in place. This was easy enough as defining a parameter called TopLip and setting it to a value of 8mm. this meant that the lip would go 4mm out over the grooves. To integrate the TopLip to the model, I added the value to the base radius of the top plate (I hindsight I should have added TopLip*2, so that the produced lip would actually be the length specified in TopLip, rather then the half). A problem I noticed after printing and assembling the prototype, was that one of the edges that was used to make the horn was not connected to the construction line used to define the tanges on the spheres, leading to a malformed bump. In the final version this should be corrected but is something that is wrong with the current model.

<img src="images/assignment05/expanded-top-with-error.png" />

### Base connector and the boat

For connecting everything together I decided to make a center piece that would connect every base part together. This piece would also act as a part of the rail system. This part consisted of a single segment, that was later rotated and merged to make the final component piece. The rail part of the model went trough a lot of designs and repairs to make it fit into the model(a lot of the work was due to me mixing up wether a parameter was radius or diameter). But the final piece turned out quite well and I think it's one of the better pieces I have designed.
<img src="images/assignment05/final-sketch-base-connector.png"/ >

With the bottom edge piece in place I had a full track that I could move a boat in, and it was time to design the test boat. The  boat consisted of a base circle with a diameter of _GrooveDimension - 2 mm_. This was to make sure that the boat wouldn't get stuck in the groove. The actual boat figure was added on top of this circle. To figure out how long the boat could be, I added a sphere with a diameter of _GearDimension / 2_. With center at the same distance away from the boat + an extra 2 mm to account for the width of the boat. I then added a line from the boats origo to the edge of the sphere on either side of the boat. This gave me an estimate on how long the boat could be. In the original calculation it looked like the boat could up to 4 cm long, something I thought was wrong. This lead to the first boat printed only being 2 cm long. the 2 cm boat had a lot of issues with falling down into the groove while at the intersection. This led to the second iteration of the boat being 3cm long, this looks like is just the right fit, but further testing is needed.

<img src="images/assignment05/boat-sketch.png" />

With the entire set of pieces ready for testing, I could assemble it together to view what the final product would look like.

<img src="images/assignment05/the-entire-ensemble.png" />
<img src="images/assignment05/the-assembled-ensemble.png" />

### Printing
The first parts I printed of the prototype was the groove piece. It was small and simple, meaning the model would be a fast print. It took about 20 minutes to print. The piece turned out quite nicely. After the succesful first print, I started printing the base, a gear and the top connector at once. I was a little afraid that it wouldn't be able to print it all, but the result was good and the assembled together nicely. In the original print i noticed that the gear did not spin properly. This was caused by the gap between the top piece and the gear being to narrow, causing a lot of friction. In the second print i corrected for this and it seemed to flow better. 
Finally with the adjusted values for the base, I could print out the second piece of the puzzle. When this was done, I wen't a little fast and added it to the previous piece with the base connector. This meant that the gear could not be added, and I learned the hard way that the pin would not let go of the base. So i had to break it of and tape it back together. I also printed a new top because I broke the previous one and a new boat that was longer:

<img src="images/assignment05/final-set-of-pieces.jpeg" />


Here is how to assemble the gear:

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment05/assembling-the-gear.mov"/>
</video>

<img src="images/assignment05/final-gear-assembly.jpeg" />

With the pieces ready it was time to assemble the prototype. due to the broken pieces, I had to tape together some of the parts, and I have discovered that the is still a little short at 3cm, meaning ot can bend just a little to much to make it past the intersection, but I do believe that the potential is there and that the machine is on the right track.

Here is the actual mechanism in play. It needed some help with moving around, so I'll be working on fixing this.

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment05/the mechanism.MOV"/>
</video>

