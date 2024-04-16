#![allow(dead_code)]
use std::convert::AsRef;
use std::{collections::BTreeMap, str::FromStr};

use graphql_parser::schema::{self as ps, Text};
use thiserror::Error;

use crate::{data::value::Word, prelude::s};

struct Compactor;

impl Compactor {
    fn word<'a, T: Text<'a>>(&self, t: T::Value) -> Word {
        Word::from(t.as_ref())
    }
}

trait Compact<T>: Sized {
    fn compact(value: T, cpt: &mut Compactor) -> Self;
}

fn word<'a, T: Text<'a>>(t: T::Value) -> Word {
    Word::from(t.as_ref())
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub definitions: Vec<Definition>,
}

impl From<s::Document> for Document {
    fn from(doc: s::Document) -> Self {
        let definitions = doc.definitions.into_iter().map(Definition::from).collect();
        Self { definitions }
    }
}

impl Compact<s::Document> for Document {
    fn compact(doc: s::Document, cpt: &mut Compactor) -> Self {
        let definitions = doc
            .definitions
            .into_iter()
            .map(|def| Definition::compact(def, cpt))
            .collect();
        Self { definitions }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    SchemaDefinition(SchemaDefinition),
    TypeDefinition(TypeDefinition),
    TypeExtension(TypeExtension),
    DirectiveDefinition(DirectiveDefinition),
}

impl From<s::Definition> for Definition {
    fn from(def: s::Definition) -> Self {
        match def {
            s::Definition::SchemaDefinition(def) => {
                Definition::SchemaDefinition(SchemaDefinition::from(def))
            }
            s::Definition::TypeDefinition(def) => {
                Definition::TypeDefinition(TypeDefinition::from(def))
            }
            s::Definition::TypeExtension(def) => {
                Definition::TypeExtension(TypeExtension::from(def))
            }
            s::Definition::DirectiveDefinition(def) => {
                Definition::DirectiveDefinition(DirectiveDefinition::from(def))
            }
        }
    }
}

impl Compact<s::Definition> for Definition {
    fn compact(def: s::Definition, cpt: &mut Compactor) -> Self {
        match def {
            s::Definition::SchemaDefinition(def) => {
                Definition::SchemaDefinition(SchemaDefinition::compact(def, cpt))
            }
            s::Definition::TypeDefinition(def) => {
                Definition::TypeDefinition(TypeDefinition::compact(def, cpt))
            }
            s::Definition::TypeExtension(def) => {
                Definition::TypeExtension(TypeExtension::compact(def, cpt))
            }
            s::Definition::DirectiveDefinition(def) => {
                Definition::DirectiveDefinition(DirectiveDefinition::compact(def, cpt))
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchemaDefinition {
    pub directives: Vec<Directive>,
    pub query: Option<Word>,
    pub mutation: Option<Word>,
    pub subscription: Option<Word>,
}

impl<'a, T: Text<'a>> From<s::SchemaDefinition<'a, T>> for SchemaDefinition {
    fn from(def: s::SchemaDefinition<'a, T>) -> Self {
        let directives = def.directives.into_iter().map(Directive::from).collect();
        let query = def.query.map(word::<'a, T>);
        let mutation = def.mutation.map(word::<'a, T>);
        let subscription = def.subscription.map(word::<'a, T>);
        Self {
            directives,
            query,
            mutation,
            subscription,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::SchemaDefinition<'a, T>> for SchemaDefinition {
    fn compact(def: s::SchemaDefinition<'a, T>, cpt: &mut Compactor) -> Self {
        let directives = def
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let query = def.query.map(|q| cpt.word::<T>(q));
        let mutation = def.mutation.map(|m| cpt.word::<T>(m));
        let subscription = def.subscription.map(|s| cpt.word::<T>(s));
        Self {
            directives,
            query,
            mutation,
            subscription,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition {
    Scalar(ScalarType),
    Object(ObjectType),
    Interface(InterfaceType),
    Union(UnionType),
    Enum(EnumType),
    InputObject(InputObjectType),
}

impl From<s::TypeDefinition> for TypeDefinition {
    fn from(def: s::TypeDefinition) -> Self {
        match def {
            s::TypeDefinition::Scalar(def) => TypeDefinition::Scalar(ScalarType::from(def)),
            s::TypeDefinition::Object(def) => TypeDefinition::Object(ObjectType::from(def)),
            s::TypeDefinition::Interface(def) => {
                TypeDefinition::Interface(InterfaceType::from(def))
            }
            s::TypeDefinition::Union(def) => TypeDefinition::Union(UnionType::from(def)),
            s::TypeDefinition::Enum(def) => TypeDefinition::Enum(EnumType::from(def)),
            s::TypeDefinition::InputObject(def) => {
                TypeDefinition::InputObject(InputObjectType::from(def))
            }
        }
    }
}

impl Compact<s::TypeDefinition> for TypeDefinition {
    fn compact(def: s::TypeDefinition, cpt: &mut Compactor) -> Self {
        match def {
            s::TypeDefinition::Scalar(def) => TypeDefinition::Scalar(ScalarType::compact(def, cpt)),
            s::TypeDefinition::Object(def) => TypeDefinition::Object(ObjectType::compact(def, cpt)),
            s::TypeDefinition::Interface(def) => {
                TypeDefinition::Interface(InterfaceType::compact(def, cpt))
            }
            s::TypeDefinition::Union(def) => TypeDefinition::Union(UnionType::compact(def, cpt)),
            s::TypeDefinition::Enum(def) => TypeDefinition::Enum(EnumType::compact(def, cpt)),
            s::TypeDefinition::InputObject(def) => {
                TypeDefinition::InputObject(InputObjectType::compact(def, cpt))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExtension {
    Scalar(ScalarTypeExtension),
    Object(ObjectTypeExtension),
    Interface(InterfaceTypeExtension),
    Union(UnionTypeExtension),
    Enum(EnumTypeExtension),
    InputObject(InputObjectTypeExtension),
}

impl<'a, T: Text<'a>> From<s::TypeExtension<'a, T>> for TypeExtension {
    fn from(ext: s::TypeExtension<'a, T>) -> Self {
        match ext {
            s::TypeExtension::Scalar(ext) => TypeExtension::Scalar(ScalarTypeExtension::from(ext)),
            s::TypeExtension::Object(ext) => TypeExtension::Object(ObjectTypeExtension::from(ext)),
            s::TypeExtension::Interface(ext) => {
                TypeExtension::Interface(InterfaceTypeExtension::from(ext))
            }
            s::TypeExtension::Union(ext) => TypeExtension::Union(UnionTypeExtension::from(ext)),
            s::TypeExtension::Enum(ext) => TypeExtension::Enum(EnumTypeExtension::from(ext)),
            s::TypeExtension::InputObject(ext) => {
                TypeExtension::InputObject(InputObjectTypeExtension::from(ext))
            }
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::TypeExtension<'a, T>> for TypeExtension {
    fn compact(ext: s::TypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        match ext {
            s::TypeExtension::Scalar(ext) => {
                TypeExtension::Scalar(ScalarTypeExtension::compact(ext, cpt))
            }
            s::TypeExtension::Object(ext) => {
                TypeExtension::Object(ObjectTypeExtension::compact(ext, cpt))
            }
            s::TypeExtension::Interface(ext) => {
                TypeExtension::Interface(InterfaceTypeExtension::compact(ext, cpt))
            }
            s::TypeExtension::Union(ext) => {
                TypeExtension::Union(UnionTypeExtension::compact(ext, cpt))
            }
            s::TypeExtension::Enum(ext) => {
                TypeExtension::Enum(EnumTypeExtension::compact(ext, cpt))
            }
            s::TypeExtension::InputObject(ext) => {
                TypeExtension::InputObject(InputObjectTypeExtension::compact(ext, cpt))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
// we use i64 as a reference implementation: graphql-js thinks even 32bit
// integers is enough. We might consider lift this limit later though
pub struct Number(pub(crate) i64);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Variable(Word),
    Int(Number),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(Word),
    List(Vec<Value>),
    Object(BTreeMap<Word, Value>),
}

impl<'a, T: Text<'a>> From<ps::Value<'a, T>> for Value {
    fn from(val: ps::Value<'a, T>) -> Self {
        use ps::Value::*;

        match val {
            Variable(name) => Value::Variable(word::<T>(name)),
            Int(num) => Value::Int(Number(num.as_i64().unwrap())),
            Float(num) => Value::Float(num),
            String(s) => Value::String(s.to_string()),
            Boolean(b) => Value::Boolean(b),
            Null => Value::Null,
            Enum(name) => Value::Enum(word::<T>(name)),
            List(list) => Value::List(list.into_iter().map(Value::from).collect()),
            Object(obj) => Value::Object(
                obj.into_iter()
                    .map(|(k, v)| (word::<T>(k), Value::from(v)))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType {
    pub name: Word,
    pub directives: Vec<Directive>,
}

impl ScalarType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
        }
    }
}

impl From<s::ScalarType> for ScalarType {
    fn from(scalar: s::ScalarType) -> Self {
        let name = word::<'static, String>(scalar.name);
        let directives = scalar.directives.into_iter().map(Directive::from).collect();
        Self { name, directives }
    }
}

impl Compact<s::ScalarType> for ScalarType {
    fn compact(scalar: s::ScalarType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(scalar.name);
        let directives = scalar
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        Self { name, directives }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
}

impl ScalarTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::ScalarTypeExtension<'a, T>> for ScalarTypeExtension {
    fn from(ext: s::ScalarTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        Self { name, directives }
    }
}

impl<'a, T: Text<'a>> Compact<s::ScalarTypeExtension<'a, T>> for ScalarTypeExtension {
    fn compact(ext: s::ScalarTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        Self { name, directives }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub name: Word,
    pub implements_interfaces: Vec<Word>,
    pub directives: Vec<Directive>,
    pub fields: Vec<Field>,
}

impl ObjectType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

impl From<s::ObjectType> for ObjectType {
    fn from(obj: s::ObjectType) -> Self {
        let name = word::<String>(obj.name);
        let implements_interfaces = obj
            .implements_interfaces
            .into_iter()
            .map(word::<String>)
            .collect();
        let directives = obj.directives.into_iter().map(Directive::from).collect();
        let fields = obj.fields.into_iter().map(Field::from).collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

impl Compact<s::ObjectType> for ObjectType {
    fn compact(obj: s::ObjectType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(obj.name);
        let implements_interfaces = obj
            .implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<String>(name))
            .collect();
        let directives = obj
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = obj
            .fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectTypeExtension {
    pub name: Word,
    pub implements_interfaces: Vec<Word>,
    pub directives: Vec<Directive>,
    pub fields: Vec<Field>,
}

impl ObjectTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::ObjectTypeExtension<'a, T>> for ObjectTypeExtension {
    fn from(ext: s::ObjectTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let implements_interfaces = ext
            .implements_interfaces
            .into_iter()
            .map(word::<T>)
            .collect();
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        let fields = ext.fields.into_iter().map(Field::from).collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::ObjectTypeExtension<'a, T>> for ObjectTypeExtension {
    fn compact(ext: s::ObjectTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let implements_interfaces = ext
            .implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = ext
            .fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: Word,
    pub arguments: Vec<(Word, Value)>,
}

impl<'a, T: Text<'a>> From<ps::Directive<'a, T>> for Directive {
    fn from(dir: ps::Directive<'a, T>) -> Self {
        let name = word::<T>(dir.name);
        let arguments = dir
            .arguments
            .into_iter()
            .map(|(k, v)| (word::<T>(k), Value::from(v)))
            .collect();
        Self { name, arguments }
    }
}

impl<'a, T: Text<'a>> Compact<ps::Directive<'a, T>> for Directive {
    fn compact(dir: ps::Directive<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(dir.name);
        let arguments = dir
            .arguments
            .into_iter()
            .map(|(k, v)| (cpt.word::<T>(k), Value::from(v)))
            .collect();
        Self { name, arguments }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    NamedType(Word),
    ListType(Box<Type>),
    NonNullType(Box<Type>),
}

impl<'a, T: Text<'a>> From<ps::Type<'a, T>> for Type {
    fn from(ty: ps::Type<'a, T>) -> Self {
        match ty {
            ps::Type::NamedType(name) => Type::NamedType(word::<T>(name)),
            ps::Type::ListType(ty) => Type::ListType(Box::new(Type::from(*ty))),
            ps::Type::NonNullType(ty) => Type::NonNullType(Box::new(Type::from(*ty))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Word,
    pub arguments: Vec<InputValue>,
    pub field_type: Type,
    pub directives: Vec<Directive>,
}

impl<'a, T: Text<'a>> From<ps::Field<'a, T>> for Field {
    fn from(field: ps::Field<'a, T>) -> Self {
        let name = word::<T>(field.name);
        let arguments = field.arguments.into_iter().map(InputValue::from).collect();
        let field_type = Type::from(field.field_type);
        let directives = field.directives.into_iter().map(Directive::from).collect();
        Self {
            name,
            arguments,
            field_type,
            directives,
        }
    }
}

impl<'a, T: Text<'a>> Compact<ps::Field<'a, T>> for Field {
    fn compact(field: ps::Field<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(field.name);
        let arguments = field
            .arguments
            .into_iter()
            .map(|arg| InputValue::compact(arg, cpt))
            .collect();
        let field_type = Type::from(field.field_type);
        let directives = field
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        Self {
            name,
            arguments,
            field_type,
            directives,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputValue {
    pub name: Word,
    pub value_type: Type,
    pub default_value: Option<Value>,
    pub directives: Vec<Directive>,
}

impl<'a, T: Text<'a>> From<ps::InputValue<'a, T>> for InputValue {
    fn from(val: ps::InputValue<'a, T>) -> Self {
        let name = word::<T>(val.name);
        let value_type = Type::from(val.value_type);
        let default_value = val.default_value.map(Value::from);
        let directives = val.directives.into_iter().map(Directive::from).collect();
        Self {
            name,
            value_type,
            default_value,
            directives,
        }
    }
}

impl<'a, T: Text<'a>> Compact<ps::InputValue<'a, T>> for InputValue {
    fn compact(val: ps::InputValue<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(val.name);
        let value_type = Type::from(val.value_type);
        let default_value = val.default_value.map(Value::from);
        let directives = val
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        Self {
            name,
            value_type,
            default_value,
            directives,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType {
    pub name: Word,
    pub implements_interfaces: Vec<Word>,
    pub directives: Vec<Directive>,
    pub fields: Vec<Field>,
}

impl InterfaceType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

impl From<s::InterfaceType> for InterfaceType {
    fn from(int: s::InterfaceType) -> Self {
        let name = word::<String>(int.name);
        let implements_interfaces = int
            .implements_interfaces
            .into_iter()
            .map(word::<String>)
            .collect();
        let directives = int.directives.into_iter().map(Directive::from).collect();
        let fields = int.fields.into_iter().map(Field::from).collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

impl Compact<s::InterfaceType> for InterfaceType {
    fn compact(int: s::InterfaceType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(int.name);
        let implements_interfaces = int
            .implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<String>(name))
            .collect();
        let directives = int
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = int
            .fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceTypeExtension {
    pub name: Word,
    pub implements_interfaces: Vec<Word>,
    pub directives: Vec<Directive>,
    pub fields: Vec<Field>,
}

impl InterfaceTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::InterfaceTypeExtension<'a, T>> for InterfaceTypeExtension {
    fn from(ext: s::InterfaceTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let implements_interfaces = ext
            .implements_interfaces
            .into_iter()
            .map(word::<T>)
            .collect();
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        let fields = ext.fields.into_iter().map(Field::from).collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::InterfaceTypeExtension<'a, T>> for InterfaceTypeExtension {
    fn compact(ext: s::InterfaceTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let implements_interfaces = ext
            .implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = ext
            .fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UnionType {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub types: Vec<Word>,
}

impl UnionType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

impl From<s::UnionType> for UnionType {
    fn from(union: s::UnionType) -> Self {
        let name = word::<String>(union.name);
        let directives = union.directives.into_iter().map(Directive::from).collect();
        let types = union.types.into_iter().map(word::<String>).collect();
        Self {
            name,
            directives,
            types,
        }
    }
}

impl Compact<s::UnionType> for UnionType {
    fn compact(union: s::UnionType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(union.name);
        let directives = union
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let types = union
            .types
            .into_iter()
            .map(|t| cpt.word::<String>(t))
            .collect();
        Self {
            name,
            directives,
            types,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub types: Vec<Word>,
}

impl UnionTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::UnionTypeExtension<'a, T>> for UnionTypeExtension {
    fn from(ext: s::UnionTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        let types = ext.types.into_iter().map(word::<T>).collect();
        Self {
            name,
            directives,
            types,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::UnionTypeExtension<'a, T>> for UnionTypeExtension {
    fn compact(ext: s::UnionTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let types = ext.types.into_iter().map(|t| cpt.word::<T>(t)).collect();
        Self {
            name,
            directives,
            types,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub values: Vec<EnumValue>,
}

impl EnumType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

impl From<s::EnumType> for EnumType {
    fn from(enum_type: s::EnumType) -> Self {
        let name = word::<String>(enum_type.name);
        let directives = enum_type
            .directives
            .into_iter()
            .map(Directive::from)
            .collect();
        let values = enum_type.values.into_iter().map(EnumValue::from).collect();
        Self {
            name,
            directives,
            values,
        }
    }
}

impl Compact<s::EnumType> for EnumType {
    fn compact(enum_type: s::EnumType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(enum_type.name);
        let directives = enum_type
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let values = enum_type
            .values
            .into_iter()
            .map(|val| EnumValue::compact(val, cpt))
            .collect();
        Self {
            name,
            directives,
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    pub name: Word,
    pub directives: Vec<Directive>,
}

impl EnumValue {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<ps::EnumValue<'a, T>> for EnumValue {
    fn from(val: ps::EnumValue<'a, T>) -> Self {
        let name = word::<T>(val.name);
        let directives = val.directives.into_iter().map(Directive::from).collect();
        Self { name, directives }
    }
}

impl<'a, T: Text<'a>> Compact<ps::EnumValue<'a, T>> for EnumValue {
    fn compact(val: ps::EnumValue<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(val.name);
        let directives = val
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        Self { name, directives }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub values: Vec<EnumValue>,
}

impl EnumTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::EnumTypeExtension<'a, T>> for EnumTypeExtension {
    fn from(ext: s::EnumTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        let values = ext.values.into_iter().map(EnumValue::from).collect();
        Self {
            name,
            directives,
            values,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::EnumTypeExtension<'a, T>> for EnumTypeExtension {
    fn compact(ext: s::EnumTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let values = ext
            .values
            .into_iter()
            .map(|val| EnumValue::compact(val, cpt))
            .collect();
        Self {
            name,
            directives,
            values,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectType {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub fields: Vec<InputValue>,
}

impl InputObjectType {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

impl From<s::InputObjectType> for InputObjectType {
    fn from(obj: s::InputObjectType) -> Self {
        let name = word::<String>(obj.name);
        let directives = obj.directives.into_iter().map(Directive::from).collect();
        let fields = obj.fields.into_iter().map(InputValue::from).collect();
        Self {
            name,
            directives,
            fields,
        }
    }
}

impl Compact<s::InputObjectType> for InputObjectType {
    fn compact(obj: s::InputObjectType, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(obj.name);
        let directives = obj
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = obj
            .fields
            .into_iter()
            .map(|field| InputValue::compact(field, cpt))
            .collect();
        Self {
            name,
            directives,
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub fields: Vec<InputValue>,
}

impl InputObjectTypeExtension {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

impl<'a, T: Text<'a>> From<s::InputObjectTypeExtension<'a, T>> for InputObjectTypeExtension {
    fn from(ext: s::InputObjectTypeExtension<'a, T>) -> Self {
        let name = word::<T>(ext.name);
        let directives = ext.directives.into_iter().map(Directive::from).collect();
        let fields = ext.fields.into_iter().map(InputValue::from).collect();
        Self {
            name,
            directives,
            fields,
        }
    }
}

impl<'a, T: Text<'a>> Compact<s::InputObjectTypeExtension<'a, T>> for InputObjectTypeExtension {
    fn compact(ext: s::InputObjectTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<T>(ext.name);
        let directives = ext
            .directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = ext
            .fields
            .into_iter()
            .map(|field| InputValue::compact(field, cpt))
            .collect();
        Self {
            name,
            directives,
            fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectiveLocation {
    // executable
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,

    // type_system
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

impl From<s::DirectiveLocation> for DirectiveLocation {
    fn from(loc: s::DirectiveLocation) -> Self {
        use s::DirectiveLocation::*;
        match loc {
            Query => DirectiveLocation::Query,
            Mutation => DirectiveLocation::Mutation,
            Subscription => DirectiveLocation::Subscription,
            Field => DirectiveLocation::Field,
            FragmentDefinition => DirectiveLocation::FragmentDefinition,
            FragmentSpread => DirectiveLocation::FragmentSpread,
            InlineFragment => DirectiveLocation::InlineFragment,
            Schema => DirectiveLocation::Schema,
            Scalar => DirectiveLocation::Scalar,
            Object => DirectiveLocation::Object,
            FieldDefinition => DirectiveLocation::FieldDefinition,
            ArgumentDefinition => DirectiveLocation::ArgumentDefinition,
            Interface => DirectiveLocation::Interface,
            Union => DirectiveLocation::Union,
            Enum => DirectiveLocation::Enum,
            EnumValue => DirectiveLocation::EnumValue,
            InputObject => DirectiveLocation::InputObject,
            InputFieldDefinition => DirectiveLocation::InputFieldDefinition,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Word,
    pub arguments: Vec<InputValue>,
    pub repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
}

impl DirectiveDefinition {
    pub fn new(name: Word) -> Self {
        Self {
            name,
            arguments: vec![],
            repeatable: false,
            locations: vec![],
        }
    }
}

impl From<s::DirectiveDefinition> for DirectiveDefinition {
    fn from(def: s::DirectiveDefinition) -> Self {
        let name = word::<String>(def.name);
        let arguments = def.arguments.into_iter().map(InputValue::from).collect();
        let repeatable = def.repeatable;
        let locations = def
            .locations
            .into_iter()
            .map(DirectiveLocation::from)
            .collect();
        Self {
            name,
            arguments,
            repeatable,
            locations,
        }
    }
}

impl Compact<s::DirectiveDefinition> for DirectiveDefinition {
    fn compact(def: s::DirectiveDefinition, cpt: &mut Compactor) -> Self {
        let name = cpt.word::<String>(def.name);
        let arguments = def
            .arguments
            .into_iter()
            .map(|arg| InputValue::compact(arg, cpt))
            .collect();
        let repeatable = def.repeatable;
        let locations = def
            .locations
            .into_iter()
            .map(DirectiveLocation::from)
            .collect();
        Self {
            name,
            arguments,
            repeatable,
            locations,
        }
    }
}

impl DirectiveLocation {
    /// Returns GraphQL syntax compatible name of the directive
    pub fn as_str(&self) -> &'static str {
        use self::DirectiveLocation::*;
        match self {
            Query => "QUERY",
            Mutation => "MUTATION",
            Subscription => "SUBSCRIPTION",
            Field => "FIELD",
            FragmentDefinition => "FRAGMENT_DEFINITION",
            FragmentSpread => "FRAGMENT_SPREAD",
            InlineFragment => "INLINE_FRAGMENT",
            Schema => "SCHEMA",
            Scalar => "SCALAR",
            Object => "OBJECT",
            FieldDefinition => "FIELD_DEFINITION",
            ArgumentDefinition => "ARGUMENT_DEFINITION",
            Interface => "INTERFACE",
            Union => "UNION",
            Enum => "ENUM",
            EnumValue => "ENUM_VALUE",
            InputObject => "INPUT_OBJECT",
            InputFieldDefinition => "INPUT_FIELD_DEFINITION",
        }
    }

    /// Returns `true` if this location is for queries (execution)
    pub fn is_query(&self) -> bool {
        use self::DirectiveLocation::*;
        match *self {
            Query | Mutation | Subscription | Field | FragmentDefinition | FragmentSpread
            | InlineFragment => true,

            Schema | Scalar | Object | FieldDefinition | ArgumentDefinition | Interface | Union
            | Enum | EnumValue | InputObject | InputFieldDefinition => false,
        }
    }

    /// Returns `true` if this location is for schema
    pub fn is_schema(&self) -> bool {
        !self.is_query()
    }
}

#[derive(Debug, Error)]
#[error("invalid directive location")]
pub struct InvalidDirectiveLocation;

impl FromStr for DirectiveLocation {
    type Err = InvalidDirectiveLocation;
    fn from_str(s: &str) -> Result<DirectiveLocation, InvalidDirectiveLocation> {
        use self::DirectiveLocation::*;
        let val = match s {
            "QUERY" => Query,
            "MUTATION" => Mutation,
            "SUBSCRIPTION" => Subscription,
            "FIELD" => Field,
            "FRAGMENT_DEFINITION" => FragmentDefinition,
            "FRAGMENT_SPREAD" => FragmentSpread,
            "INLINE_FRAGMENT" => InlineFragment,
            "SCHEMA" => Schema,
            "SCALAR" => Scalar,
            "OBJECT" => Object,
            "FIELD_DEFINITION" => FieldDefinition,
            "ARGUMENT_DEFINITION" => ArgumentDefinition,
            "INTERFACE" => Interface,
            "UNION" => Union,
            "ENUM" => Enum,
            "ENUM_VALUE" => EnumValue,
            "INPUT_OBJECT" => InputObject,
            "INPUT_FIELD_DEFINITION" => InputFieldDefinition,
            _ => return Err(InvalidDirectiveLocation),
        };

        Ok(val)
    }
}
