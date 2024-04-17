#![allow(dead_code)]
use std::convert::AsRef;
use std::{collections::BTreeMap, str::FromStr};

use graphql_parser::schema::{self as ps, Text};
use thiserror::Error;

use crate::data::value::Word;

struct Compactor;

impl Compactor {
    fn word<'a, T: Text<'a>>(&self, t: T::Value) -> Word {
        Word::from(t.as_ref())
    }
}

trait Compact<T>: Sized {
    fn compact(value: T, cpt: &mut Compactor) -> Self;
}

pub fn compact<'a, T: Text<'a>>(doc: ps::Document<'a, T>) -> Document {
    let mut cpt = Compactor;
    Document::compact(doc, &mut cpt)
}

fn word<'a, T: Text<'a>>(t: T::Value) -> Word {
    Word::from(t.as_ref())
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub definitions: Vec<Definition>,
}

impl<'a, T: Text<'a>> Compact<ps::Document<'a, T>> for Document {
    fn compact(doc: ps::Document<'a, T>, cpt: &mut Compactor) -> Self {
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

impl<'a, T: Text<'a>> Compact<ps::Definition<'a, T>> for Definition {
    fn compact(def: ps::Definition<'a, T>, cpt: &mut Compactor) -> Self {
        match def {
            ps::Definition::SchemaDefinition(def) => {
                Definition::SchemaDefinition(SchemaDefinition::compact(def, cpt))
            }
            ps::Definition::TypeDefinition(def) => {
                Definition::TypeDefinition(TypeDefinition::compact(def, cpt))
            }
            ps::Definition::TypeExtension(def) => {
                Definition::TypeExtension(TypeExtension::compact(def, cpt))
            }
            ps::Definition::DirectiveDefinition(def) => {
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

impl<'a, T: Text<'a>> Compact<ps::SchemaDefinition<'a, T>> for SchemaDefinition {
    fn compact(def: ps::SchemaDefinition<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::SchemaDefinition {
            position: _,
            directives,
            query,
            mutation,
            subscription,
        } = def;
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let query = query.map(|q| cpt.word::<T>(q));
        let mutation = mutation.map(|m| cpt.word::<T>(m));
        let subscription = subscription.map(|s| cpt.word::<T>(s));
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

impl<'a, T: Text<'a>> Compact<ps::TypeDefinition<'a, T>> for TypeDefinition {
    fn compact(def: ps::TypeDefinition<'a, T>, cpt: &mut Compactor) -> Self {
        match def {
            ps::TypeDefinition::Scalar(def) => {
                TypeDefinition::Scalar(ScalarType::compact(def, cpt))
            }
            ps::TypeDefinition::Object(def) => {
                TypeDefinition::Object(ObjectType::compact(def, cpt))
            }
            ps::TypeDefinition::Interface(def) => {
                TypeDefinition::Interface(InterfaceType::compact(def, cpt))
            }
            ps::TypeDefinition::Union(def) => TypeDefinition::Union(UnionType::compact(def, cpt)),
            ps::TypeDefinition::Enum(def) => TypeDefinition::Enum(EnumType::compact(def, cpt)),
            ps::TypeDefinition::InputObject(def) => {
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

impl<'a, T: Text<'a>> Compact<ps::TypeExtension<'a, T>> for TypeExtension {
    fn compact(ext: ps::TypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        use ps::TypeExtension as Ps;
        match ext {
            Ps::Scalar(ext) => TypeExtension::Scalar(ScalarTypeExtension::compact(ext, cpt)),
            Ps::Object(ext) => TypeExtension::Object(ObjectTypeExtension::compact(ext, cpt)),
            Ps::Interface(ext) => {
                TypeExtension::Interface(InterfaceTypeExtension::compact(ext, cpt))
            }
            Ps::Union(ext) => TypeExtension::Union(UnionTypeExtension::compact(ext, cpt)),
            Ps::Enum(ext) => TypeExtension::Enum(EnumTypeExtension::compact(ext, cpt)),
            Ps::InputObject(ext) => {
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
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(Word),
    List(Vec<Value>),
    Object(BTreeMap<Word, Value>),
}

impl<'a, T: Text<'a>> Compact<ps::Value<'a, T>> for Value {
    fn compact(value: ps::Value<'a, T>, cpt: &mut Compactor) -> Self {
        match value {
            ps::Value::Variable(name) => Value::Variable(cpt.word::<T>(name)),
            ps::Value::Int(n) => Value::Int(n.as_i64().unwrap()),
            ps::Value::Float(f) => Value::Float(f),
            ps::Value::String(s) => Value::String(s.to_string()),
            ps::Value::Boolean(b) => Value::Boolean(b),
            ps::Value::Null => Value::Null,
            ps::Value::Enum(e) => Value::Enum(cpt.word::<T>(e)),
            ps::Value::List(l) => {
                Value::List(l.into_iter().map(|v| Value::compact(v, cpt)).collect())
            }
            ps::Value::Object(o) => Value::Object(
                o.into_iter()
                    .map(|(k, v)| (cpt.word::<T>(k), Value::compact(v, cpt)))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::ScalarType<'a, T>> for ScalarType {
    fn compact(scalar: ps::ScalarType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::ScalarType {
            name,
            directives,
            position: _,
            description,
        } = scalar;
        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            directives,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
}

impl<'a, T: Text<'a>> Compact<ps::ScalarTypeExtension<'a, T>> for ScalarTypeExtension {
    fn compact(ext: ps::ScalarTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::ScalarTypeExtension {
            name,
            directives,
            position: _,
        } = ext;
        let name = cpt.word::<T>(name);
        let directives = directives
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
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::ObjectType<'a, T>> for ObjectType {
    fn compact(obj: ps::ObjectType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::ObjectType {
            name,
            implements_interfaces,
            directives,
            fields,
            position: _,
            description,
        } = obj;
        let name = cpt.word::<T>(name);
        let implements_interfaces = implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
            description,
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

impl<'a, T: Text<'a>> Compact<ps::ObjectTypeExtension<'a, T>> for ObjectTypeExtension {
    fn compact(ext: ps::ObjectTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::ObjectTypeExtension {
            name,
            implements_interfaces,
            directives,
            fields,
            position: _,
        } = ext;
        let name = cpt.word::<T>(name);
        let implements_interfaces = implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
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

impl<'a, T: Text<'a>> Compact<ps::Directive<'a, T>> for Directive {
    fn compact(dir: ps::Directive<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::Directive {
            name,
            arguments,
            position: _,
        } = dir;
        let name = cpt.word::<T>(name);
        let arguments = arguments
            .into_iter()
            .map(|(k, v)| (cpt.word::<T>(k), Value::compact(v, cpt)))
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

impl<'a, T: Text<'a>> Compact<ps::Type<'a, T>> for Type {
    fn compact(ty: ps::Type<'a, T>, cpt: &mut Compactor) -> Self {
        match ty {
            ps::Type::NamedType(name) => Type::NamedType(cpt.word::<T>(name)),
            ps::Type::ListType(ty) => Type::ListType(Box::new(Type::compact(*ty, cpt))),
            ps::Type::NonNullType(ty) => Type::NonNullType(Box::new(Type::compact(*ty, cpt))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Word,
    pub arguments: Vec<InputValue>,
    pub field_type: Type,
    pub directives: Vec<Directive>,
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::Field<'a, T>> for Field {
    fn compact(field: ps::Field<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::Field {
            name,
            arguments,
            field_type,
            directives,
            position: _,
            description,
        } = field;

        let name = cpt.word::<T>(name);
        let arguments = arguments
            .into_iter()
            .map(|arg| InputValue::compact(arg, cpt))
            .collect();
        let field_type = Type::compact(field_type, cpt);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            arguments,
            field_type,
            directives,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputValue {
    pub name: Word,
    pub value_type: Type,
    pub default_value: Option<Value>,
    pub directives: Vec<Directive>,
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::InputValue<'a, T>> for InputValue {
    fn compact(val: ps::InputValue<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::InputValue {
            position: _,
            description,
            name,
            value_type,
            default_value,
            directives,
        } = val;

        let name = cpt.word::<T>(name);
        let value_type = Type::compact(value_type, cpt);
        let default_value = default_value.map(|v| Value::compact(v, cpt));
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            value_type,
            default_value,
            directives,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType {
    pub name: Word,
    pub implements_interfaces: Vec<Word>,
    pub directives: Vec<Directive>,
    pub fields: Vec<Field>,
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::InterfaceType<'a, T>> for InterfaceType {
    fn compact(int: ps::InterfaceType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::InterfaceType {
            position: _,
            description,
            name,
            implements_interfaces,
            directives,
            fields,
        } = int;

        let name = cpt.word::<T>(name);
        let implements_interfaces = implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
            .into_iter()
            .map(|field| Field::compact(field, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            implements_interfaces,
            directives,
            fields,
            description,
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

impl<'a, T: Text<'a>> Compact<ps::InterfaceTypeExtension<'a, T>> for InterfaceTypeExtension {
    fn compact(ext: ps::InterfaceTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::InterfaceTypeExtension {
            position: _,
            name,
            implements_interfaces,
            directives,
            fields,
        } = ext;

        let name = cpt.word::<T>(name);
        let implements_interfaces = implements_interfaces
            .into_iter()
            .map(|name| cpt.word::<T>(name))
            .collect();
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
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
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::UnionType<'a, T>> for UnionType {
    fn compact(union: ps::UnionType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::UnionType {
            position: _,
            description,
            name,
            directives,
            types,
        } = union;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let types = types.into_iter().map(|t| cpt.word::<T>(t)).collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            directives,
            types,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub types: Vec<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::UnionTypeExtension<'a, T>> for UnionTypeExtension {
    fn compact(ext: ps::UnionTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::UnionTypeExtension {
            position: _,
            name,
            directives,
            types,
        } = ext;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let types = types.into_iter().map(|t| cpt.word::<T>(t)).collect();
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
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::EnumType<'a, T>> for EnumType {
    fn compact(enum_type: ps::EnumType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::EnumType {
            position: _,
            description,
            name,
            directives,
            values,
        } = enum_type;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let values = values
            .into_iter()
            .map(|val| EnumValue::compact(val, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            directives,
            values,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::EnumValue<'a, T>> for EnumValue {
    fn compact(val: ps::EnumValue<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::EnumValue {
            position: _,
            description,
            name,
            directives,
        } = val;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            directives,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub values: Vec<EnumValue>,
}

impl<'a, T: Text<'a>> Compact<ps::EnumTypeExtension<'a, T>> for EnumTypeExtension {
    fn compact(ext: ps::EnumTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::EnumTypeExtension {
            position: _,
            name,
            directives,
            values,
        } = ext;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let values = values
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
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::InputObjectType<'a, T>> for InputObjectType {
    fn compact(obj: ps::InputObjectType<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::InputObjectType {
            position: _,
            description,
            name,
            directives,
            fields,
        } = obj;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
            .into_iter()
            .map(|field| InputValue::compact(field, cpt))
            .collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            directives,
            fields,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectTypeExtension {
    pub name: Word,
    pub directives: Vec<Directive>,
    pub fields: Vec<InputValue>,
}

impl<'a, T: Text<'a>> Compact<ps::InputObjectTypeExtension<'a, T>> for InputObjectTypeExtension {
    fn compact(ext: ps::InputObjectTypeExtension<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::InputObjectTypeExtension {
            position: _,
            name,
            directives,
            fields,
        } = ext;

        let name = cpt.word::<T>(name);
        let directives = directives
            .into_iter()
            .map(|dir| Directive::compact(dir, cpt))
            .collect();
        let fields = fields
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

impl From<ps::DirectiveLocation> for DirectiveLocation {
    fn from(loc: ps::DirectiveLocation) -> Self {
        use ps::DirectiveLocation::*;
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
    pub description: Option<Word>,
}

impl<'a, T: Text<'a>> Compact<ps::DirectiveDefinition<'a, T>> for DirectiveDefinition {
    fn compact(def: ps::DirectiveDefinition<'a, T>, cpt: &mut Compactor) -> Self {
        let ps::DirectiveDefinition {
            position: _,
            description,
            name,
            arguments,
            repeatable,
            locations,
        } = def;

        let name = cpt.word::<T>(name);
        let arguments = arguments
            .into_iter()
            .map(|arg| InputValue::compact(arg, cpt))
            .collect();
        let locations = locations.into_iter().map(DirectiveLocation::from).collect();
        let description = description.map(|d| Word::from(d));
        Self {
            name,
            arguments,
            repeatable,
            locations,
            description,
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
