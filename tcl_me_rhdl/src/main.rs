use std::io::Write;
use tcl_me_rhdl::{self as tcl, Prop};

fn main() {
    let mut script = tcl::Script::default();
    let root_path = camino::Utf8PathBuf::from(".");
    let project_name = "demo";
    script.add(tcl::CreateProject {
        path: root_path.clone(),
        part: "xc7z010clg400-1".to_string(),
        name: project_name.to_string(),
        force: true,
    });
    script.add(tcl::CreateBlockDesign {
        name: "system".to_string(),
    });
    let ps0 = tcl::CellName::new("ps_0");
    let prop = tcl::Prop {
        name: "PCW_USE_S_AXI_HP0".into(),
        value: "1".into(),
    };
    let gp_clock = tcl::Connection {
        local_port: "M_AXI_GP0_ACLK".into(),
        remote_node: ps0.clone(),
        remote_port: "FCLK_CLK0".into(),
    };
    let hp_clock = tcl::Connection {
        local_port: "S_AXI_HP0_ACLK".into(),
        remote_node: ps0.clone(),
        remote_port: "FCLK_CLK0".into(),
    };
    script.add(tcl::Cell {
        id: "xilinx.com:ip:processing_system7:5.5".into(),
        name: ps0.clone(),
        props: vec![prop],
        pin_connections: vec![gp_clock, hp_clock],
        interface_connections: vec![],
    });
    let configs = [
        ("make_external", "{FIXED_IO, DDR}"),
        ("apply_board_preset", "1"),
        ("Master", "Disable"),
        ("Slave", "Disable"),
    ];
    script.add(tcl::ApplyBlockDesignAutomation {
        rule: "xilinx.com:bd_rule:processing_system7".into(),
        config: configs.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
        target_cell: tcl::Reference::Cell(ps0.clone()),
    });
    let rst_0 = tcl::CellName::new("rst_0");
    script.add(tcl::Cell {
        id: "xilinx.com:ip:proc_sys_reset:5.0".into(),
        name: rst_0.clone(),
        props: vec![],
        pin_connections: vec![],
        interface_connections: vec![],
    });
    // Create a port for the LED
    let led_o = tcl::PortName::new("led_o");
    script.add(tcl::CreateBlockDesignPort {
        name: led_o.clone(),
        direction: tcl::PortDirection::Output,
        port_type: tcl::PortType::Data,
        from: 7,
        to: 0,
    });
    // set_property -dict [list \
    //  CONFIG.C_ALL_OUTPUTS {1} \
    //  CONFIG.C_GPIO_WIDTH {8} \
    //] [get_bd_cells axi_gpio_0]
    let axi_gpio = tcl::CellName::new("axi_gpio_0");
    script.add(tcl::Cell {
        id: "xilinx.com:ip:axi_gpio:2.0".into(),
        name: axi_gpio.clone(),
        props: vec![
            Prop::new("C_ALL_OUTPUTS", "1"),
            Prop::new("C_GPIO_WIDTH", "8"),
        ],
        pin_connections: vec![],
        interface_connections: vec![],
    });
    // connect_bd_net [get_bd_ports led_o] [get_bd_pins axi_gpio_0/gpio_io_o]
    script.add(tcl::ConnectBlockDesignNet {
        items: vec![
            tcl::PortOrPin::Port(led_o.clone()),
            tcl::PortOrPin::Pin(axi_gpio.clone(), "gpio_io_o".into()),
        ],
    });
    script.add(tcl::ApplyBlockDesignAutomation {
        rule: "xilinx.com:bd_rule:board".into(),
        config: vec![tcl::ConfigEntry::new("Manual_Source", "{Auto}")],
        target_cell: tcl::Reference::Interface(axi_gpio.clone(), "GPIO".into()),
    });
    script.add(tcl::ApplyBlockDesignAutomation {
        rule: "xilinx.com:bd_rule:axi4".into(),
        config: vec![
            tcl::ConfigEntry::new("Master", format!("/{}/M_AXI_GP0", ps0)),
            tcl::ConfigEntry::new("Clk", "Auto"),
        ],
        target_cell: tcl::Reference::Interface(axi_gpio.clone(), "S_AXI".into()),
    });
    let sources_path = root_path
        .join(format!("{}.srcs", project_name))
        .join("sources_1")
        .join("bd")
        .join("system");
    let system_bd_path = sources_path.join("system.bd");
    script.add(tcl::GenerateAllTargets {
        path: system_bd_path.clone(),
    });
    script.add(tcl::MakeWrapper {
        path: system_bd_path.clone(),
        is_top: true,
    });
    let wrapper_path = sources_path.join("hdl").join("system_wrapper.v");
    script.add(tcl::AddFiles {
        paths: vec![wrapper_path],
        kind: tcl::FileType::Source,
    });
    script.add(tcl::AddFiles {
        paths: vec!["ports.xdc".into(), "clocks.xdc".into()],
        kind: tcl::FileType::Constraint,
    });
    script.add(tcl::CloseProject);
    let file = std::fs::File::create("jnk.tcl").unwrap();
    let mut buf = std::io::BufWriter::new(file);
    script.commands.iter().for_each(|cmd| {
        writeln!(buf, "{}", cmd).unwrap();
    });
    drop(buf);
}
