mod bytecode;
pub mod class_printer;
mod stack;
mod frames;
mod thread;
mod execution;
mod heap;
mod jvm;
mod method_area;
mod class;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader};
use log::{info, debug, trace, warn};

static VERSIONS: [&str; 20] = ["1.1", "1.2", "1.3", "1.4", "5.0", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"];
const VERSION_SHIFT: usize = 45;

#[derive(Debug, PartialEq)]
pub struct ConstantPoolUtf8Info {
    tag: u8,
    pub string: String,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolIntegerInfo {
    tag: u8,
    pub value: i32,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolFloatInfo {
    tag: u8,
    pub value: f32,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolLongInfo {
    tag: u8,
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolDoubleInfo {
    tag: u8,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolClassInfo {
    tag: u8,
    pub name_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolStringInfo {
    tag: u8,
    pub name_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolFieldRefInfo {
    tag: u8,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolMethodRefInfo {
    tag: u8,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolInterfaceMethodRefInfo {
    tag: u8,
    pub class_index: usize,
    pub name_and_type_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConstantPoolNameAndTypeInfo {
    tag: u8,
    pub name_index: usize,
    pub descriptor_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct DynamicInfo {
    tag: u8,
    pub bootstrap_method_attr_index: usize,
    pub name_and_type_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct MethodKindInfo {
    tag: u8,
    pub reference_kind: u8,
    pub reference_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct InvokeDynamicInfo {
    tag: u8,
    pub bootstrap_method_attr_index: usize,
    pub name_and_type_index: usize,
}


#[derive(Debug, PartialEq)]
pub enum ConstantPoolInfo {
    Utf8(ConstantPoolUtf8Info),
    Integer(ConstantPoolIntegerInfo),
    Float(ConstantPoolFloatInfo),
    Long(ConstantPoolLongInfo),
    Double(ConstantPoolDoubleInfo),
    Class(ConstantPoolClassInfo),
    String(ConstantPoolStringInfo),
    FieldRef(ConstantPoolFieldRefInfo),
    MethodRef(ConstantPoolMethodRefInfo),
    InterfaceMethodRef(ConstantPoolInterfaceMethodRefInfo),
    NameAndType(ConstantPoolNameAndTypeInfo),
    MethodKind(MethodKindInfo),
    Dynamic(DynamicInfo),
    InvokeDynamic(InvokeDynamicInfo),
}

#[derive(Debug, PartialEq)]
pub struct AttributeConstantValue {
    pub constant_value_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ExceptionTableEntry {
    start_pc: usize,
    end_pc: usize,
    handler_pc: usize,
    catch_pc: usize,
}

#[derive(Debug, PartialEq)]
pub struct LineNumberTableEntry {
    start_pc: usize,
    line_number: usize,
}

#[derive(Debug, PartialEq)]
pub struct AttributeCode {
    attribute_name_index: usize,
    max_stack: usize,
    max_locals: usize,
    code_length: usize,
    code: Vec<u8>,
    exceptions: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}

impl AttributeCode {
    pub fn print(&self) {
        // for byte in self.code {
        //     match byte { _ => {} }
        // }
    }
}

#[derive(Debug, PartialEq)]
pub struct AttributeExceptions {
// TODO
}

#[derive(Debug, PartialEq)]
pub struct AttributeSourceFile {
// TODO
}

#[derive(Debug, PartialEq)]
pub struct AttributeLineNumberTable {
    pub attribute_name_index: usize,
    pub attribute_name: String,
    pub attribute_length: usize,
    pub line_number_entries: Vec<LineNumberTableEntry>,
}

#[derive(Debug, PartialEq)]
pub struct AttributeLocalVariableTable {
// TODO
}

#[derive(Debug, PartialEq)]
pub struct AttributeInnerClasses {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct AttributeSynthetic {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct AttributeDeprecated {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    ConstantValue(AttributeConstantValue),
    Code(AttributeCode),
    Exceptions(AttributeExceptions),
    SourceFile(AttributeSourceFile),
    LineNumberTable(AttributeLineNumberTable),
    LocalVariableTable(AttributeLocalVariableTable),
    InnerClasses(AttributeInnerClasses),
    Synthetic(AttributeSynthetic),
    Deprecated(AttributeDeprecated),
}

#[derive(Debug, PartialEq)]
pub struct AttributeInfo {
    pub attribute_name_index: usize,
    pub attribute: Attribute,
}

#[derive(Debug, PartialEq)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: usize,
    pub name: String,
    pub descriptor_index: usize,
    pub attributes_info: Vec<AttributeInfo>,
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: usize,
    pub name: String,
    pub descriptor_index: usize,
    pub attributes_info: Vec<AttributeInfo>,
}

#[derive(PartialEq, Debug)]
pub struct ClassFlags(u16);

impl ClassFlags {
    pub fn new(flags: u16) -> Self {
        ClassFlags(flags)
    }

    pub fn is_public(&self) -> bool {
        self.is_bit_set(0x0001)
    }

    pub fn is_final(&self) -> bool {
        self.is_bit_set(0x0010)
    }

    pub fn is_super(&self) -> bool {
        self.is_bit_set(0x0020)
    }

    pub fn is_interface(&self) -> bool {
        self.is_bit_set(0x0200)
    }

    pub fn is_abstract(&self) -> bool {
        self.is_bit_set(0x0400)
    }

    pub fn is_synthetic(&self) -> bool {
        self.is_bit_set(0x1000)
    }

    pub fn is_annotation(&self) -> bool {
        self.is_bit_set(0x2000)
    }

    pub fn is_enum(&self) -> bool {
        self.is_bit_set(0x4000)
    }

    pub fn is_module(&self) -> bool {
        self.is_bit_set(0x8000)
    }

    fn is_bit_set(&self, bit: u16) -> bool {
        self.0 & bit != 0
    }
}

#[derive(PartialEq, Debug)]
pub struct JavaClass {
    pub prelude: u32,
    pub major: u16,
    pub minor: u16,
    pub constant_pool_count: usize,
    pub constant_pool_infos: Vec<ConstantPoolInfo>,
    pub flags: ClassFlags,
    pub this_class: usize,
    pub super_class: usize,
    pub interfaces_count: usize,
    pub interfaces_indexes: Vec<usize>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

struct BytesReader {
    eight_bytes_buffer: [u8; 8],
    four_bytes_buffer: [u8; 4],
    two_bytes_buffer: [u8; 2],
    one_byte_buffer: [u8; 1],
    buf_reader: BufReader<File>,
    read_bytes: Vec<usize>,
}

impl BytesReader {
    fn new(buf_reader: BufReader<File>) -> Self {
        BytesReader {
            eight_bytes_buffer: [0; 8],
            four_bytes_buffer: [0; 4],
            two_bytes_buffer: [0; 2],
            one_byte_buffer: [0; 1],
            buf_reader,
            read_bytes: Vec::new(),
        }
    }

    fn read_i64(&mut self) -> i64 {
        self.buf_reader.read(&mut self.eight_bytes_buffer).unwrap();
        self.mark_read_bytes(8);
        i64::from_be_bytes(self.eight_bytes_buffer)
    }

    fn read_u32(&mut self) -> u32 {
        self.buf_reader.read(&mut self.four_bytes_buffer).unwrap();
        self.mark_read_bytes(4);
        u32::from_be_bytes(self.four_bytes_buffer)
    }

    fn read_i32(&mut self) -> i32 {
        self.buf_reader.read(&mut self.four_bytes_buffer).unwrap();
        self.mark_read_bytes(4);
        i32::from_be_bytes(self.four_bytes_buffer)
    }

    fn read_u16(&mut self) -> u16 {
        self.buf_reader.read(&mut self.two_bytes_buffer).unwrap();
        self.mark_read_bytes(2);
        u16::from_be_bytes(self.two_bytes_buffer)
    }

    fn read_i16(&mut self) -> i16 {
        self.buf_reader.read(&mut self.two_bytes_buffer).unwrap();
        self.mark_read_bytes(2);
        i16::from_be_bytes(self.two_bytes_buffer)
    }

    fn read_u8(&mut self) -> u8 {
        self.buf_reader.read(&mut self.one_byte_buffer).unwrap();
        self.mark_read_bytes(1);
        u8::from_be_bytes(self.one_byte_buffer)
    }

    fn read_f32(&mut self) -> f32 {
        self.buf_reader.read(&mut self.four_bytes_buffer).unwrap();
        self.mark_read_bytes(4);
        f32::from_be_bytes(self.four_bytes_buffer)
    }

    fn read_f64(&mut self) -> f64 {
        self.buf_reader.read(&mut self.eight_bytes_buffer).unwrap();
        self.mark_read_bytes(8);
        f64::from_be_bytes(self.eight_bytes_buffer)
    }

    fn read_bytes(&mut self, bytes_to_read: usize) -> Vec<u8> {
        let mut output: Vec<u8> = vec![0; bytes_to_read];
        self.buf_reader.read(&mut output).unwrap();
        self.mark_read_bytes(bytes_to_read);
        output
    }

    fn read_string(&mut self, bytes_of_utf8_string: usize) -> String {
        let vec = self.read_bytes(bytes_of_utf8_string);
        self.mark_read_bytes(bytes_of_utf8_string);
        String::from_utf8(vec).unwrap()
    }

    fn consume(&mut self, amt: usize) -> () {
        self.mark_read_bytes(amt);
        self.buf_reader.consume(amt);
    }

    fn mark_read_bytes(&mut self, amt: usize) {
        for num in self.read_bytes.iter_mut() {
            *num += amt;
        }
    }

    fn begin_read_block(&mut self) {
        self.read_bytes.push(0);
    }

    fn assert_read(&mut self, amt: usize) {
        let read_bytes = self.read_bytes.pop().unwrap();
        assert_eq!(read_bytes, amt, "Expected to read {amt} bytes but read {read_bytes}");
    }
}

impl JavaClass {
    fn check_preludee(self) -> Result<(), String> {
        Self::check_prelude(self.prelude)
    }
    fn java_version(self) -> Result<&'static str, String> {
        Self::resolve_version(self.major)
    }

    pub fn resolve_version(major: u16) -> Result<&'static str, String> {
        let major_index = major as usize - VERSION_SHIFT;
        if major_index > VERSIONS.len() - 1 {
            Err(format!("Major version {major} is not supported"))
        } else {
            Ok(VERSIONS[major_index])
        }
    }

    fn check_prelude(prelude: u32) -> Result<(), String> {
        if prelude == 0xCAFEBABE {
            Ok(())
        } else {
            Err(format!("The prelude is 0x{:X} instead of 0xCAFEBABE", prelude))
        }
    }


    pub fn load_from_file(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;

        let mut bytes_reader = BytesReader::new(BufReader::new(file));


        let prelude = bytes_reader.read_u32();
        let minor = bytes_reader.read_u16();
        let major = bytes_reader.read_u16();
        let constant_pool_count: usize = bytes_reader.read_u16() as usize;

        let mut constant_pool_infos: Vec<ConstantPoolInfo> = Vec::with_capacity(constant_pool_count + 1);

        for _ in 1..constant_pool_count {
            let tag = bytes_reader.read_u8();
            match tag {
                1 => {
                    let length = bytes_reader.read_u16() as usize;
                    let string = bytes_reader.read_string(length);
                    constant_pool_infos.push(ConstantPoolInfo::Utf8(ConstantPoolUtf8Info {
                        tag,
                        string,
                    }));
                }
                3 => {
                    constant_pool_infos.push(ConstantPoolInfo::Integer(ConstantPoolIntegerInfo {
                        tag,
                        value: bytes_reader.read_i32(),
                    }))
                }
                4 => {
                    constant_pool_infos.push(ConstantPoolInfo::Float(ConstantPoolFloatInfo {
                        tag,
                        value: bytes_reader.read_f32(),
                    }))
                }
                5 => {
                    constant_pool_infos.push(ConstantPoolInfo::Long(ConstantPoolLongInfo {
                        tag,
                        value: bytes_reader.read_i64(),
                    }))
                }
                6 => {
                    constant_pool_infos.push(ConstantPoolInfo::Double(ConstantPoolDoubleInfo {
                        tag,
                        value: bytes_reader.read_f64(),
                    }))
                }
                7 => {
                    constant_pool_infos.push(ConstantPoolInfo::Class(ConstantPoolClassInfo {
                        tag,
                        name_index: bytes_reader.read_u16() as usize,
                    }))
                }
                8 => {
                    constant_pool_infos.push(ConstantPoolInfo::String(ConstantPoolStringInfo {
                        tag,
                        name_index: bytes_reader.read_u16() as usize,
                    }))
                }
                9 => {
                    constant_pool_infos.push(ConstantPoolInfo::FieldRef(ConstantPoolFieldRefInfo {
                        tag,
                        class_index: bytes_reader.read_u16() as usize,
                        name_and_type_index: bytes_reader.read_u16() as usize,
                    }))
                }
                10 => {
                    constant_pool_infos.push(ConstantPoolInfo::MethodRef(ConstantPoolMethodRefInfo {
                        tag,
                        class_index: bytes_reader.read_u16() as usize,
                        name_and_type_index: bytes_reader.read_u16() as usize,
                    }))
                }
                11 => {
                    constant_pool_infos.push(ConstantPoolInfo::InterfaceMethodRef(ConstantPoolInterfaceMethodRefInfo {
                        tag,
                        class_index: bytes_reader.read_u16() as usize,
                        name_and_type_index: bytes_reader.read_u16() as usize,
                    }))
                }
                12 => {
                    constant_pool_infos.push(ConstantPoolInfo::NameAndType(ConstantPoolNameAndTypeInfo {
                        tag,
                        name_index: bytes_reader.read_u16() as usize,
                        descriptor_index: bytes_reader.read_u16() as usize,
                    }))
                }
                15 => {
                    constant_pool_infos.push(ConstantPoolInfo::MethodKind(MethodKindInfo {
                        tag,
                        reference_kind: bytes_reader.read_u8(),
                        reference_index: bytes_reader.read_u16() as usize,
                    }))
                }
                17 => {
                    constant_pool_infos.push(ConstantPoolInfo::Dynamic(DynamicInfo {
                        tag,
                        bootstrap_method_attr_index: bytes_reader.read_u16() as usize,
                        name_and_type_index: bytes_reader.read_u16() as usize,
                    }))
                }
                18 => {
                    constant_pool_infos.push(ConstantPoolInfo::InvokeDynamic(InvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index: bytes_reader.read_u16() as usize,
                        name_and_type_index: bytes_reader.read_u16() as usize,
                    }))
                }

                _ => {
                    unimplemented!("jjjj")
                }
            }
        }

        let flags = ClassFlags::new(bytes_reader.read_u16());
        let this_class = bytes_reader.read_u16() as usize;
        let super_class = bytes_reader.read_u16() as usize;

        let interfaces_count: usize = bytes_reader.read_u16() as usize;
        let mut interfaces_indexes: Vec<usize> = Vec::with_capacity(interfaces_count);

        for _ in 0..interfaces_count {
            interfaces_indexes.push(bytes_reader.read_u16() as usize);
        }

        let fields_count: usize = bytes_reader.read_u16() as usize;
        let mut fields: Vec<FieldInfo> = Vec::with_capacity(fields_count);

        for _n in 0..fields_count {
            let access_flags = bytes_reader.read_u16();
            let name_index = bytes_reader.read_u16() as usize;
            let name = Self::get_name_info(&constant_pool_infos, &name_index);
            let descriptor_index = bytes_reader.read_u16() as usize;

            let attributes_info = Self::read_attributes(&mut bytes_reader, &constant_pool_infos);

            fields.push(FieldInfo {
                access_flags,
                name_index,
                name,
                descriptor_index,
                attributes_info,
            });
        }

        let methods_count = bytes_reader.read_u16() as usize;
        let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_count);

        for _ in 0..methods_count {
            let access_flags = bytes_reader.read_u16();
            let name_index = bytes_reader.read_u16() as usize;
            let name = Self::get_name_info(&constant_pool_infos, &name_index);
            let descriptor_index = bytes_reader.read_u16() as usize;
            let attributes_info = Self::read_attributes(&mut bytes_reader, &constant_pool_infos);

            methods.push(MethodInfo {
                access_flags,
                name_index,
                name,
                descriptor_index,
                attributes_info,
            });
        }


        Ok(JavaClass {
            prelude,
            major,
            minor,
            constant_pool_count,
            constant_pool_infos,
            flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces_indexes,
            fields,
            methods,
        })
    }

    fn read_attributes(bytes_reader: &mut BytesReader, constant_pool_infos: &Vec<ConstantPoolInfo>) -> Vec<AttributeInfo> {
        let attributes_count = bytes_reader.read_u16() as usize;
        debug!("It has {} attributes", attributes_count);
        let mut attributes_info: Vec<AttributeInfo> = Vec::with_capacity(attributes_count);

        for n in 0..attributes_count {
            if let Some(attribute_info) = Self::read_attribute(bytes_reader, constant_pool_infos, n, attributes_count) {
                attributes_info.push(attribute_info);
            }
        }
        attributes_info
    }

    fn read_attribute(bytes_reader: &mut BytesReader, constant_pool_infos: &Vec<ConstantPoolInfo>, n: usize, attributes_count: usize) -> Option<AttributeInfo> {
        let attribute_name_index = bytes_reader.read_u16() as usize;
        let attribute_name = Self::get_name_info(&constant_pool_infos, &attribute_name_index);
        let attribute_length = bytes_reader.read_u32() as usize;
        debug!("attribute {n}/{attributes_count} has name {attribute_name} and length {attribute_length}");
        bytes_reader.begin_read_block();

        match attribute_name.as_str() {
            "ConstantValue" => {
                let constant_value_index = bytes_reader.read_u16() as usize;
                bytes_reader.assert_read(attribute_length);
                Some(AttributeInfo {
                    attribute_name_index,
                    attribute: Attribute::ConstantValue(AttributeConstantValue { constant_value_index }),
                })
            }
            "Code" => {
                let max_stack = bytes_reader.read_u16() as usize;
                let max_locals = bytes_reader.read_u16() as usize;
                let code_length = bytes_reader.read_u32() as usize;
                let code = bytes_reader.read_bytes(code_length);
                let exception_table_length = bytes_reader.read_u16() as usize;
                let mut exceptions: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length);

                for _ in 0..exception_table_length {
                    let start_pc = bytes_reader.read_u16() as usize;
                    let end_pc = bytes_reader.read_u16() as usize;
                    let handler_pc = bytes_reader.read_u16() as usize;
                    let catch_pc = bytes_reader.read_u16() as usize;

                    exceptions.push(ExceptionTableEntry {
                        start_pc,
                        end_pc,
                        handler_pc,
                        catch_pc,
                    })
                }
                let attributes = Self::read_attributes(bytes_reader, constant_pool_infos);
                bytes_reader.assert_read(attribute_length);
                Some(AttributeInfo {
                    attribute_name_index,
                    attribute: Attribute::Code(AttributeCode {
                        attribute_name_index,
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exceptions,
                        attributes,
                    }),
                })
            }
            "StackMapTable" => {
                unimplemented!("StackMapTable")
            }
            "BootstrapMethods" => {
                unimplemented!("BootstrapMethods")
            }
            "NestHost" => {
                unimplemented!("NestHost")
            }
            "NestMembers" => {
                unimplemented!("NestMembers")
            }
            "PermittedSubclasses" => {
                unimplemented!("PermittedSubclasses")
            }
            "LineNumberTable" => {
                let line_number_table_length = bytes_reader.read_u16() as usize;
                let mut line_number_entries: Vec<LineNumberTableEntry> = Vec::with_capacity(line_number_table_length);
                for _ in 0..line_number_table_length {
                    let start_pc = bytes_reader.read_u16() as usize;
                    let line_number = bytes_reader.read_u16() as usize;
                    line_number_entries.push(LineNumberTableEntry {
                        start_pc,
                        line_number,
                    })
                }
                bytes_reader.assert_read(attribute_length);
                Some(AttributeInfo {
                    attribute_name_index,
                    attribute: Attribute::LineNumberTable(AttributeLineNumberTable {
                        attribute_name_index,
                        attribute_name,
                        attribute_length,
                        line_number_entries,
                    }),
                })
            }
            _ => {
                debug!("Skipping {attribute_length} bytes for {attribute_name}");
                bytes_reader.consume(attribute_length);
                bytes_reader.assert_read(attribute_length);
                None
            }
        }
    }

    fn get_name_info(constant_pool_infos: &Vec<ConstantPoolInfo>, attribute_name_index: &usize) -> String {
        let info = &constant_pool_infos[attribute_name_index - 1];
        if let ConstantPoolInfo::Utf8(i) = info {
            (&i).string.clone()
        } else {
            panic!("Name is wrong")
        }
    }
}

#[test]
fn test_version_resolves() {
    assert_eq!("1.1", JavaClass::resolve_version(45).unwrap());
    assert_eq!("1.2", JavaClass::resolve_version(46).unwrap());
    assert_eq!("19", JavaClass::resolve_version(63).unwrap());
    assert_eq!("20", JavaClass::resolve_version(64).unwrap());
    assert!(JavaClass::resolve_version(65).is_err_and(|err| err == "Major version 65 is not supported"));
}

#[test]
fn test_check_prelude() {
    assert!(JavaClass::check_prelude(0xCAFEBABE).is_ok());
    assert!(JavaClass::check_prelude(0xCAFEBABB).is_err_and(|err| err == "The prelude is 0xCAFEBABB instead of 0xCAFEBABE"));
}
