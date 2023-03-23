/// The geometry module contains utility functions and structures related to geometry and
/// coordinates of the SSL field.
pub mod geometry;
/// This module contains the input struct of the robot's control system.
pub mod input;
/// The output module contains the output struct of the robot's control system.
pub mod output;
/// The tool module contains the tool struct of the robot's control system,
/// such as an annotation and a graph.
pub mod tool;
/// The world module contains the data structures for representing the state of the world
/// in the control system. It includes information about robots, the ball, and the field.
pub mod world;
/// The `annotation` module contains data structures and functionality for managing
/// graphical annotations to be drawn on the SSL RoboCup field viewer.
pub mod annotation;
