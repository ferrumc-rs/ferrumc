#[cfg(test)]
mod test {
    use ferrumc_macros::block;
    #[derive(Debug, PartialEq, Eq)]
    struct BlockId(u32);
    #[cfg(false)]
    include!(concat!(env!("OUT_DIR"), "/block_test.rs"));
}
