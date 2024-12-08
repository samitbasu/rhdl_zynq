update_compile_order -fileset sources_1
open_bd_design {/home/samitbasu/Devel/rhdl_zynq/tcl_me_rhdl/jnk/demo.srcs/sources_1/bd/system/system.bd}
Reading block design file </home/samitbasu/Devel/rhdl_zynq/tcl_me_rhdl/jnk/demo.srcs/sources_1/bd/system/system.bd>...
Adding component instance block -- xilinx.com:ip:processing_system7:5.5 - ps_0
Successfully read diagram <system> from block design file </home/samitbasu/Devel/rhdl_zynq/tcl_me_rhdl/jnk/demo.srcs/sources_1/bd/system/system.bd>
open_bd_design {/home/samitbasu/Devel/rhdl_zynq/tcl_me_rhdl/jnk/demo.srcs/sources_1/bd/system/system.bd}
create_bd_cell -type ip -vlnv xilinx.com:ip:axi_gpio:2.0 axi_gpio_0
apply_bd_automation -rule xilinx.com:bd_rule:board -config { Manual_Source {Auto}}  [get_bd_intf_pins axi_gpio_0/GPIO]
apply_bd_automation -rule xilinx.com:bd_rule:axi4 -config { Clk_master {/ps_0/FCLK_CLK0 (50 MHz)} Clk_slave {Auto} Clk_xbar {Auto} Master {/ps_0/M_AXI_GP0} Slave {/axi_gpio_0/S_AXI} ddr_seg {Auto} intc_ip {New AXI Interconnect} master_apm {0}}  [get_bd_intf_pins axi_gpio_0/S_AXI]
Slave segment '/axi_gpio_0/S_AXI/Reg' is being assigned into address space '/ps_0/Data' at <0x4120_0000 [ 64K ]>.
