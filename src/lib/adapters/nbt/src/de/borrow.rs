use crate::de::converter::FromNbt;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use std::io::Write;
use crate::{NBTSerializable, NBTSerializeOptions};
use ferrumc_general_purpose::simd::arrays;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum NbtTag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl From<u8> for NbtTag {
    fn from(tag: u8) -> Self {
        match tag {
            0 => NbtTag::End,
            1 => NbtTag::Byte,
            2 => NbtTag::Short,
            3 => NbtTag::Int,
            4 => NbtTag::Long,
            5 => NbtTag::Float,
            6 => NbtTag::Double,
            7 => NbtTag::ByteArray,
            8 => NbtTag::String,
            9 => NbtTag::List,
            10 => NbtTag::Compound,
            11 => NbtTag::IntArray,
            12 => NbtTag::LongArray,
            _ => panic!("Invalid NbtTag: {}", tag),
        }
    }
}

#[derive(Debug)]
pub enum NbtTapeElement<'a> {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [i8]),
    String(&'a str),
    List {
        el_type: NbtTag,
        size: usize,
        elements_pos: usize,
    },
    // For better cache locality. It's way faster than hashmaps.
    // Although lookups are O(n), the n is very small.
    // But the data is big so it's worth it.
    Compound(Vec<(&'a str, NbtTapeElement<'a>)>),
    // This is owned data because of SIMD
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTapeElement<'_> {
    pub const fn nbt_type(&self) -> &'static str {
        match self {
            NbtTapeElement::End => "End",
            NbtTapeElement::Byte(_) => "Byte",
            NbtTapeElement::Short(_) => "Short",
            NbtTapeElement::Int(_) => "Int",
            NbtTapeElement::Long(_) => "Long",
            NbtTapeElement::Float(_) => "Float",
            NbtTapeElement::Double(_) => "Double",
            NbtTapeElement::ByteArray(_) => "ByteArray",
            NbtTapeElement::String(_) => "String",
            NbtTapeElement::List { .. } => "List",
            NbtTapeElement::Compound(_) => "Compound",
            NbtTapeElement::IntArray(_) => "IntArray",
            NbtTapeElement::LongArray(_) => "LongArray",
        }
    }
    pub const fn nbt_id(&self) -> u8 {
        match self {
            NbtTapeElement::End => NbtTag::End as u8,
            NbtTapeElement::Byte(_) => NbtTag::Byte as u8,
            NbtTapeElement::Short(_) => NbtTag::Short as u8,
            NbtTapeElement::Int(_) => NbtTag::Int as u8,
            NbtTapeElement::Long(_) => NbtTag::Long as u8,
            NbtTapeElement::Float(_) => NbtTag::Float as u8,
            NbtTapeElement::Double(_) => NbtTag::Double as u8,
            NbtTapeElement::ByteArray(_) => NbtTag::ByteArray as u8,
            NbtTapeElement::String(_) => NbtTag::String as u8,
            NbtTapeElement::List { .. } => NbtTag::List as u8,
            NbtTapeElement::Compound(_) => NbtTag::Compound as u8,
            NbtTapeElement::IntArray(_) => NbtTag::IntArray as u8,
            NbtTapeElement::LongArray(_) => NbtTag::LongArray as u8,
        }
    }
}

pub struct NbtTape<'a> {
    data: &'a [u8],
    pos: usize,
    depth: usize,
    // => The root tag is always a compound tag.
    pub root: Option<(&'a str, NbtTapeElement<'a>)>,
}

