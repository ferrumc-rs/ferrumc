use crate::codec::encode::{NetEncode, NetEncodeOpts, errors::NetEncodeError};
use ferrumc_text::TextComponent;
use std::io::Write;
use tokio::io::AsyncWrite;

impl NetEncode for TextComponent {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        // We can call the method on the struct because it returns standard Vec<u8>
        let nbt_bytes = self.serialize_nbt();
        writer.write_all(&nbt_bytes).map_err(NetEncodeError::Io)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        use tokio::io::AsyncWriteExt;

        let nbt_bytes = self.serialize_nbt();
        writer
            .write_all(&nbt_bytes)
            .await
            .map_err(NetEncodeError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrumc_macros::NetEncode;
    use ferrumc_text::{ComponentBuilder, NamedColor};
    use std::io::Cursor;

    #[derive(NetEncode)]
    struct TestPacket {
        message: TextComponent,
        overlay: bool,
    }

    #[test]
    fn test_serialize_to_nbt() {
        // Reconstruct your complex component here
        let component = ComponentBuilder::translate(
            "chat.type.text",
            vec![
                ComponentBuilder::text("GStudiosX")
                    .color(NamedColor::Blue)
                    .build(),
                ComponentBuilder::text("Hi")
                    .extra(ComponentBuilder::keybind("key.jump"))
                    .build(),
            ],
        );

        let mut cursor = Cursor::new(Vec::new());

        // Test the encoding logic
        TestPacket::encode(
            &TestPacket {
                message: component,
                overlay: false,
            },
            &mut cursor,
            &NetEncodeOpts::WithLength,
        )
        .unwrap();

        let bytes = cursor.into_inner();
        println!("Encoded Bytes: {:?}", bytes);

        // Add assertions here to verify the bytes contain
        // the NBT Root Tag (0x0A) and the empty name (0x00 0x00).
        // This confirms your Protocol crate handles the text format correctly.
    }
}
