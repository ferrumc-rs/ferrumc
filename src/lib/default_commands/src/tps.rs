use bevy_ecs::system::ResMut;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_performance::{memory::MemoryUnit, ServerPerformance};
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

#[command("tps")]
fn tps_command(#[sender] sender: Sender, performance_res: ResMut<ServerPerformance>) {
    let performance = performance_res.into_inner();

    let tps = &performance.tps;
    let (current, peak) = performance.memory.get_memory(MemoryUnit::Megabytes);

    sender.send_message(
        TextComponentBuilder::new("Server Performance\n")
            .color(NamedColor::Gray)
            // TPS section
            .extra(
                TextComponentBuilder::new("TPS ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps_1s()))
            .extra(
                TextComponentBuilder::new("| ")
                    .color(NamedColor::DarkGray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps_5s()))
            .extra(
                TextComponentBuilder::new("| ")
                    .color(NamedColor::DarkGray)
                    .build(),
            )
            .extra(get_tps_component(tps.tps_15s()))
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
            .extra(get_percentile_component(Some(tps.avg_tick_ms())))
            .extra(
                TextComponentBuilder::new("p50 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.p50_ms()))
            .extra(
                TextComponentBuilder::new("p95 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.p95_ms()))
            .extra(
                TextComponentBuilder::new("p99 ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(get_percentile_component(tps.p99_ms()))
            // Memory section
            .extra(TextComponent::from("\n\n"))
            .extra(
                TextComponentBuilder::new("Memory (MB)\n")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new("used ")
                    .color(NamedColor::Gray)
                    .build(),
            )
            .extra(
                TextComponentBuilder::new(format!("{}MB", current))
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
                TextComponentBuilder::new(format!("{}MB", peak))
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

fn get_percentile_component(percentile: Option<f64>) -> TextComponent {
    match percentile {
        Some(ms) => {
            let color = if ms > 100.0 {
                NamedColor::Red
            } else if ms > 50.0 {
                NamedColor::Yellow
            } else {
                NamedColor::Green
            };

            TextComponentBuilder::new(format!("{:.2}ms ", ms))
                .color(color)
                .build()
        }
        None => TextComponentBuilder::new("0.00ms ")
            .color(NamedColor::Green)
            .build(),
    }
}
