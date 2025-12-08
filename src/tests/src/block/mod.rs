#[cfg(test)]
mod test {
    use ferrumc_macros::block;
    #[expect(unused_imports)]
    #[derive(Debug, PartialEq, Eq)]
    struct BlockStateId(u32);
    #[cfg(false)]
    include!(concat!(env!("OUT_DIR"), "/block_test.rs"));
}
