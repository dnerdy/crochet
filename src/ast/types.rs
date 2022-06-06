use crate::ast::span::Span;
use crate::ast::literal::Lit;
use crate::ast::ident::Ident;
use crate::types::Primitive;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LamType {
    pub span: Span,
    pub params: Vec<TypeAnn>,
    pub ret: Box<TypeAnn>,
    pub type_params: Option<Vec<TypeParam>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrimType {
    pub span: Span,
    pub prim: Primitive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LitType {
    pub span: Span,
    pub lit: Lit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeRef {
    pub span: Span,
    pub name: String,
    pub type_params: Option<Vec<TypeAnn>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectType {
    pub span: Span,
    pub props: Vec<TProp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TProp {
    pub span: Span,
    pub name: String,
    pub optional: bool,
    pub type_ann: Box<TypeAnn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnionType {
    pub span: Span,
    pub types: Vec<TypeAnn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntersectionType {
    pub span: Span,
    pub types: Vec<TypeAnn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleType {
    pub span: Span,
    pub types: Vec<TypeAnn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAnn {
    Lam(LamType),
    Lit(LitType),
    Prim(PrimType),
    Object(ObjectType),
    TypeRef(TypeRef),
    Union(UnionType),
    Intersection(IntersectionType),
    Tuple(TupleType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub span: Span,
    pub name: Ident,
    pub constraint: Option<Box<TypeAnn>>,
    pub default: Option<Box<TypeAnn>>,
}