impl<'a> NbtTapeElement<'a> {
    pub fn get(&self, key: &str) -> Option<&NbtTapeElement<'a>> {
        match self {
            NbtTapeElement::Compound(elements) => {
                for (name, element) in elements {
                    if name == &key {
                        return Some(element);
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn as_compound(&self) -> Option<&Vec<(&'a str, NbtTapeElement<'a>)>> {
        match self {
            NbtTapeElement::Compound(elements) => Some(elements),
            _ => None,
        }
    }

    /*pub fn as_list<T: NbtDeserializable<'a>>(&self, tape: &NbtTape<'a>) -> Option<Vec<T>> {
        tape.unpack_list(self)
    }*/
    pub fn as_list<T: FromNbt<'a>>(&self, tape: &NbtTape<'a>) -> Option<Vec<T>> {
        tape.unpack_list(self)
    }
}

impl<'a> NbtTape<'a> {
    // Data must live for atleast 'a
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            depth: 0,
            root: None,
        }
    }

    pub fn parse(&mut self) {
        self.parse_tag();
    }

    fn parse_tag(&mut self) {
        let tag = NbtTag::from(self.read_byte());
        if tag != NbtTag::Compound {
            panic!("Root tag must be a compound tag! Instead got: {:?}", tag);
        }

        let name: &str = <&str>::parse_from_nbt(self, NbtDeserializableOptions::None);

        self.root = Some((
            name,
            NbtTapeElement::parse_from_nbt(
                self,
                NbtDeserializableOptions::TagType(NbtTag::Compound),
            ),
        ));
    }

    #[inline]
    fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.pos];
        self.pos += 1;
        byte
    }

    #[inline]
    fn read_n_bytes(&mut self, n: usize) -> &'a [u8] {
        let start = self.pos;
        self.pos += n;
        &self.data[start..self.pos]
    }

    pub fn get(&self, key: &str) -> Option<&NbtTapeElement<'a>> {
        let res = self.root.as_ref().map(|(_, element)| element.get(key));

        res.flatten()
    }

    pub fn unpack_list<T: FromNbt<'a>>(&self, element: &NbtTapeElement<'a>) -> Option<Vec<T>> {
        match element {
            NbtTapeElement::List {
                elements_pos,
                size,
                el_type,
            } => {
                let mut tape = NbtTape {
                    data: self.data,
                    pos: *elements_pos,
                    depth: 0,
                    root: None,
                };
                let mut elements = vec![];
                for _ in 0..*size {
                    let nbt_element = NbtTapeElement::parse_from_nbt(
                        &mut tape,
                        NbtDeserializableOptions::TagType(el_type.clone()),
                    );

                    let element = T::from_nbt(&tape, &nbt_element).unwrap();

                    elements.push(element);
                }
                Some(elements)
            }
            NbtTapeElement::ByteArray(data) => {
                // I mean you wouldn't want to get the wrong type of data right?
                let data_vec = (*data).to_vec();
                let data = unsafe { std::mem::transmute::<Vec<i8>, Vec<T>>(data_vec) };

                if size_of::<T>() != size_of::<i8>() {
                    panic!("Invalid type conversion!");
                }

                // safety: there is none :) jk its a byte array so its fine
                // todo: revisit and see if this is actually safe
                // let data = unsafe {
                //     std::mem::forget(data);
                //     Vec::from_raw_parts(data.as_ptr() as *mut T, data.len(), data.len())
                // };

                Some(data)
            }
            NbtTapeElement::IntArray(data) => {
                let data = data.clone();
                let data = unsafe { std::mem::transmute::<Vec<i32>, Vec<T>>(data) };
                Some(data)
            }
            NbtTapeElement::LongArray(data) => {
                let data = data.clone();
                let data = unsafe { std::mem::transmute::<Vec<i64>, Vec<T>>(data) };
                Some(data)
            }
            _ => None,
        }
    }

    pub fn unpack_list_sliced<T: NbtDeserializable<'a>>(
        &self,
        element: &NbtTapeElement<'a>,
    ) -> Option<&'a [T]> {
        match element {
            NbtTapeElement::ByteArray(data) => {
                // I mean you wouldn't want to get the wrong type of data right?
                if size_of::<T>() != size_of::<i8>() {
                    return None;
                }
                let data = unsafe { std::mem::transmute::<&[i8], &[T]>(data) };

                Some(data)
            }
            NbtTapeElement::IntArray(data) => {
                if size_of::<T>() != size_of::<i32>() {
                    return None;
                }

                let data = data.as_slice();
                let data = unsafe { std::mem::transmute::<&[i32], &[T]>(data) };

                Some(data)
            }
            NbtTapeElement::LongArray(data) => {
                if size_of::<T>() != size_of::<i64>() {
                    return None;
                }

                let data = data.as_slice();
                let data = unsafe { std::mem::transmute::<&[i64], &[T]>(data) };

                Some(data)
            }

            _ => None,
        }
    }
}
impl NbtTape<'_> {
    /// Skips over a single tag based on its type.
    fn skip_tag(&mut self, tag: u8) -> usize {
        let start_pos = self.pos;
        match NbtTag::from(tag) {
            NbtTag::End => {
                // End tag has no payload.
            }
            NbtTag::Byte => {
                self.pos += 1;
            }
            NbtTag::Short => {
                self.pos += 2;
            }
            NbtTag::Int | NbtTag::Float => {
                self.pos += 4;
            }
            NbtTag::Long | NbtTag::Double => {
                self.pos += 8;
            }
            NbtTag::ByteArray => {
                // ByteArray: 4-byte length followed by 'length' bytes.
                let length = i32::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
                self.pos += length;
            }
            NbtTag::String => {
                // String: 2-byte length followed by 'length' bytes.
                let length = u16::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
                self.pos += length;
            }
            NbtTag::List => {
                // List: 1-byte element type, 4-byte length, followed by elements.
                let el_type = self.read_byte();
                let length = i32::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
                self.skip_list(el_type, length);
            }
            NbtTag::Compound => {
                // Compound: Contains named tags until an End tag.
                self.skip_compound();
            }
            NbtTag::IntArray => {
                // IntArray: 4-byte length followed by 'length' * 4 bytes.
                let length = i32::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
                self.pos += length * 4;
            }
            NbtTag::LongArray => {
                // LongArray: 4-byte length followed by 'length' * 8 bytes.
                let length = i32::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
                self.pos += length * 8;
            }
        }
        self.pos - start_pos
    }

    /// Skips over a list's elements based on element type and length.
    fn skip_list(&mut self, el_type: u8, length: usize) -> usize {
        let start_pos = self.pos;
        for _ in 0..length {
            self.skip_tag(el_type);
        }
        self.pos - start_pos
    }

    /// Skips over a compound's elements until an End tag is encountered.
    fn skip_compound(&mut self) -> usize {
        let start_pos = self.pos;
        loop {
            let tag_byte = self.read_byte();
            let tag = NbtTag::from(tag_byte);
            if tag == NbtTag::End {
                break;
            }
            // Skip the name: 2-byte length + name bytes.
            let name_length = u16::parse_from_nbt(self, NbtDeserializableOptions::None) as usize;
            self.pos += name_length;
            // Skip the tag's payload.
            self.skip_tag(tag as u8);
        }
        self.pos - start_pos
    }
}

