//! Integration tests for the shard's non-player entity store and its per-tick
//! physics.
//!
//! These exercise the store and the gravity/integration/drag step through the
//! public API only, focusing on the property the crate's determinism invariant
//! demands: the same sequence of inputs produces identical observable state and
//! outputs on any run, independent of shard position.

use ferrumc_core::EntityId;
use ferrumc_math::{ShardPos, Vec3};
use ferrumc_sim::{GameInput, GameOutput, SimShard, GRAVITY_ITEM};

/// A scripted store operation applied to a shard (no ticking; pure store use).
#[derive(Clone, Copy)]
enum Op {
    Spawn { position: Vec3, velocity: Vec3 },
    Remove(EntityId),
}

/// Applies `ops` to a fresh shard at `shard_pos` and returns it. Spawns use zero
/// gravity because these tests never tick: they assert store mechanics (ids,
/// membership, order), not motion.
///
/// Spawns must succeed: the scripts here are far below the id-exhaustion
/// ceiling, so a failure would be a bug in the store, not the test.
fn run_at(shard_pos: ShardPos, ops: &[Op]) -> SimShard {
    let mut shard = SimShard::new(shard_pos);
    for op in ops {
        match *op {
            Op::Spawn { position, velocity } => {
                shard
                    .spawn_entity(position, velocity, 0.0)
                    .expect("id available");
            }
            Op::Remove(id) => {
                shard.remove_entity(id);
            }
        }
    }
    shard
}

/// Applies `ops` to a fresh shard at a fixed position.
fn run(ops: &[Op]) -> SimShard {
    run_at(ShardPos::new(1, -1), ops)
}

/// A full observable snapshot of a shard's entity store: for every entity, in
/// ascending id order, its id and physical fields. Two shards with equal
/// snapshots are indistinguishable through the public store API.
fn snapshot(shard: &SimShard) -> Vec<(i32, Vec3, Vec3, bool)> {
    shard
        .entity_ids()
        .map(|id| {
            (
                id.get(),
                shard.entity_position(id).expect("present"),
                shard.entity_velocity(id).expect("present"),
                shard.entity_on_ground(id).expect("present"),
            )
        })
        .collect()
}

/// Spawn three entities, remove the middle one, then spawn two more. Ids `1..=5`
/// are issued in order; id `2` is removed and must never be reused.
fn sample_ops() -> Vec<Op> {
    vec![
        Op::Spawn {
            position: Vec3::new(0.0, 64.0, 0.0),
            velocity: Vec3::ZERO,
        },
        Op::Spawn {
            position: Vec3::new(1.0, 70.0, 2.0),
            velocity: Vec3::new(0.0, -0.04, 0.0),
        },
        Op::Spawn {
            position: Vec3::new(-5.0, 64.0, 5.0),
            velocity: Vec3::ZERO,
        },
        Op::Remove(EntityId::new(2)),
        Op::Spawn {
            position: Vec3::new(3.0, 80.0, 3.0),
            velocity: Vec3::new(0.1, 0.0, -0.1),
        },
        Op::Spawn {
            position: Vec3::new(8.0, 64.0, 8.0),
            velocity: Vec3::ZERO,
        },
    ]
}

#[test]
fn same_op_sequence_yields_identical_store() {
    let ops = sample_ops();
    let first = snapshot(&run(&ops));
    let second = snapshot(&run(&ops));
    assert_eq!(
        first, second,
        "identical op sequences must produce identical stores"
    );
}

#[test]
fn snapshot_reflects_the_scripted_operations() {
    let shard = run(&sample_ops());
    // Five spawns, one removal -> four resident entities.
    assert_eq!(shard.entity_count(), 4);
    // Id 2 was removed and never reissued; ids stay in ascending order.
    let ids: Vec<i32> = shard.entity_ids().map(EntityId::get).collect();
    assert_eq!(ids, vec![1, 3, 4, 5]);
    // Spot-check a preserved field: entity 4 kept its spawn velocity.
    assert_eq!(
        shard.entity_velocity(EntityId::new(4)),
        Some(Vec3::new(0.1, 0.0, -0.1))
    );
}

