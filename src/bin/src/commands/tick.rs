//! `/tick` debug commands for the fluid simulation clock.
//!
//! These let a developer pause and single-step fluid spreading so the large cascade produced by a
//! single bucket placement can be inspected one game tick at a time, instead of being buried in
//! logs. Only the fluid simulation is affected; the rest of the server keeps ticking, so you can
//! still move around and place/break blocks while fluids are frozen.
//!
//! * `/tick freeze` — stop advancing fluid ticks.
//! * `/tick run` — resume normal fluid ticking.
//! * `/tick step [n]` — while frozen, advance fluid simulation by `n` ticks (default 1).

use crate::systems::fluids::FluidTickControl;
use bevy_ecs::prelude::ResMut;
use ferrumc_commands::arg::primitive::int::Integer;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_text::{NamedColor, TextComponentBuilder};

#[command("tick freeze")]
fn tick_freeze(#[sender] sender: Sender, mut control: ResMut<FluidTickControl>) {
    control.frozen = true;
    control.steps = 0;
    sender.send_message(
        TextComponentBuilder::new("Fluid simulation frozen. Use /tick step [n] or /tick run.")
            .color(NamedColor::Yellow)
            .build(),
        false,
    );
}

#[command("tick run")]
fn tick_run(#[sender] sender: Sender, mut control: ResMut<FluidTickControl>) {
    control.frozen = false;
    control.steps = 0;
    sender.send_message(
        TextComponentBuilder::new("Fluid simulation resumed.")
            .color(NamedColor::Green)
            .build(),
        false,
    );
}

/// Step count is bounded so a typo cannot queue a runaway number of steps.
type StepCount = Integer<1, 10_000>;

#[command("tick step")]
fn tick_step(
    #[sender] sender: Sender,
    #[arg] count: StepCount,
    mut control: ResMut<FluidTickControl>,
) {
    // Stepping implies frozen: if the user steps without freezing first, freeze now so the steps
    // are meaningful rather than racing the normal clock.
    control.frozen = true;
    let n = (*count).max(0) as u32;
    control.steps = control.steps.saturating_add(n);
    sender.send_message(
        TextComponentBuilder::new(format!(
            "Queued {n} fluid step(s); {} pending.",
            control.steps
        ))
        .color(NamedColor::Aqua)
        .build(),
        false,
    );
}
