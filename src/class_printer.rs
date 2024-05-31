use crate::{Attribute, ConstantPoolInfo, ConstantPoolNameAndTypeInfo, ConstantPoolUtf8Info, JavaClass};
use text_colorizer::*;

pub struct ClassPrinter(JavaClass);

impl ClassPrinter {
    pub fn new(java_class: JavaClass) -> Self {
        Self(java_class)
    }

    pub fn print(self) {
        let class = self.0;
        println!("{}: 0x{:X}", "Prelude".bold().blue(), class.prelude);
        println!("{}: {}.{}", "Version".bold().blue(), class.major, class.minor);
        println!("{}: {}", "Constant pool count".bold().blue(), class.constant_pool_count);
        Self::print_contants_pool(&class);
        println!("Flags: \n  public: {}\n  final: {}\n  super: {}\n  interface: {}\n  abstract: {}\n  synthetic: {}\n  annotation: {}\n  enum: {}\n  module: {}",
                 class.flags.is_public(),
                 class.flags.is_final(),
                 class.flags.is_super(),
                 class.flags.is_interface(),
                 class.flags.is_abstract(),
                 class.flags.is_synthetic(),
                 class.flags.is_annotation(),
                 class.flags.is_enum(),
                 class.flags.is_module());


        if let ConstantPoolInfo::Class(class_info) = &class.constant_pool_infos[class.this_class - 1] {
            if let ConstantPoolInfo::Utf8(class_name) = &class.constant_pool_infos[class_info.name_index - 1] {
                println!("This class: {} ({})", class_name.string, class.this_class);
            }
        }
        if let ConstantPoolInfo::Class(class_info) = &class.constant_pool_infos[class.super_class - 1] {
            if class_info.name_index > 0 {
                if let ConstantPoolInfo::Utf8(class_name) = &class.constant_pool_infos[class_info.name_index - 1] {
                    println!("Super class: {} ({})", class_name.string, class.super_class);
                }
            }
        }

        println!("Number of interfaces: {}", class.interfaces_count);

        for interfaces_index in &class.interfaces_indexes {
            if let Some(class_constant) = Self::get_class_name(&class, &interfaces_index) {
                println!("  {} ({})", class_constant.string, interfaces_index);
            }
        }

        println!("Number of fields: {}", class.fields.len());

        for field in class.fields {
            println!("  {}", field.name);
        }

        println!("Number of methods: {}", class.methods.len());

        for method in class.methods {
            println!("  {} at {}", method.name, method.descriptor_index);
            for attribute in method.attributes_info {
                match attribute.attribute {
                    Attribute::ConstantValue(_) => {}
                    Attribute::Code(c) => {
                        c.print();
                    }
                    Attribute::Exceptions(_) => {}
                    Attribute::SourceFile(_) => {}
                    Attribute::LineNumberTable(_) => {}
                    Attribute::LocalVariableTable(_) => {}
                    Attribute::InnerClasses(_) => {}
                    Attribute::Synthetic(_) => {}
                    Attribute::Deprecated(_) => {}
                }
            }
        }
    }

    fn constant_name_coloured(constant_name: &str) -> ColoredString {
        constant_name.red()
    }

