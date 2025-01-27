use azalea_buf::AzBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket, PartialEq)]
pub struct ClientboundContainerSetContent {
    #[var]
    pub container_id: i32,
    #[var]
    pub state_id: u32,
    pub items: Vec<ItemStack>,
    pub carried_item: ItemStack,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzaleaRead;

    use super::ClientboundContainerSetContent;

    #[test]
    fn test_read_write_container_set_content() {
        let contents = [
            1, 2, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 196, 6, 0, 0, 0,
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundContainerSetContent::azalea_read(&mut buf).unwrap();
        println!("{:?}", packet);

        assert_eq!(buf.position(), contents.len() as u64);

        let mut buf = Vec::new();
        packet.write(&mut buf).unwrap();
        assert_eq!(buf, contents);
    }
}