pub enum NbtDeserializableOptions {
    None,
    TagType(NbtTag),
}
pub trait NbtDeserializable<'a>: Sized {
    fn parse_from_bytes(data: &'a [u8]) -> Self;
    fn parse_from_nbt(tape: &mut NbtTape<'a>, _opts: NbtDeserializableOptions) -> Self {
        //! By default, this function directly reads the bytes
        //! from the tape and BE deserializes them.

        // Read from current pos ~ pos + size_of::<Self>()
        Self::parse_from_bytes(tape.read_n_bytes(size_of::<Self>()))
    }
}

mod primitives {
    use super::NbtDeserializable;
    impl NbtDeserializable<'_> for i8 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            data[0] as i8
        }
    }
    impl NbtDeserializable<'_> for u8 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            u8::from_be_bytes([data[0]])
        }
    }
    impl NbtDeserializable<'_> for i16 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            i16::from_be_bytes([data[0], data[1]])
        }
    }

    impl NbtDeserializable<'_> for u16 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            u16::from_be_bytes([data[0], data[1]])
        }
    }

    impl NbtDeserializable<'_> for i32 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            i32::from_be_bytes([data[0], data[1], data[2], data[3]])
        }
    }

    impl NbtDeserializable<'_> for u32 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            u32::from_be_bytes([data[0], data[1], data[2], data[3]])
        }
    }

    impl NbtDeserializable<'_> for i64 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            i64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ])
        }
    }

    impl NbtDeserializable<'_> for u64 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            u64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ])
        }
    }

    impl NbtDeserializable<'_> for f32 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            f32::from_be_bytes([data[0], data[1], data[2], data[3]])
        }
    }

    impl NbtDeserializable<'_> for f64 {
        fn parse_from_bytes(data: &[u8]) -> Self {
            f64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ])
        }
    }

    impl NbtDeserializable<'_> for bool {
        fn parse_from_bytes(data: &[u8]) -> Self {
            data[0] != 0
        }
    }
}

mod taped {
    use super::*;

    impl<'a> NbtDeserializable<'a> for NbtTapeElement<'a> {
        fn parse_from_bytes(data: &'a [u8]) -> Self {
            let mut tape = NbtTape::new(data);
            let opts = NbtDeserializableOptions::TagType(NbtTag::Compound);
            Self::parse_from_nbt(&mut tape, opts)
        }