    fn print_contants_pool(class: &JavaClass) {
        for (index, constant_pool_info) in class.constant_pool_infos.iter().enumerate() {
            let const_index = index + 1;
            match constant_pool_info {
                ConstantPoolInfo::Utf8(utf8_info) => {
                    println!("  {}. {}: {}", const_index, Self::constant_name_coloured("Utf8"), utf8_info.string)
                }
                ConstantPoolInfo::Integer(integer_info) => {
                    println!("  {}. {}: {}", const_index, Self::constant_name_coloured("Integer"), integer_info.value)
                }
                ConstantPoolInfo::Float(float_info) => {
                    println!("  {}. {}: {}", const_index, Self::constant_name_coloured("Float"), float_info.value)
                }
                ConstantPoolInfo::Long(long_info) => {
                    println!("  {}. {}: {}", const_index, Self::constant_name_coloured("Long"), long_info.value)
                }
                ConstantPoolInfo::Double(double_info) => {
                    println!("  {}. {}: {}", const_index, Self::constant_name_coloured("Double"), double_info.value)
                }
                ConstantPoolInfo::Class(value) => {
                    let name = Self::get_name_constant(class, &value.name_index).expect("Name should exist");
                    println!("  {}. {}: {} ({})", const_index, Self::constant_name_coloured("Class"), name.string.bold(), value.name_index)
                }
                ConstantPoolInfo::String(string_info) => {
                    let name = Self::get_name_constant(class, &string_info.name_index).expect("Name should exist");
                    println!("  {}. {}: \"{}\" ({})", const_index, Self::constant_name_coloured("String"), name.string.bold(), string_info.name_index)
                }
                ConstantPoolInfo::FieldRef(field_ref_info) => {
                    let class_name = Self::get_class_name(class, &field_ref_info.class_index).expect("Name should exist");
                    let name_and_type = Self::get_name_and_type_constant(class, &field_ref_info.name_and_type_index).expect("Name should exist");
                    let field_name = Self::get_name_constant(class, &name_and_type.name_index).expect("name for method doesn't doesn't exist");
                    let field_type_name = Self::get_name_constant(class, &name_and_type.descriptor_index).expect("name for descriptor doesn't exist");
                    println!("  {}. {}: {} {}.{}, class index ({}), name and type index ({})",
                             const_index,
                             Self::constant_name_coloured("Field ref"),
                             field_type_name.string.bold(),
                             class_name.string.bold(),
                             field_name.string.bold(),
                             field_ref_info.class_index,
                             field_ref_info.name_and_type_index)
                }
                ConstantPoolInfo::MethodRef(method_ref_info) => {
                    let class_name = Self::get_class_name(class, &method_ref_info.class_index).expect("Name should exist");
                    let name_and_type = Self::get_name_and_type_constant(class, &method_ref_info.name_and_type_index).expect("Name should exist");
                    let method_name = Self::get_name_constant(class, &name_and_type.name_index).expect("name for method doesn't doesn't exist");
                    let method_descriptor = Self::get_name_constant(class, &name_and_type.descriptor_index).expect("name for descriptor doesn't exist");
                    println!("  {}. {}: {}.{} {}, class index: {}, name and type index: {}",
                             const_index,
                             Self::constant_name_coloured("Method ref"),
                             class_name.string,
                             method_name.string.bold(),
                             method_descriptor.string,
                             method_ref_info.class_index,
                             method_ref_info.name_and_type_index)
                }
                ConstantPoolInfo::InterfaceMethodRef(interface_and_method_ref_info) => {
                    println!("  {}. {}: name index({}), descriptor index ({})",
                             const_index,
                             Self::constant_name_coloured("Interface method ref"),
                             interface_and_method_ref_info.class_index,
                             interface_and_method_ref_info.name_and_type_index)
                }
                ConstantPoolInfo::NameAndType(name_and_type_info) => {
                    let name = Self::get_name_constant(class, &name_and_type_info.name_index).expect("name for method doesn't doesn't exist");
                    let descriptor = Self::get_name_constant(class, &name_and_type_info.descriptor_index).expect("name for descriptor doesn't exist");
                    println!("  {}. {}: {}{}, name index: {}, descriptor index: {}",
                             const_index,
                             Self::constant_name_coloured("Name and type"),
                             name.string.bold(),
                             descriptor.string.bold(),
                             name_and_type_info.name_index,
                             name_and_type_info.descriptor_index)
                }
                ConstantPoolInfo::MethodKind(method_kind) => {
                    println!("  {}. {}: reference_kind({}), reference_index({})",
                             const_index,
                             Self::constant_name_coloured("Method kind"),
                             method_kind.reference_kind,
                             method_kind.reference_index)
                }
                ConstantPoolInfo::Dynamic(dynamic) => {
                    println!("  {}. {}: bootstrap_method_attr_index({}), name_and_type_index({})",
                             const_index,
                             Self::constant_name_coloured("Dynamic"),
                             dynamic.bootstrap_method_attr_index,
                             dynamic.name_and_type_index)
                }
                ConstantPoolInfo::InvokeDynamic(invoke_dynamic) => {
                    let name_and_type = Self::get_name_and_type_constant(class, &invoke_dynamic.name_and_type_index).expect("Name should exist");
                    let method_name = Self::get_name_constant(class, &name_and_type.name_index).expect("name for method doesn't doesn't exist");
                    let method_descriptor = Self::get_name_constant(class, &name_and_type.descriptor_index).expect("name for descriptor doesn't exist");
                    println!("  {}. {}: {} {} bootstrap_method_attr_index({}), name_and_type_index({})",
                             const_index,
                             Self::constant_name_coloured("Invoke dynamic"),
                             method_name.string.bold(),
                             method_descriptor.string.bold(),
                             invoke_dynamic.bootstrap_method_attr_index,
                             invoke_dynamic.name_and_type_index)
                }
            }
        }
    }

    fn get_class_name<'a>(class: &'a JavaClass, class_index: &usize) -> Option<&'a ConstantPoolUtf8Info> {
        if let ConstantPoolInfo::Class(class_info) = &class.constant_pool_infos[class_index - 1] {
            Self::get_name_constant(class, &class_info.name_index)
        } else {
            None
        }
    }

    fn get_name_constant<'a>(class: &'a JavaClass, name_index: &usize) -> Option<&'a ConstantPoolUtf8Info> {
        if let ConstantPoolInfo::Utf8(class_name) = &class.constant_pool_infos[name_index - 1] {
            Some(class_name)
        } else {
            None
        }
    }

    fn get_name_and_type_constant<'a>(class: &'a JavaClass, name_index: &usize) -> Option<&'a ConstantPoolNameAndTypeInfo> {
        if let ConstantPoolInfo::NameAndType(name_and_type) = &class.constant_pool_infos[name_index - 1] {
            Some(name_and_type)
        } else {
            None
        }
    }
}
