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

For my second iteration of ideas i tried to make a railsystem for the spools. The idea was that each spool would follow a designated track around the weave. Each set of spools would go in oposite directions and follow zig zag rail that would cross each other. My inital challenge with this desing was to make sure that each spool would go down the correct path when they would reach the intersection. I first thought about using a flipper that would alter between the two potensial paths. This could either be controlled by the motors or by the spool itself, making it close after a spool passes. The intersection would look something like this:

<img src="images/assignment05/latched-intersection.png" />

I tried to make a test design for this mechanism, but it fell short due me not realizing how to make the latch work, I then started exploring with using a sliding rail that would function like a train junction and slide in the new railway after a spool has passed.

<img src="weave-iteration-2-5.png" />

This design did seem promising at first, but the timing mechanism required to make it work seemed like a lot of uneccesary complexity, so I scrapped the idea and went back to the drawing board on the rails.

## The slightly better better idea, boats and rails.

While i was designing the rails i search a lot around to see if i could find similar mechanisms to the thing i was working on. This was when i found the holy grail to having multiple parts move in zigzag motion. While looking at lego weaves on youtube, I stumbled upon a lego __braiding__ machine. This machine moved 3 spools around in a figure eight pattern, and did exatly what my machine needed to do, only with a few more wheels and intersections included. This amazing discovery lead me do a deep dive in the world of braiding and braiding machines.

One of the best displays i could find on my mechanism was a video by thang010146 on Youtube named "Drawing eight-shaped line 3". This video displayed a model being moved around in an eight-shape, only using gears for motion. The mechanism behind this movement was the boat shaped guide that followed the track. This meant that when the boat reached the intersection it could be forced into the correct rail without a second mechanism directing it, this was a very genius method, and ultimately what i decided to implement for my weave (or braider). <a href="https://www.youtube.com/watch?v=2dVaRpPl9Vg">Link to the video.</a>

After some more digging i found out that what i was trying to make is called a may pole braider, and is used to create hollow circular braids, and the mechanism itself is a horn gear braider, and is a trusted and robust mechanism for making braids. <a href="https://en.wikipedia.org/wiki/Braiding_machine">This was read here at Wikipedia</a>. 
I did also find a 3d printable version of the braider, but i decided to make my own version of it without stealing someone elses design.

## The best idea, a proper braider.

Now that I was corrected in terms of calling it a weave rather than a braid, I could start to design the braider(I'll from now on refer to it as a braider rather than a weave).
When i designed this version of the braider i wanted to make sure That the placement of the gears and the grooves was correct. Here is the final version of the _first_ draft of the braider.

<img src="images/assignment05/weave-iteration-3.png"/>

### The gears and grooves.

When i 