        fn parse_from_nbt(tape: &mut NbtTape<'a>, opts: NbtDeserializableOptions) -> Self {
            let tag = match opts {
                NbtDeserializableOptions::None => panic!("NbtTapeElement must have a tag type!"),
                NbtDeserializableOptions::TagType(tag) => tag,
            };
            match tag {
                NbtTag::End => NbtTapeElement::End,
                NbtTag::Byte => {
                    NbtTapeElement::Byte(i8::parse_from_nbt(tape, NbtDeserializableOptions::None))
                }
                NbtTag::Short => {
                    NbtTapeElement::Short(i16::parse_from_nbt(tape, NbtDeserializableOptions::None))
                }
                NbtTag::Int => {
                    NbtTapeElement::Int(i32::parse_from_nbt(tape, NbtDeserializableOptions::None))
                }
                NbtTag::Long => {
                    NbtTapeElement::Long(i64::parse_from_nbt(tape, NbtDeserializableOptions::None))
                }
                NbtTag::Float => {
                    NbtTapeElement::Float(f32::parse_from_nbt(tape, NbtDeserializableOptions::None))
                }
                NbtTag::Double => NbtTapeElement::Double(f64::parse_from_nbt(
                    tape,
                    NbtDeserializableOptions::None,
                )),
                NbtTag::ByteArray => {
                    let len = i32::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;
                    let data = tape.read_n_bytes(len);
                    let data = arrays::u8_slice_to_i8(data);
                    NbtTapeElement::ByteArray(data)
                }
                NbtTag::String => {
                    let len = u16::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;
                    let data = tape.read_n_bytes(len);
                    // SAFETY: The string is of len and is valid utf8
                    let data = unsafe { std::str::from_utf8_unchecked(data) };
                    NbtTapeElement::String(data)
                }
                NbtTag::List => {
                    let el_type = tape.read_byte();
                    let size = i32::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;

                    let elements_pos = tape.pos;

                    // Skip the list's elements
                    tape.skip_list(el_type, size);

                    let el_type = NbtTag::from(el_type);
                    NbtTapeElement::List {
                        el_type,
                        size,
                        elements_pos,
                    }
                }
                NbtTag::Compound => {
                    tape.depth += 1;
                    let mut elements = vec![];
                    loop {
                        let tag = NbtTag::from(tape.read_byte());
                        if tag == NbtTag::End {
                            tape.depth -= 1;

                            return NbtTapeElement::Compound(elements);
                        }

                        let name = <&str>::parse_from_nbt(tape, NbtDeserializableOptions::None);
                        let element = NbtTapeElement::parse_from_nbt(
                            tape,
                            NbtDeserializableOptions::TagType(tag),
                        );
                        elements.push((name, element));
                    }
                }
                NbtTag::IntArray => {
                    let len = i32::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;
                    let data = tape.read_n_bytes(len * size_of::<i32>());
                    let data = arrays::u8_slice_to_i32_be(data);
                    NbtTapeElement::IntArray(data)
                }
                NbtTag::LongArray => {
                    let len = i32::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;
                    let data = tape.read_n_bytes(len * size_of::<i64>());
                    let data = arrays::u8_slice_to_i64_be(data);
                    NbtTapeElement::LongArray(data)
                }
            }
        }
    }
}

mod general {
    use super::*;

    impl<'a> NbtDeserializable<'a> for String {
        fn parse_from_bytes(data: &'a [u8]) -> Self {
            //! Mustn't call this function with length prefixed data. Must only be the string itself.
            //! Look at the implementation of `<&str>::parse_from_bytes` for more detail!
            // SAFETY: The string is valid utf8, unless ofc, you mess up the call.
            unsafe { std::str::from_utf8_unchecked(data) }.to_string()
        }

        fn parse_from_nbt(tape: &mut NbtTape<'a>, _opts: NbtDeserializableOptions) -> Self {
            <&str>::parse_from_nbt(tape, NbtDeserializableOptions::None).to_string()
        }
    }

    impl<'a> NbtDeserializable<'a> for &'a str {
        fn parse_from_bytes(data: &'a [u8]) -> Self {
            // This function must be called with the length buffer exactly of the string.
            // The data must NOT be length prefixed. Just the plain utf data.
            // (I don't know why I made it like this lmfao)
            // SAFETY: The string is valid utf8. Not length prefixed!
            unsafe { std::str::from_utf8_unchecked(data) }
        }

        fn parse_from_nbt(tape: &mut NbtTape<'a>, _opts: NbtDeserializableOptions) -> Self {
            let len = u16::parse_from_nbt(tape, NbtDeserializableOptions::None) as usize;
            if len == 0 {
                return "";
            }
            let data = tape.read_n_bytes(len);
            Self::parse_from_bytes(data)
        }
    }
}

/// tf, whats the point of this?
/// the data will probably die?? idk? possibly not? ?? lmao
impl NetEncode for NbtTape<'_> {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        let data = self.data;
        writer.write_all(data)?;
        Ok(())
    }

    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        use tokio::io::AsyncWriteExt;
        let data = self.data;
        writer.write_all(data).await?;
        Ok(())
    }
}


