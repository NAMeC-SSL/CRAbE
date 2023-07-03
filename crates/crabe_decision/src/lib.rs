//! # CRAbE_decision
//!
//! This crate implements the main system AI for Robocup SSL Games using the
//! Manager-Strategies-Actions architecture.
//!
//! It includes several modules, such as action, manager, pipeline, and strategy.

/// The `action` module contains the definitions of various actions that can be
/// performed by a robot, such as moving to a certain point.
pub mod action;
/// The `manager` module is responsible for coordinating and executing the `Strategies`.
/// It contains multiple manager implementation, which is in charge of managing
/// the execution of the strategies.
pub mod manager;
/// The pipeline module contains the DecisionPipeline struct, which defines the pipeline
/// for making decisions for a SSL robot fleet based on the filtered input data.
/// The pipeline follows a Manager-Strategies-Actions architecture, where the Manager is
/// responsible for coordinating and executing the Strategies,
/// which in turn use `Action` to issue actions for each robots.
pub mod pipeline;
/// The strategy module contains the Strategy trait and various implementations of strategies.
/// Strategies are behaviors that one or multiple robots can adopt in order to achieve a certain goal.
pub mod strategy;

pub mod constants;