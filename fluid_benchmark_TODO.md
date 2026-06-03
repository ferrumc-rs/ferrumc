# 流体性能测试与火焰图 TODO

记录流体模拟性能工作的后续计划。当前已落地的是 `src/lib/world/src/fluid/tests_integration.rs`
里的 tick 计时压测（`tick_timing_*`），它能量化单 tick 耗时并暴露超时，但**只测量、不优化**。

## 背景：已确认的问题

tick 计时测试已用数据证实流体扩散的 per-tick 成本会超出服务器预算：

- 服务器预算：`50ms`（20 TPS，见 `game_loop.rs` 的 `tick` schedule）。
- 实测 `tick_timing_many_simultaneous_sources_bounded`（169 个水源同时铺开）：
  4 个 tick 超 50ms，最坏 ~246ms（约预算的 5 倍）。
- 与生产日志吻合：放水后出现 `Schedule 'tick' overran: took 268ms`。

现有压测的硬断言**故意设得很宽松**（单 tick < 20×预算 = 1s），只拦截灾难性回归，
避免因机器快慢导致 CI 抖动。精细信号靠 `--nocapture` 打印的报告。

查看报告：
```
cargo test -p ferrumc-world --lib fluid::tests_integration::tick_timing -- --nocapture
```

## 计划一：引入 criterion 基准测试

目标：把"一次性 wall-clock 打印"升级为可回归对比的统计基准。

- [ ] 在 `ferrumc-world` 的 `Cargo.toml` 加 `criterion` dev-dependency 与 `[[bench]]` 项
      （已有 `src/lib/world/src/benches/` 目录，参考 `edit_bench.rs` / `world.rs` 的现有约定）。
- [ ] 新增 `fluid_bench.rs`，复用 tests_integration 里的世界构造思路，对以下场景建立基准：
  - 单源平面扩散（front 扩张成本）。
  - N 源同时铺开（批量 tick 成本，N 取 16 / 64 / 169 / 512 做规模曲线）。
  - 阶梯瀑布（复杂地形级联）。
  - 排水洞转向（重复坡度搜索成本）。
- [ ] 对比串行 vs 并行路径：`PARALLEL_THRESHOLD = 64`（见 `src/bin/src/systems/fluids/mod.rs`），
      基准要同时覆盖阈值上下两侧，确认并行确实有收益而非纯开销。
- [ ] 记录基线数字，后续优化以 `criterion` 的 baseline 对比验证提升。

## 计划二：火焰图生成流程

目标：定位那 ~246ms 到底花在哪。候选热点（需用数据证实，勿凭猜测）：

- `slope_distance` 的 BFS 每次调用都新建 `HashSet` / `VecDeque`（`vanilla.rs`），高频分配。
- 同一 tick 内对相邻 cell 重复跑坡度搜索的冗余。
- `compute_tick` 的 per-cell 开销 × 宽 front。

工具链（Linux，本机为 Intel Ultra 7 155H）：

- [ ] 安装 `cargo-flamegraph`（`cargo install flamegraph`），依赖 `perf`
      （`sudo apt install linux-perf` 或对应发行版包；可能需要
      `echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid`）。
- [ ] 为火焰图准备一个可执行的压测入口。两种方式择一：
  - 临时 `#[ignore]` 的长循环测试（如 N=512 源跑数千 tick），用
    `cargo flamegraph --test ...` 采样；或
  - 一个 `examples/fluid_stress.rs` 独立二进制，便于 `cargo flamegraph --example fluid_stress`。
- [ ] 用 `--release` 采样（debug 的分配/边界检查会扭曲热点分布）。
- [ ] 产出 `flamegraph.svg`，归档到本文件旁或 `.etc/` 下，标注采样时的场景与 commit。

## 计划三：基于数据的优化方向（待火焰图确认后再动手）

> 原则：先测后改，每项优化都用计划一的基准验证收益，避免凭直觉改。

- [ ] `slope_distance`：复用线程局部的 `HashSet`/`VecDeque` 缓冲，或改用定长 visited 位图
      （搜索半径有界 `slope_find_distance`，可用小固定数组替代哈希集）。
- [ ] 批内去重 / 缓存：同一 tick 内对相同 cell 的坡度搜索结果可复用。
- [ ] 复核并行路径在大批次下的实际线程利用率与争用情况。

## 注意事项

- 这些是性能工作，**不是发布阻塞项**。当前流体功能正确性已由测试守护。
- 时间断言天然受机器影响，criterion 基准应在相对稳定的环境跑，关注趋势而非绝对值。
