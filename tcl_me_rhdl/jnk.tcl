create_project demo . -part xc7z010clg400-1 -force
create_bd_design system
set cell [create_bd_cell -type ip -vlnv xilinx.com:ip:processing_system7:5.5 ps_0]
set_property  CONFIG.PCW_USE_S_AXI_HP0 1 $cell
connect_bd_net [get_bd_pins ps_0/M_AXI_GP0_ACLK] [get_bd_pins ps_0/FCLK_CLK0]
connect_bd_net [get_bd_pins ps_0/S_AXI_HP0_ACLK] [get_bd_pins ps_0/FCLK_CLK0]

apply_bd_automation -rule xilinx.com:bd_rule:processing_system7 -config {
make_external {FIXED_IO, DDR}
apply_board_preset 1
Master Disable
Slave Disable
} [get_bd_cells ps_0]

set cell [create_bd_cell -type ip -vlnv xilinx.com:ip:proc_sys_reset:5.0 rst_0]

create_bd_port -dir O -from 7 -to 0 -type data led_o
set cell [create_bd_cell -type ip -vlnv xilinx.com:ip:axi_gpio:2.0 axi_gpio_0]
set_property  CONFIG.C_ALL_OUTPUTS 1 $cell
set_property  CONFIG.C_GPIO_WIDTH 8 $cell

connect_bd_net [get_bd_ports led_o] [get_bd_pins axi_gpio_0/gpio_io_o]
apply_bd_automation -rule xilinx.com:bd_rule:board -config {
Manual_Source {Auto}
} [get_bd_intf_pins axi_gpio_0/GPIO]

apply_bd_automation -rule xilinx.com:bd_rule:axi4 -config {
Master /ps_0/M_AXI_GP0
Clk Auto
} [get_bd_intf_pins axi_gpio_0/S_AXI]

generate_target all [get_files ./demo.srcs/sources_1/bd/system/system.bd]
make_wrapper -files [get_files ./demo.srcs/sources_1/bd/system/system.bd] -top
add_files -fileset sources_1 {./demo.srcs/sources_1/bd/system/hdl/system_wrapper.v }
add_files -fileset constrs_1 {ports.xdc clocks.xdc }
update_compile_order -fileset sources_1
launch_runs impl_1 -to_step write_bitstream
wait_on_run impl_1
open_run [get_runs impl_1]
set_property BITSTREAM.GENERAL.COMPRESS TRUE [current_design]
write_bitstream -force -file ./system.bit

close_project
