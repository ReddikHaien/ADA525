$fa = 1;
$fs = 0.4;


module laptop(lid_rotation, color = "#4b4d51"){
    laptop_width = 32;
    laptop_depth = 20.5;
    color(color){        
        cube([laptop_depth, laptop_width, 1], center=true);
        
        translate([-laptop_depth/2,0,0])
        rotate([0,lid_rotation,0])
        translate([-laptop_depth/2,0,0])
        cube([laptop_depth, laptop_width, 1], center=true);
    }
}

module dock_port(color = "#b804e0"){
    port_width = 9;
    port_depth = 4.5;
    port_height = 1.02;
    
    color(color){
        cube([port_depth, port_width, port_height], center=true);
        translate([0,-4.5, 0])
        rotate([90,0,0])
        cylinder(h=1, r=0.5);
    }
}

module stand_seating(color = "#04bbe0"){
    
    color(color){
    
    }
}

module stand_leg(color = "#84e004"){
    leg_width = 6;
    leg_thickness = 0.5;
    leg_length = 20;
    color(color){
        cube([leg_length,leg_width,leg_thickness], center=true);
        translate([leg_length/2, 0, 0])
        cylinder(h=leg_thickness, r=leg_width/2, center=true);
    }
}


//laptop(45);
//dock_port();


stand_leg();