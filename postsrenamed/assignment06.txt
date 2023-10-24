---
title: "Assignment 06: Input-Output"
author: "Fredrik Eide Fluge"
timestamp: "2023-10-24T21:20:00"
---

## Disclaimer
I'd like to start this assignment by saying that I don't think this work is the best i could do. I have chosen a very simple mechanism that doesn't necessarily fit the description of a sensor, but I hope i can make up for it by discussing around what I've identified as required for my knitting machine.

## Initial sketch of the circuitry.
After planning out what i wanted for electronics on the knitting machine. I landed on the following requirements for the components
### Input
The input should be a varying input device that allows for fine controll over the speed of which the knitter is operating. The component I landed on for the assignment is a b5k potentionmeter that has a resistance of 5k ohm. 

### Display
When you're using the knitter, you should have a simple display that gives you a descriptive explanation of what the system is doing. For this i decided to use 4 LEDs with the following meaning
| | | | |
| --- | --- | --- | --- |
| green | green | yellow | red |
| nice and slow | a little faster | god speed | maximum power! |

Ideally I'd like to replace this with a LCD display for writing some text, but for the inital prototype i think the LEDs will do a nice job.

### Knitter interaction
The electronicts should drive a motor that is attached to the knitter. I do have some stepper motors available with drivers that i bought a few years ago, but I do not have a suitable power source yet, so for the time being. The knitter is represented by a simple SG90 servo. [SG90 servo](https://www.kjell.com/globalassets/mediaassets/701916_87897_datasheet_en.pdf?ref=4287817A7A). The reason I'll be using the servo is that they use the pwm functionality of the Arduino for selecting the angle. I do think that i'll be using this for the driver. But of course this might change in the future.

## Digital sketch
Before I made the physical representation of the circuitry, a sketch was made at Tinkercad. Tinkercad delivers a free software for designing and simulation circuits, and this made it possible for me to see how my electronics would work without frying any LEDs. The digital sketch looks like this:

<img src="images/assignment06/Graphics-view.png" class="image" />

With the generated schematic sketch:

<img src="images/assignment06/Schematic-view.png" class="image" />

## Input data
The potentiometer is read by using Arduino's `analogRead(int pin)` method. This method yields a 5-bit integer value, this value should ideally be from 0 to 1023 and to represent the entire range, but for my prototype the best accuracy I could get was 100/130 to 1023. I tried to use different potentiometers, but the problem remaind so I. think the reason behind this offset in min values is a misconfiguration between the expected voltage read by the analog pin, and the actual voltage being delivered. The input is sampled 20 times before being averaged and used for constructing a percentage between 0 and 100.

```
  int amt = 0;

  for (int i = 0; i < 20; i++){
    amt += analogRead(A3);  
  }
  amt /= 20;

  int percentage = map(amt, 130, 1023, 0, 100);
```

## Rate of data

I've hadded a delay of 20 milliseconds to the main loop on the Arduino. This is because i don't think I'll need a high data rate since the input element doesn't change its state that rapidly. Most often you'll change the input once and leave it at that for a period of time.
For the amount of noise that is present in the data, I do think I'll need to find a way to reduce this in the future. Mostly to prevent jittering in the display and motors.

## Display

The display is just 4 LEDs connected to the digital pins 2 - 5. After the analog data has been read and mapped to the correct ranges, a simple check is performed to see if the percentage is above a given bound. If so, then the corresponding LED is turned on, otherwise it's turned off.

```
void loop(){
  int percentage = ...;
  setLed(led25p,  percentage,  0);
  setLed(led50p,  percentage, 25);
  setLed(led75p,  percentage, 50);
  setLed(led100p, percentage, 75);
}


void setLed(int led, int amt, const int limit){
  if (amt > limit){
    digitalWrite(led, HIGH);
  }
  else{
    digitalWrite(led, LOW);
  }
}
```

## Servo.

The servo is controlled in the same way as the LEDs, only that there is an additional mapping from the percentage to an angle between 0 and 180.
This angle is then written to the sero using the `Servo::write(int angle)` method provided.

```
int angle = map(percentage, 0, 100, 0, 180);
servo.write(angle);
```

## Physical product

The final circuit looks like this:

<img src="images/assignment06/circuit.jpeg" class="image"/>

It's powered with 5v from the usb port connected to the pc, but i suspect the final product will need an stronger power supply to support the motors. 

When powered the circuit works like this:

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment06/it-works.mp4"/>
</video>

As you can see there is some jittering in the servo when the output reaches 100% I think this is caused by me using a bad sampling of the input, and I wish to explore more aroung getting better results from the data. I'll also find a proper power supply for the motor I have so that i can make a proper mechanism that I can use later.

All in all, this was a very short assignment. I'm not totally happy with the result and I think I could have done better by using a different sensor and using more time at improving the code to reduce errors and jittering in the system. But due to other assignments and obligations I had a little less time than ususal, and I'mhappy with the result as it's something I can build upon later.