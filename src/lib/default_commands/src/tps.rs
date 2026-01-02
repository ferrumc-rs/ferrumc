use std::time::Duration;

use bevy_ecs::system::ResMut;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_performance::{memory::MemoryUnit, ServerPerformance};
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

#[command("tps")]
fn tps_command(#[sender] sender: Sender, performance_res: ResMut<ServerPerformance>) {
    let performance = performance_res.into_inner();

    let tps = &performance.tps;
    let (current, peak) = performance.memory.get_memory(MemoryUnit::Kilobytes);

    sender.send_message(
        TextComponentBuilder::new("Server Performance\n")
            .color(NamedColor::Gray)
            // TPS section
            .extra(
                TextComponentBuilder::new("TPS ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps(Duration::from_secs(1))))
            .extra(
                TextComponentBuilder::new("| ")
                    .color(NamedColor::DarkGray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps(Duration::from_secs(5))))
            .extra(
                TextComponentBuilder::new("| ")
                    .color(NamedColor::DarkGray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps(Duration::from_secs(15))))
            .extra(
                TextComponentBuilder::new(" (1s / 5s / 15s)\n\n")
                    .color(NamedColor::Gray)
                    .build(),
            )
            // Tick duration section
            .extra(
                TextComponentBuilder::new("Tick Time (ms)\n")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new("avg ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.avg_tick_ms()))
            .extra(
                TextComponentBuilder::new("p50 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.tick_duration(0.50)))
            .extra(
                TextComponentBuilder::new("p95 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.tick_duration(0.95)))
            .extra(
                TextComponentBuilder::new("p99 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.tick_duration(0.99)))
            // Memory section
            .extra(TextComponent::from("\n\n"))
            .extra(
                TextComponentBuilder::new("Memory (MiB)\n")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new("used ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new(format!("{:.2}MiB", get_mib(current)))
                    .color(NamedColor::White)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new(" | ")
                    .color(NamedColor::DarkGray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new("peak ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new(format!("{:.2}MiB", get_mib(peak)))
                    .color(NamedColor::White)
                    .build(),
            )
            .build(),
        false,
    );
}

fn get_tps_component(tps: f32) -> TextComponent {
    let color = if tps < 14.0 {
        NamedColor::Red
    } else if tps < 16.0 {
        NamedColor::Yellow
    } else {
        NamedColor::Green
    };

    TextComponentBuilder::new(format!("{:.2} ", tps))
        .color(color)
        .build()
}

fn get_percentile_component(percentile: f64) -> TextComponent {
    let color = if percentile > 100.0 {
        NamedColor::Red
    } else if percentile > 50.0 {
        NamedColor::Yellow
    } else {
        NamedColor::Green
    };

    TextComponentBuilder::new(format!("{:.2}ms ", percentile))
        .color(color)
        .build()
}

fn get_mib(kb: u64) -> f64 {
    (kb as f64) / 1024.0
}
