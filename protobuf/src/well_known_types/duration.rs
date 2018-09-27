// This file is generated by rust-protobuf 2.0.6. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Duration {
    // message fields
    pub seconds: i64,
    pub nanos: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip_deserializing,skip_serializing))]
    cached_size: ::protobuf::CachedSize,
}

impl Duration {
    pub fn new() -> Duration {
        ::std::default::Default::default()
    }

    // int64 seconds = 1;

    pub fn clear_seconds(&mut self) {
        self.seconds = 0;
    }

    // Param is passed by value, moved
    pub fn set_seconds(&mut self, v: i64) {
        self.seconds = v;
    }

    pub fn get_seconds(&self) -> i64 {
        self.seconds
    }

    // int32 nanos = 2;

    pub fn clear_nanos(&mut self) {
        self.nanos = 0;
    }

    // Param is passed by value, moved
    pub fn set_nanos(&mut self, v: i32) {
        self.nanos = v;
    }

    pub fn get_nanos(&self) -> i32 {
        self.nanos
    }
}

impl ::protobuf::Message for Duration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.seconds = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.nanos = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.seconds != 0 {
            my_size += ::protobuf::rt::value_size(1, self.seconds, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nanos != 0 {
            my_size += ::protobuf::rt::value_size(2, self.nanos, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.seconds != 0 {
            os.write_int64(1, self.seconds)?;
        }
        if self.nanos != 0 {
            os.write_int32(2, self.nanos)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Duration {
        Duration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "seconds",
                    |m: &Duration| { &m.seconds },
                    |m: &mut Duration| { &mut m.seconds },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "nanos",
                    |m: &Duration| { &m.nanos },
                    |m: &mut Duration| { &mut m.nanos },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Duration>(
                    "Duration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Duration {
        static mut instance: ::protobuf::lazy::Lazy<Duration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Duration,
        };
        unsafe {
            instance.get(Duration::new)
        }
    }
}

impl ::protobuf::Clear for Duration {
    fn clear(&mut self) {
        self.clear_seconds();
        self.clear_nanos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Duration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Duration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/protobuf/duration.proto\x12\x0fgoogle.protobuf\":\n\x08Dura\
    tion\x12\x18\n\x07seconds\x18\x01\x20\x01(\x03R\x07seconds\x12\x14\n\x05\
    nanos\x18\x02\x20\x01(\x05R\x05nanosB|\n\x13com.google.protobufB\rDurati\
    onProtoP\x01Z*github.com/golang/protobuf/ptypes/duration\xf8\x01\x01\xa2\
    \x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.WellKnownTypesJ\xaa\"\n\x06\x12\
    \x04\x1e\0g\x01\n\xcc\x0c\n\x01\x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protoc\
    ol\x20Buffers\x20-\x20Google's\x20data\x20interchange\x20format\n\x20Cop\
    yright\x202008\x20Google\x20Inc.\x20\x20All\x20rights\x20reserved.\n\x20\
    https://developers.google.com/protocol-buffers/\n\n\x20Redistribution\
    \x20and\x20use\x20in\x20source\x20and\x20binary\x20forms,\x20with\x20or\
    \x20without\n\x20modification,\x20are\x20permitted\x20provided\x20that\
    \x20the\x20following\x20conditions\x20are\n\x20met:\n\n\x20\x20\x20\x20\
    \x20*\x20Redistributions\x20of\x20source\x20code\x20must\x20retain\x20th\
    e\x20above\x20copyright\n\x20notice,\x20this\x20list\x20of\x20conditions\
    \x20and\x20the\x20following\x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Red\
    istributions\x20in\x20binary\x20form\x20must\x20reproduce\x20the\x20abov\
    e\n\x20copyright\x20notice,\x20this\x20list\x20of\x20conditions\x20and\
    \x20the\x20following\x20disclaimer\n\x20in\x20the\x20documentation\x20an\
    d/or\x20other\x20materials\x20provided\x20with\x20the\n\x20distribution.\
    \n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\x20Inc\
    .\x20nor\x20the\x20names\x20of\x20its\n\x20contributors\x20may\x20be\x20\
    used\x20to\x20endorse\x20or\x20promote\x20products\x20derived\x20from\n\
    \x20this\x20software\x20without\x20specific\x20prior\x20written\x20permi\
    ssion.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIG\
    HT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20\
    EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\
    \x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABIL\
    ITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20D\
    ISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\
    \x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIR\
    ECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\
    \x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREM\
    ENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE\
    ,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOW\
    EVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WH\
    ETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INC\
    LUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\
    \x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\
    \x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\
    \n\x01\x02\x12\x03\x20\x08\x17\n\x08\n\x01\x08\x12\x03\"\0;\n\x0b\n\x04\
    \x08\xe7\x07\0\x12\x03\"\0;\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\"\x07\
    \x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\"\x07\x17\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\"\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\
    \"\x1a:\n\x08\n\x01\x08\x12\x03#\0\x1f\n\x0b\n\x04\x08\xe7\x07\x01\x12\
    \x03#\0\x1f\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03#\x07\x17\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03#\x07\x17\n\x0e\n\x07\x08\xe7\x07\x01\x02\
    \0\x01\x12\x03#\x07\x17\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03#\x1a\x1e\
    \n\x08\n\x01\x08\x12\x03$\0A\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03$\0A\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03$\x07\x11\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03$\x07\x11\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03$\x07\x11\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03$\x14@\n\x08\n\x01\
    \x08\x12\x03%\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03%\0,\n\x0c\n\x05\x08\
    \xe7\x07\x03\x02\x12\x03%\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\
    \x03%\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03%\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03%\x16+\n\x08\n\x01\x08\x12\x03&\0.\
    \n\x0b\n\x04\x08\xe7\x07\x04\x12\x03&\0.\n\x0c\n\x05\x08\xe7\x07\x04\x02\
    \x12\x03&\x07\x1b\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03&\x07\x1b\n\x0e\
    \n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03&\x07\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x04\x07\x12\x03&\x1e-\n\x08\n\x01\x08\x12\x03'\0\"\n\x0b\n\x04\x08\
    \xe7\x07\x05\x12\x03'\0\"\n\x0c\n\x05\x08\xe7\x07\x05\x02\x12\x03'\x07\
    \x1a\n\r\n\x06\x08\xe7\x07\x05\x02\0\x12\x03'\x07\x1a\n\x0e\n\x07\x08\
    \xe7\x07\x05\x02\0\x01\x12\x03'\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x05\x03\
    \x12\x03'\x1d!\n\x08\n\x01\x08\x12\x03(\0!\n\x0b\n\x04\x08\xe7\x07\x06\
    \x12\x03(\0!\n\x0c\n\x05\x08\xe7\x07\x06\x02\x12\x03(\x07\x18\n\r\n\x06\
    \x08\xe7\x07\x06\x02\0\x12\x03(\x07\x18\n\x0e\n\x07\x08\xe7\x07\x06\x02\
    \0\x01\x12\x03(\x07\x18\n\x0c\n\x05\x08\xe7\x07\x06\x07\x12\x03(\x1b\x20\
    \n\x92\x0c\n\x02\x04\0\x12\x04Z\0g\x01\x1a\x85\x0c\x20A\x20Duration\x20r\
    epresents\x20a\x20signed,\x20fixed-length\x20span\x20of\x20time\x20repre\
    sented\n\x20as\x20a\x20count\x20of\x20seconds\x20and\x20fractions\x20of\
    \x20seconds\x20at\x20nanosecond\n\x20resolution.\x20It\x20is\x20independ\
    ent\x20of\x20any\x20calendar\x20and\x20concepts\x20like\x20\"day\"\n\x20\
    or\x20\"month\".\x20It\x20is\x20related\x20to\x20Timestamp\x20in\x20that\
    \x20the\x20difference\x20between\n\x20two\x20Timestamp\x20values\x20is\
    \x20a\x20Duration\x20and\x20it\x20can\x20be\x20added\x20or\x20subtracted\
    \n\x20from\x20a\x20Timestamp.\x20Range\x20is\x20approximately\x20+-10,00\
    0\x20years.\n\n\x20Example\x201:\x20Compute\x20Duration\x20from\x20two\
    \x20Timestamps\x20in\x20pseudo\x20code.\n\n\x20\x20\x20\x20\x20Timestamp\
    \x20start\x20=\x20...;\n\x20\x20\x20\x20\x20Timestamp\x20end\x20=\x20...\
    ;\n\x20\x20\x20\x20\x20Duration\x20duration\x20=\x20...;\n\n\x20\x20\x20\
    \x20\x20duration.seconds\x20=\x20end.seconds\x20-\x20start.seconds;\n\
    \x20\x20\x20\x20\x20duration.nanos\x20=\x20end.nanos\x20-\x20start.nanos\
    ;\n\n\x20\x20\x20\x20\x20if\x20(duration.seconds\x20<\x200\x20&&\x20dura\
    tion.nanos\x20>\x200)\x20{\n\x20\x20\x20\x20\x20\x20\x20duration.seconds\
    \x20+=\x201;\n\x20\x20\x20\x20\x20\x20\x20duration.nanos\x20-=\x20100000\
    0000;\n\x20\x20\x20\x20\x20}\x20else\x20if\x20(durations.seconds\x20>\
    \x200\x20&&\x20duration.nanos\x20<\x200)\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20duration.seconds\x20-=\x201;\n\x20\x20\x20\x20\x20\x20\x20duration.n\
    anos\x20+=\x201000000000;\n\x20\x20\x20\x20\x20}\n\n\x20Example\x202:\
    \x20Compute\x20Timestamp\x20from\x20Timestamp\x20+\x20Duration\x20in\x20\
    pseudo\x20code.\n\n\x20\x20\x20\x20\x20Timestamp\x20start\x20=\x20...;\n\
    \x20\x20\x20\x20\x20Duration\x20duration\x20=\x20...;\n\x20\x20\x20\x20\
    \x20Timestamp\x20end\x20=\x20...;\n\n\x20\x20\x20\x20\x20end.seconds\x20\
    =\x20start.seconds\x20+\x20duration.seconds;\n\x20\x20\x20\x20\x20end.na\
    nos\x20=\x20start.nanos\x20+\x20duration.nanos;\n\n\x20\x20\x20\x20\x20i\
    f\x20(end.nanos\x20<\x200)\x20{\n\x20\x20\x20\x20\x20\x20\x20end.seconds\
    \x20-=\x201;\n\x20\x20\x20\x20\x20\x20\x20end.nanos\x20+=\x201000000000;\
    \n\x20\x20\x20\x20\x20}\x20else\x20if\x20(end.nanos\x20>=\x201000000000)\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20end.seconds\x20+=\x201;\n\x20\x20\x20\
    \x20\x20\x20\x20end.nanos\x20-=\x201000000000;\n\x20\x20\x20\x20\x20}\n\
    \n\x20Example\x203:\x20Compute\x20Duration\x20from\x20datetime.timedelta\
    \x20in\x20Python.\n\n\x20\x20\x20\x20\x20td\x20=\x20datetime.timedelta(d\
    ays=3,\x20minutes=10)\n\x20\x20\x20\x20\x20duration\x20=\x20Duration()\n\
    \x20\x20\x20\x20\x20duration.FromTimedelta(td)\n\n\n\n\n\n\x03\x04\0\x01\
    \x12\x03Z\x08\x10\np\n\x04\x04\0\x02\0\x12\x03^\x02\x14\x1ac\x20Signed\
    \x20seconds\x20of\x20the\x20span\x20of\x20time.\x20Must\x20be\x20from\
    \x20-315,576,000,000\n\x20to\x20+315,576,000,000\x20inclusive.\n\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04^\x02Z\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03^\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03^\x08\x0f\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03^\x12\x13\n\x83\x03\n\x04\x04\0\x02\x01\x12\x03f\
    \x02\x12\x1a\xf5\x02\x20Signed\x20fractions\x20of\x20a\x20second\x20at\
    \x20nanosecond\x20resolution\x20of\x20the\x20span\n\x20of\x20time.\x20Du\
    rations\x20less\x20than\x20one\x20second\x20are\x20represented\x20with\
    \x20a\x200\n\x20`seconds`\x20field\x20and\x20a\x20positive\x20or\x20nega\
    tive\x20`nanos`\x20field.\x20For\x20durations\n\x20of\x20one\x20second\
    \x20or\x20more,\x20a\x20non-zero\x20value\x20for\x20the\x20`nanos`\x20fi\
    eld\x20must\x20be\n\x20of\x20the\x20same\x20sign\x20as\x20the\x20`second\
    s`\x20field.\x20Must\x20be\x20from\x20-999,999,999\n\x20to\x20+999,999,9\
    99\x20inclusive.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04f\x02^\x14\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03f\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03f\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03f\x10\x11b\x06proto\
    3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
