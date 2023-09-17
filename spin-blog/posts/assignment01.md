---
title: "Assignment 01: CAD of final project"
author: "Fredrik Eide Fluge"
timestamp: "2023-09-17T19:25:00"
---

WHen starting the design of my semester project. I decided that one of the main hurdles to pass would be the main knitting mechanism. This was due to the threads overlapping a lot, meaning the entire spool of thread would need to be moved to prevent creating knots. My initial plan for this was to have 8 sticks placed in a fan pattern with the spools on their ends. The sticks would feed threads in to the middle of the fan where the final finger trap would be produced.

My sketch plan looked like this:

<img src="images/assignment01/initial-thread-spool.png" class="image"/>

For moving the pins i thought about having a set of lifting arms placed benath them. The prinicple is that there are "jump arms" and "slide arms" intertwined in the fan, these arms would then either lift the stick to its next destination, thereby jumping over a thread, or slide beneath it. The reason i think this mechanism could work is that a stick will always slide after a jump and vice versa.
Benath is an attempt at explaining the mechanism.

<img src="images/assignment01/Mechanism-explained.png" class="image" />

For making the arms i made two designes. One of the jumper and one of the slider. The slider will be supported above the sports for the jumper, meaning it can slide unobstructed beneath the model.

Currently in the sketch the arms are of equal length, but in the final sketch the arms will be shortened down to make space for an inner and outer resting ring for the sticks with spools, and the sliding arm will be shorter pass under the jumper.

jumper: 

<img src="images/assignment01/initial-lifting-arm.png" class="image" />

slider:

<img src="images/assignment01/initial-sliding-arm.png" class="image" />

I made a circular link that is meant to connect together the jumping arms and sliding arms respectively. Currently their hollow, but i imagine that they'll be filled and have a rail in them to keep them centered, this would make the knitter more structurally sound and would output the finger traps above the machine. Each link consist of 4 connectors, allowing 4 lifters to be connected together


<img src="images/assignment01/initial-connector.png" class="image" />


Together the knitter would look something like below, I have not added any supporting structure yet, since I have yet to land a proper model for the mechanism. Currently I'm considering using some form of actuators and a microcontroller like Arduino. The reason behind this decision is because a programmable system would allow better control over timing and movement, something i think is crucial in this project. But I'll explore different options, like using a geared system to drive the arms.

<img src="images/assignment01/initial-setup.png" class="image" />

All in all, I aim to fabricate most of this system myself. Ideally the only components required would be electronics, bearings and some screws, while the rest is locally produced on campus. The idea is not quite specced out yet, but I plan on testing out the mechanism in the following weeks

-- Fredrik Eide Fluge.