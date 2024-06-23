use ferrumc_utils::error::Error;
use crate::Connection;

pub mod incoming;
pub mod outgoing;

pub trait IncomingPacket {
    async fn handle(&self, conn: &mut Connection) -> Result<Option<Vec<u8>>, Error>;
}