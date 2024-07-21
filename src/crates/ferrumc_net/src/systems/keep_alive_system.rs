use std::any::Any;
use ferrumc_macros::AutoGenName;
use crate::systems::System;

#[derive(AutoGenName)]
struct KeepAliveSystem;

impl System for KeepAliveSystem {
    async fn run(&self) {
        loop {

        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}


