extern crate prost_build;

use std::path::{Path, PathBuf};

fn compile_packet(filename: &str, protos: &[impl AsRef<Path>], includes: &[impl AsRef<Path>]) {
    let mut build = prost_build::Config::new();

    build
        .default_package_filename(filename)
        .out_dir(PathBuf::from("src/protobuf"))
        .compile_protos(protos, includes)
        .unwrap_or_else(|_| panic!("Failed to compile {} protobuf files", filename));
}

fn main() {
    compile_packet(
        "simulation_packet",
        &[
            "protobuf/simulation/ssl_simulation_control.proto",
            "protobuf/simulation/ssl_simulation_robot_control.proto",
            "protobuf/simulation/ssl_simulation_robot_feedback.proto",
        ],
        &["protobuf/simulation/"],
    );

    compile_packet(
        "vision_packet",
        &["protobuf/vision/messages_robocup_ssl_wrapper.proto"],
        &["protobuf/vision"],
    );

    compile_packet(
        "game_controller_packet",
        &["protobuf/game_controller/ssl_gc_referee_message.proto"],
        &["protobuf/game_controller"],
    );

    compile_packet(
        "robot_packet",
        &["protobuf/robot/base_wrapper.proto"],
        &["protobuf/robot"],
    );
}
