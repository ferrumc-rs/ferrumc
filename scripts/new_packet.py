import os.path
import re

incoming_template = """
use crate::packets::IncomingPacket;

use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "++id++", state = "play")]
pub struct ++name++ {
}

impl IncomingPacket for ++name++ {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        todo!()
    }
}
"""

outgoing_template = """
use ferrumc_macros::{packet, NetEncode};\
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "++id++", state="play")]
pub struct ++name++ {}
"""


def to_snake_case(string) -> str:
    return string.lower().replace(" ", "_")


def to_camel_case(string) -> str:
    return string.title().replace(" ", "")


packet_type_input = input("Incoming or outgoing packet? (i/o): ")
packet_type = ""
if packet_type_input == "i":
    packet_type = "incoming"
elif packet_type_input == "o":
    packet_type = "outgoing"
else:
    print("Invalid input")
    exit()

packet_name = input("Packet name: ")
packets_dir = os.path.join(
    os.path.join(os.path.dirname(__file__), ".."), "src/lib/net/src/packets"
)

packet_id = input(
    "Packet ID (formatted as snake case,\
look on https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol if you need to get the id): "
)

with open(f"{packets_dir}/{packet_type}/{to_snake_case(packet_name)}.rs", "x") as f:
    if packet_type == "incoming":
        f.write(
            incoming_template.replace("++name++", to_camel_case(packet_name)).replace(
                "++id++", packet_id
            )
        )
        with open(f"{packets_dir}/incoming/mod.rs", "a") as modfile:
            modfile.write(f"\npub mod {to_snake_case(packet_name)};")

        with open(f"{packets_dir}/mod.rs", "r+") as registry:
            content = registry.read()
            content = re.sub(
                "pub enum Packet {\n",
                "pub enum Packet {\n    "
                + f"{to_camel_case(packet_name)}({to_snake_case(packet_name)}::{to_camel_case(packet_name)}),\n",
                content,
                flags=re.M,
            )
            registry.seek(0)
            registry.write(content)
    else:
        f.write(
            outgoing_template.replace("++name++", to_camel_case(packet_name)).replace(
                "++id++", packet_id
            )
        )
        with open(f"{packets_dir}/outgoing/mod.rs", "a") as modfile:
            modfile.write(f"\npub mod {to_snake_case(packet_name)};")
