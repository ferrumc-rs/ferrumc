#[cfg(test)]
mod test {
    use ferrumc_macros::block;
    #[derive(Debug, PartialEq, Eq)]
    struct BlockStateId(u32);
    #[cfg(false)]
    include!(concat!(env!("OUT_DIR"), "/block_test.rs"));
    #[test]
    fn simple() {
        assert_eq!(block!("deepslate", { axis: "x" }), BlockStateId(25964));
        assert_eq!(block!("deepslate", { axis: "y" }), BlockStateId(25965));
        assert_eq!(block!("deepslate", { axis: "z" }), BlockStateId(25966));
        assert_eq!(
            block!(
                "big_dripleaf",
                {
                    facing: "north",
                    tilt: "full",
                    waterlogged: true
                }
            ),
            BlockStateId(25910)
        );
    }
}