impl NbtTapeElement<'_> {
    pub fn serialize_as_network(&self, tape: &mut NbtTape, writer: &mut Vec<u8>, opts: &NBTSerializeOptions) -> NetEncodeResult<()> {
        /*if let NBTSerializeOptions::WithHeader(name) = opts {
            writer.write_all(&[self.nbt_id()])?;
            name.serialize(writer, &NBTSerializeOptions::None);
        }*/
        
        match opts {
            NBTSerializeOptions::None => {}
            NBTSerializeOptions::WithHeader(name) => {
                writer.write_all(&[self.nbt_id()])?;
                name.serialize(writer, &NBTSerializeOptions::None);
            }
            NBTSerializeOptions::Network | NBTSerializeOptions::Flatten => {
                writer.write_all(&[self.nbt_id()])?;
            }
        }
        
        
        match self {
            NbtTapeElement::End => Ok(()),
            NbtTapeElement::Byte(val) => {
                writer.write_all(&[*val as u8])?;
                Ok(())
            }
            NbtTapeElement::Short(val) => {
                writer.write_all(&val.to_be_bytes())?;
                Ok(())
            }
            NbtTapeElement::Int(val) => {
                writer.write_all(&val.to_be_bytes())?;
                Ok(())
            }
            NbtTapeElement::Long(val) => {
                writer.write_all(&val.to_be_bytes())?;
                Ok(())
            }
            NbtTapeElement::Float(val) => {
                writer.write_all(&val.to_be_bytes())?;
                Ok(())
            }
            NbtTapeElement::Double(val) => {
                writer.write_all(&val.to_be_bytes())?;
                Ok(())
            }
            NbtTapeElement::ByteArray(data) => {
                (data.len() as i32).serialize(writer, &NBTSerializeOptions::None);
                let data = unsafe { std::mem::transmute::<&[i8], &[u8]>(data) };
                writer.write_all(data)?;
                Ok(())
            }
            NbtTapeElement::String(data) => {
                data.serialize(writer, &NBTSerializeOptions::None);
                /*let data = data.as_bytes();
                (data.len() as u16).serialize(writer, &NBTSerializeOptions::None);
                writer.write_all(data)?;*/
                Ok(())
            }
            /*NbtTapeElement::List {
                el_type,
                size,
                elements_pos,
            } => {
                writer.write_all(&[el_type.clone() as u8])?;
                (*size as i32).serialize(writer, &NBTSerializeOptions::None);

                let start = *elements_pos;

                // rewind tape to the start of the list.
                tape.pos = start;

                // read the entire list (it returns the entire list)
                let skipped = tape.skip_list(el_type.clone() as u8, *size);

                let end = start + skipped;

                let data = &tape.data[start..end];

                writer.write_all(data)?;

                Ok(())
            }*/
            NbtTapeElement::List {
                el_type,
                size,
                elements_pos,
            } => {
                writer.write_all(&[el_type.clone() as u8])?;
                (*size as i32).serialize(writer, &NBTSerializeOptions::None);

                // Rewind tape to the start of the list.
                tape.pos = *elements_pos;

                // For each element in the list, parse and serialize it.
                for _ in 0..*size {
                    let element = NbtTapeElement::parse_from_nbt(
                        tape,
                        NbtDeserializableOptions::TagType(el_type.clone()),
                    );
                    element.serialize_as_network(tape, writer, &NBTSerializeOptions::None)?;
                }

                Ok(())
            }
            NbtTapeElement::Compound(elements) => {
                for (name, element) in elements {
                    writer.write_all(&[element.nbt_id()])?;
                    name.serialize(writer, &NBTSerializeOptions::None);
                    element.serialize_as_network(tape, writer, &NBTSerializeOptions::None)?;
                }
                writer.write_all(&[NbtTag::End as u8])?;
                Ok(())
            }
            NbtTapeElement::IntArray(data) => {
                (data.len() as i32).serialize(writer, &NBTSerializeOptions::None);
                let data = unsafe {
                    std::mem::transmute::<&[i32], &[u32]>(data.as_slice())
                };
                let data = arrays::u32_slice_to_u8_be(data);
                writer.write_all(data.as_slice())?;
                Ok(())
            }
            NbtTapeElement::LongArray(data) => {
                (data.len() as i32).serialize(writer, &NBTSerializeOptions::None);
                let data = unsafe {
                    std::mem::transmute::<&[i64], &[u64]>(data.as_slice())
                };
                let data = arrays::u64_slice_to_u8_be(data);
                writer.write_all(data.as_slice())?;
                Ok(())
            }
        }
    }
}
