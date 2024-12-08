### LED

set_property IOSTANDARD LVCMOS33 [get_ports {led_o[*]}]
set_property SLEW SLOW [get_ports {led_o[*]}]
set_property DRIVE 8 [get_ports {led_o[*]}]

set_property PACKAGE_PIN F16 [get_ports {led_o[0]}]
set_property PACKAGE_PIN F17 [get_ports {led_o[1]}]
set_property PACKAGE_PIN G19 [get_ports {led_o[2]}]
set_property PACKAGE_PIN G15 [get_ports {led_o[3]}]
set_property PACKAGE_PIN G14 [get_ports {led_o[4]}]
set_property PACKAGE_PIN F20 [get_ports {led_o[5]}]
set_property PACKAGE_PIN G20 [get_ports {led_o[6]}]
set_property PACKAGE_PIN H20 [get_ports {led_o[7]}]
