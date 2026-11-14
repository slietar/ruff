//! A small public facade over ty's internal type API, used by `python-doc-extractor`.
//!
//! Not part of upstream ruff: this file lives on a python-doc-extractor branch of the
//! slietar/ruff fork. Upstream narrowed most of these items to `pub(crate)`
//! (astral-sh/ruff#27339), so this module re-exports the needed types and wraps the needed
//! methods as plain `pub fn`s. Keep it thin: logic belongs in the extractor, not here.

use ruff_python_ast::name::Name;
use ty_python_core::ProgramFile;
use ty_python_core::definition::Definition;
use ty_python_core::scope::ScopeId;

use crate::Db;
use crate::place::{Place, imported_symbol};
use crate::types::list_members::all_end_of_scope_members;

pub use crate::types::class::{ClassLiteral, ClassType};
pub use crate::types::class_base::ClassBase;
pub use crate::types::function::{FunctionType, OverloadLiteral};
pub use crate::types::{
    DynamicType, NominalInstanceType, ParameterKind, ProgramEnvironment, Type, UnionType,
};

/// The name of a class.
pub fn class_name<'db>(db: &'db dyn Db, class: ClassLiteral<'db>) -> &'db str {
    class.name(db).as_str()
}

/// The definition of a class, if it has one (dynamic classes may not).
pub fn class_definition<'db>(db: &'db dyn Db, class: ClassLiteral<'db>) -> Option<Definition<'db>> {
    class.definition(db)
}

/// The class's MRO, including the class itself as the first element.
pub fn class_mro<'db>(db: &'db dyn Db, class: ClassLiteral<'db>) -> Vec<ClassBase<'db>> {
    class.iter_mro(db).collect()
}

/// The class literal behind a (possibly specialized) class type.
pub fn class_type_literal<'db>(db: &'db dyn Db, class: ClassType<'db>) -> ClassLiteral<'db> {
    class.class_literal(db)
}

/// The body scope of a class defined by a `class` statement, or `None` for dynamic classes.
pub fn class_body_scope<'db>(db: &'db dyn Db, class: ClassLiteral<'db>) -> Option<ScopeId<'db>> {
    match class {
        ClassLiteral::Static(class) => Some(class.body_scope(db)),
        _ => None,
    }
}

/// The names of all members bound at the end of `scope`.
pub fn end_of_scope_member_names<'db>(db: &'db dyn Db, scope: ScopeId<'db>) -> Vec<Name> {
    all_end_of_scope_members(db, scope)
        .map(|member| member.member.name)
        .collect()
}

/// The type of attribute `name` on `ty`, or `None` if it is (possibly) undefined.
pub fn member_type<'db>(
    db: &'db dyn Db,
    env: &ProgramEnvironment<'db>,
    ty: Type<'db>,
    name: &str,
) -> Option<Type<'db>> {
    ty.member(db, env, name).ignore_possibly_undefined()
}

/// The class of a nominal instance type.
pub fn nominal_instance_class<'db>(
    db: &'db dyn Db,
    env: &ProgramEnvironment<'db>,
    instance: NominalInstanceType<'db>,
) -> ClassType<'db> {
    instance.class(db, env)
}

/// A function's overloads in source order, plus its implementation, if any.
pub fn function_overloads_and_implementation<'db>(
    db: &'db dyn Db,
    function: FunctionType<'db>,
) -> (&'db [OverloadLiteral<'db>], Option<OverloadLiteral<'db>>) {
    function.overloads_and_implementation(db)
}

/// A parameter of an overload: its kind and its annotated (or inferred) type.
pub struct OverloadParameter<'db> {
    pub kind: ParameterKind<'db>,
    pub ty: Type<'db>,
}

/// The parameters and return type of a single overload (or implementation).
pub fn overload_signature<'db>(
    db: &'db dyn Db,
    overload: OverloadLiteral<'db>,
) -> (Vec<OverloadParameter<'db>>, Type<'db>) {
    let signature = overload.signature(db);
    let parameters = signature
        .parameters()
        .iter()
        .map(|param| OverloadParameter {
            kind: param.kind().clone(),
            ty: param.annotated_type(),
        })
        .collect();
    (parameters, signature.return_ty)
}

/// The type of the module-level symbol `name` as seen by importers of `file`.
pub fn imported_symbol_type<'db>(
    db: &'db dyn Db,
    env: &ProgramEnvironment<'db>,
    file: ProgramFile<'db>,
    name: &str,
) -> Option<Type<'db>> {
    match imported_symbol(db, env, Some(file), name, None).place {
        Place::Defined(place) => Some(place.ty),
        Place::Undefined => None,
    }
}
