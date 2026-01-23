#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SignBlockType {
    AcaciaSign,
    BambooSign,
    BirchSign,
    CherrySign,
    CrimsonSign,
    DarkOakSign,
    JungleSign,
    MangroveSign,
    OakSign,
    PaleOakSign,
    SpruceSign,
    WarpedSign,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SignBlock {
    pub block_type: SignBlockType,
    pub rotation: i32,
    pub waterlogged: bool,
}
impl SignBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<SignBlock>();
}
impl TryFrom<u32> for SignBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            4462u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4463u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4464u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4465u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4466u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4467u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4468u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4469u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4470u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4471u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4472u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4473u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4474u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4475u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4476u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4477u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4478u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4479u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4480u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4481u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4482u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4483u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4484u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4485u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4486u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4487u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4488u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4489u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4490u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4491u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4492u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4493u32 => Ok(SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4654u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4655u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4656u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4657u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4658u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4659u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4660u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4661u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4662u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4663u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4664u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4665u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4666u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4667u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4668u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4669u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4670u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4671u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4672u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4673u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4674u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4675u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4676u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4677u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4678u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4679u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4680u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4681u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4682u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4683u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4684u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4685u32 => Ok(SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4430u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4431u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4432u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4433u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4434u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4435u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4436u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4437u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4438u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4439u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4440u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4441u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4442u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4443u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4444u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4445u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4446u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4447u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4448u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4449u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4450u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4451u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4452u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4453u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4454u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4455u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4456u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4457u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4458u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4459u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4460u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4461u32 => Ok(SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4494u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4495u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4496u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4497u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4498u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4499u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4500u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4501u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4502u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4503u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4504u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4505u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4506u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4507u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4508u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4509u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4510u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4511u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4512u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4513u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4514u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4515u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4516u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4517u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4518u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4519u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4520u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4521u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4522u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4523u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4524u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4525u32 => Ok(SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 15i32,
                waterlogged: false,
            }),
            20299u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            20300u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            20301u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            20302u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            20303u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            20304u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            20305u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            20306u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            20307u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            20308u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            20309u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            20310u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            20311u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            20312u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            20313u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            20314u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            20315u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            20316u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            20317u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            20318u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            20319u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            20320u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            20321u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            20322u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            20323u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            20324u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            20325u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            20326u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            20327u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            20328u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            20329u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            20330u32 => Ok(SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4558u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4559u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4560u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4561u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4562u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4563u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4564u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4565u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4566u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4567u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4568u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4569u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4570u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4571u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4572u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4573u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4574u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4575u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4576u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4577u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4578u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4579u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4580u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4581u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4582u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4583u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4584u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4585u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4586u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4587u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4588u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4589u32 => Ok(SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4526u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4527u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4528u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4529u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4530u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4531u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4532u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4533u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4534u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4535u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4536u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4537u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4538u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4539u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4540u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4541u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4542u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4543u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4544u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4545u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4546u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4547u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4548u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4549u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4550u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4551u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4552u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4553u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4554u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4555u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4556u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4557u32 => Ok(SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4622u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4623u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4624u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4625u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4626u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4627u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4628u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4629u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4630u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4631u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4632u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4633u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4634u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4635u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4636u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4637u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4638u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4639u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4640u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4641u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4642u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4643u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4644u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4645u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4646u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4647u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4648u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4649u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4650u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4651u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4652u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4653u32 => Ok(SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4366u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4367u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4368u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4369u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4370u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4371u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4372u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4373u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4374u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4375u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4376u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4377u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4378u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4379u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4380u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4381u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4382u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4383u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4384u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4385u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4386u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4387u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4388u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4389u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4390u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4391u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4392u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4393u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4394u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4395u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4396u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4397u32 => Ok(SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4590u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4591u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4592u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4593u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4594u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4595u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4596u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4597u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4598u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4599u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4600u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4601u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4602u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4603u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4604u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4605u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4606u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4607u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4608u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4609u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4610u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4611u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4612u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4613u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4614u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4615u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4616u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4617u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4618u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4619u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4620u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4621u32 => Ok(SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            4398u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            4399u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            4400u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            4401u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            4402u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            4403u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            4404u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            4405u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            4406u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            4407u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            4408u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            4409u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            4410u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            4411u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            4412u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            4413u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            4414u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            4415u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            4416u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            4417u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            4418u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            4419u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            4420u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            4421u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            4422u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            4423u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            4424u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            4425u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            4426u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            4427u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            4428u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            4429u32 => Ok(SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            20331u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 0i32,
                waterlogged: true,
            }),
            20332u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 0i32,
                waterlogged: false,
            }),
            20333u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 1i32,
                waterlogged: true,
            }),
            20334u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 1i32,
                waterlogged: false,
            }),
            20335u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 2i32,
                waterlogged: true,
            }),
            20336u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 2i32,
                waterlogged: false,
            }),
            20337u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 3i32,
                waterlogged: true,
            }),
            20338u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 3i32,
                waterlogged: false,
            }),
            20339u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 4i32,
                waterlogged: true,
            }),
            20340u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 4i32,
                waterlogged: false,
            }),
            20341u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 5i32,
                waterlogged: true,
            }),
            20342u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 5i32,
                waterlogged: false,
            }),
            20343u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 6i32,
                waterlogged: true,
            }),
            20344u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 6i32,
                waterlogged: false,
            }),
            20345u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 7i32,
                waterlogged: true,
            }),
            20346u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 7i32,
                waterlogged: false,
            }),
            20347u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 8i32,
                waterlogged: true,
            }),
            20348u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 8i32,
                waterlogged: false,
            }),
            20349u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 9i32,
                waterlogged: true,
            }),
            20350u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 9i32,
                waterlogged: false,
            }),
            20351u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 10i32,
                waterlogged: true,
            }),
            20352u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 10i32,
                waterlogged: false,
            }),
            20353u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 11i32,
                waterlogged: true,
            }),
            20354u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 11i32,
                waterlogged: false,
            }),
            20355u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 12i32,
                waterlogged: true,
            }),
            20356u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 12i32,
                waterlogged: false,
            }),
            20357u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 13i32,
                waterlogged: true,
            }),
            20358u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 13i32,
                waterlogged: false,
            }),
            20359u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 14i32,
                waterlogged: true,
            }),
            20360u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 14i32,
                waterlogged: false,
            }),
            20361u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 15i32,
                waterlogged: true,
            }),
            20362u32 => Ok(SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 15i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SignBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4462u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4463u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4464u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4465u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4466u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4467u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4468u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4469u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4470u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4471u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4472u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4473u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4474u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4475u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4476u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4477u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4478u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4479u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4480u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4481u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4482u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4483u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4484u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4485u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4486u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4487u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4488u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4489u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4490u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4491u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4492u32),
            SignBlock {
                block_type: SignBlockType::AcaciaSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4493u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4654u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4655u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4656u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4657u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4658u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4659u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4660u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4661u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4662u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4663u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4664u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4665u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4666u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4667u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4668u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4669u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4670u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4671u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4672u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4673u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4674u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4675u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4676u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4677u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4678u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4679u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4680u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4681u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4682u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4683u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4684u32),
            SignBlock {
                block_type: SignBlockType::BambooSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4685u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4430u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4431u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4432u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4433u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4434u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4435u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4436u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4437u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4438u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4439u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4440u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4441u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4442u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4443u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4444u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4445u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4446u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4447u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4448u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4449u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4450u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4451u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4452u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4453u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4454u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4455u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4456u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4457u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4458u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4459u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4460u32),
            SignBlock {
                block_type: SignBlockType::BirchSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4461u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4494u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4495u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4496u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4497u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4498u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4499u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4500u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4501u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4502u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4503u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4504u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4505u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4506u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4507u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4508u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4509u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4510u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4511u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4512u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4513u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4514u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4515u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4516u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4517u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4518u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4519u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4520u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4521u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4522u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4523u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4524u32),
            SignBlock {
                block_type: SignBlockType::CherrySign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4525u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(20299u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(20300u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(20301u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(20302u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(20303u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(20304u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(20305u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(20306u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(20307u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(20308u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(20309u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(20310u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(20311u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(20312u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(20313u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(20314u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(20315u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(20316u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(20317u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(20318u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(20319u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(20320u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(20321u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(20322u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(20323u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(20324u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(20325u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(20326u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(20327u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(20328u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(20329u32),
            SignBlock {
                block_type: SignBlockType::CrimsonSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(20330u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4558u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4559u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4560u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4561u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4562u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4563u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4564u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4565u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4566u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4567u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4568u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4569u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4570u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4571u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4572u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4573u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4574u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4575u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4576u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4577u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4578u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4579u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4580u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4581u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4582u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4583u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4584u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4585u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4586u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4587u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4588u32),
            SignBlock {
                block_type: SignBlockType::DarkOakSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4589u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4526u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4527u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4528u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4529u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4530u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4531u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4532u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4533u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4534u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4535u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4536u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4537u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4538u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4539u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4540u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4541u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4542u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4543u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4544u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4545u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4546u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4547u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4548u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4549u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4550u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4551u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4552u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4553u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4554u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4555u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4556u32),
            SignBlock {
                block_type: SignBlockType::JungleSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4557u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4622u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4623u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4624u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4625u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4626u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4627u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4628u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4629u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4630u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4631u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4632u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4633u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4634u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4635u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4636u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4637u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4638u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4639u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4640u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4641u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4642u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4643u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4644u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4645u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4646u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4647u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4648u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4649u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4650u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4651u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4652u32),
            SignBlock {
                block_type: SignBlockType::MangroveSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4653u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4366u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4367u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4368u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4369u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4370u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4371u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4372u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4373u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4374u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4375u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4376u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4377u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4378u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4379u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4380u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4381u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4382u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4383u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4384u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4385u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4386u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4387u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4388u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4389u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4390u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4391u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4392u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4393u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4394u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4395u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4396u32),
            SignBlock {
                block_type: SignBlockType::OakSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4397u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4590u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4591u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4592u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4593u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4594u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4595u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4596u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4597u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4598u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4599u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4600u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4601u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4602u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4603u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4604u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4605u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4606u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4607u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4608u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4609u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4610u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4611u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4612u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4613u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4614u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4615u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4616u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4617u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4618u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4619u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4620u32),
            SignBlock {
                block_type: SignBlockType::PaleOakSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4621u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(4398u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(4399u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(4400u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(4401u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(4402u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(4403u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(4404u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(4405u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(4406u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(4407u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(4408u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(4409u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(4410u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(4411u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(4412u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(4413u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(4414u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(4415u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(4416u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(4417u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(4418u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(4419u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(4420u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(4421u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(4422u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(4423u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(4424u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(4425u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(4426u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(4427u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(4428u32),
            SignBlock {
                block_type: SignBlockType::SpruceSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(4429u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 0i32,
                waterlogged: true,
            } => Ok(20331u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 0i32,
                waterlogged: false,
            } => Ok(20332u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 1i32,
                waterlogged: true,
            } => Ok(20333u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 1i32,
                waterlogged: false,
            } => Ok(20334u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 2i32,
                waterlogged: true,
            } => Ok(20335u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 2i32,
                waterlogged: false,
            } => Ok(20336u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 3i32,
                waterlogged: true,
            } => Ok(20337u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 3i32,
                waterlogged: false,
            } => Ok(20338u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 4i32,
                waterlogged: true,
            } => Ok(20339u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 4i32,
                waterlogged: false,
            } => Ok(20340u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 5i32,
                waterlogged: true,
            } => Ok(20341u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 5i32,
                waterlogged: false,
            } => Ok(20342u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 6i32,
                waterlogged: true,
            } => Ok(20343u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 6i32,
                waterlogged: false,
            } => Ok(20344u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 7i32,
                waterlogged: true,
            } => Ok(20345u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 7i32,
                waterlogged: false,
            } => Ok(20346u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 8i32,
                waterlogged: true,
            } => Ok(20347u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 8i32,
                waterlogged: false,
            } => Ok(20348u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 9i32,
                waterlogged: true,
            } => Ok(20349u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 9i32,
                waterlogged: false,
            } => Ok(20350u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 10i32,
                waterlogged: true,
            } => Ok(20351u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 10i32,
                waterlogged: false,
            } => Ok(20352u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 11i32,
                waterlogged: true,
            } => Ok(20353u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 11i32,
                waterlogged: false,
            } => Ok(20354u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 12i32,
                waterlogged: true,
            } => Ok(20355u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 12i32,
                waterlogged: false,
            } => Ok(20356u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 13i32,
                waterlogged: true,
            } => Ok(20357u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 13i32,
                waterlogged: false,
            } => Ok(20358u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 14i32,
                waterlogged: true,
            } => Ok(20359u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 14i32,
                waterlogged: false,
            } => Ok(20360u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 15i32,
                waterlogged: true,
            } => Ok(20361u32),
            SignBlock {
                block_type: SignBlockType::WarpedSign,
                rotation: 15i32,
                waterlogged: false,
            } => Ok(20362u32),
            _ => Err(()),
        }
    }
}