/// Drives `inputs` through a fresh shard for one tick and returns its outputs.
fn run_inputs(inputs: &[GameInput]) -> Vec<GameOutput> {
    let mut shard = SimShard::new(ShardPos::new(0, 0));
    for input in inputs {
        shard.enqueue(input.clone()).expect("inbox has room");
    }
    shard.run_tick()
}

#[test]
fn spawn_and_despawn_inputs_produce_matching_outputs() {
    // Zero gravity + zero velocity keep this focused on the spawn/despawn
    // boundary (no follow-up EntityMoved from the physics step this tick).
    let inputs = vec![
        GameInput::SpawnEntity {
            position: Vec3::new(0.0, 64.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: 0.0,
        },
        GameInput::SpawnEntity {
            position: Vec3::new(1.0, 64.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: 0.0,
        },
        GameInput::DespawnEntity {
            entity: EntityId::new(1),
        },
    ];
    let outputs = run_inputs(&inputs);
    assert_eq!(outputs.len(), 3);
    // Spawns first (FIFO), each with an ascending id.
    let GameOutput::EntitySpawned { entity: a, .. } = outputs[0] else {
        panic!("expected spawn, got {:?}", outputs[0]);
    };
    let GameOutput::EntitySpawned { entity: b, .. } = outputs[1] else {
        panic!("expected spawn, got {:?}", outputs[1]);
    };
    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 2);
    // Then the despawn of id 1.
    assert_eq!(
        outputs[2],
        GameOutput::EntityDespawned {
            entity: EntityId::new(1),
        }
    );
}

#[test]
fn identical_spawn_sequences_produce_identical_outputs() {
    // Nonzero gravity so the physics step runs; determinism must still hold.
    let inputs = vec![
        GameInput::SpawnEntity {
            position: Vec3::new(0.0, 64.0, 0.0),
            velocity: Vec3::ZERO,
            gravity: GRAVITY_ITEM,
        },
        GameInput::SpawnEntity {
            position: Vec3::new(3.0, 70.0, -1.0),
            velocity: Vec3::new(0.1, 0.0, 0.1),
            gravity: GRAVITY_ITEM,
        },
        GameInput::DespawnEntity {
            entity: EntityId::new(1),
        },
        GameInput::SpawnEntity {
            position: Vec3::new(5.0, 64.0, 5.0),
            velocity: Vec3::ZERO,
            gravity: GRAVITY_ITEM,
        },
    ];
    let first = run_inputs(&inputs);
    let second = run_inputs(&inputs);
    assert_eq!(first, second);
}

#[test]
fn falling_entity_is_bit_deterministic_across_ticks() {
    // Two shards, same falling spawn, same number of empty ticks. Gravity and
    // drag are f64 arithmetic in a fixed order, so positions after N ticks must
    // match bit-for-bit.
    fn drive(ticks: usize) -> Vec<GameOutput> {
        let mut shard = SimShard::new(ShardPos::new(0, 0));
        shard
            .enqueue(GameInput::SpawnEntity {
                position: Vec3::new(1.5, 128.0, -0.25),
                velocity: Vec3::new(0.0, -0.125, 0.0),
                gravity: GRAVITY_ITEM,
            })
            .expect("room");
        let mut all_outputs = shard.run_tick();
        for _ in 1..ticks {
            all_outputs.extend(shard.run_tick());
        }
        all_outputs
    }
    assert_eq!(drive(5), drive(5));
}

#[test]
fn id_allocation_is_independent_of_shard_position() {
    // The id counter is per-shard and never reads shard coordinates, so two
    // shards at different positions allocate identical ids for the same ops.
    let ops = sample_ops();
    let here = run_at(ShardPos::new(0, 0), &ops);
    let far = run_at(ShardPos::new(100, -100), &ops);
    assert_eq!(snapshot(&here), snapshot(&far));
}
