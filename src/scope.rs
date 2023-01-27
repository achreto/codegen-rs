use std::fmt::{self, Write};

use indexmap::IndexMap;

use crate::comment::Comment;
use crate::consts::Const;
use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::import::Import;
use crate::item::Item;
use crate::license::License;
use crate::module::Module;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;

/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Debug, Clone)]
pub struct Scope {
    /// Scope documentation
    docs: Option<Docs>,

    /// Scope License
    license: Option<License>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}

impl Scope {
    /// Returns a new scope
    pub fn new() -> Self {
        Self {
            docs: None,
            license: None,
            imports: IndexMap::new(),
            items: vec![],
        }
    }

    /// adds documentation to the scope
    pub fn docs(&mut self, docs: Docs) -> &mut Self {
        self.docs = Some(docs);
        self
    }

    /// adds license information to the scope
    pub fn license(&mut self, license: License) -> &mut Self {
        self.license = Some(license);
        self
    }

    /// Import a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn import(&mut self, path: &str, ty: &str) -> &mut Import {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let ty = ty.split("::").next().unwrap_or(ty);
        self.imports
            .entry(path.to_string())
            .or_default()
            .entry(ty.to_string())
            .or_insert_with(|| Import::new(path, ty))
    }

    /// Push a new cost definition, returning a mutable reference to it.
    pub fn new_const(&mut self, name: &str, ty: &str, value: &str) -> &mut Const {
        self.push_const(Const::new(name, ty, value));

        match self.items.last_mut().unwrap() {
            Item::Const(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a const definition
    pub fn push_const(&mut self, item: Const) -> &mut Self {
        self.items.push(Item::Const(item));
        self
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: &str) -> &mut Module {
        self.push_module(Module::new(name));

        match self.items.last_mut().unwrap() {
            Item::Module(v) => v,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter_mut()
            .filter_map(|item| match item {
                Item::Module(ref mut module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Module(module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module(&mut self, name: &str) -> &mut Module {
        if self.get_module(name).is_some() {
            self.get_module_mut(name).unwrap()
        } else {
            self.new_module(name)
        }
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        assert!(self.get_module(&item.name).is_none());
        self.items.push(Item::Module(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: &str) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match self.items.last_mut().unwrap() {
            Item::Struct(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a struct definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.items.push(Item::Struct(item));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.push_fn(Function::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Push a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: &str) -> &mut Trait {
        self.push_trait(Trait::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Trait(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: &str) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a structure definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Push a new comment, returning a mutable reference to it.
    pub fn new_comment(&mut self, comment: &str) -> &mut Comment {
        self.push_comment(Comment::new(comment));

        match *self.items.last_mut().unwrap() {
            Item::Comment(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a comment
    pub fn push_comment(&mut self, comment: Comment) -> &mut Self {
        self.items.push(Item::Comment(comment));
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: &str) -> &mut Impl {
        self.push_impl(Impl::new(target));

        match *self.items.last_mut().unwrap() {
            Item::Impl(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Push a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: &str) -> &mut Self {
        self.items.push(Item::Raw(val.to_string()));
        self
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // documentation and license information
        self.license.as_ref().map(|l| l.fmt(fmt));
        self.docs.as_ref().map(|d| d.fmt(fmt));

        self.fmt_imports(fmt)?;

        if !self.imports.is_empty() {
            writeln!(fmt)?;
        }

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                writeln!(fmt)?;
            }

            match &item {
                Item::Comment(v) => v.fmt(fmt)?,
                Item::Module(v) => v.fmt(fmt)?,
                Item::Const(v) => v.fmt(fmt)?,
                Item::Struct(v) => v.fmt(fmt)?,
                Item::Function(v) => v.fmt(false, fmt)?,
                Item::Trait(v) => v.fmt(fmt)?,
                Item::Enum(v) => v.fmt(fmt)?,
                Item::Impl(v) => v.fmt(fmt)?,
                Item::Raw(v) => {
                    writeln!(fmt, "{v}")?;
                }
            }
        }

        Ok(())
    }

    fn fmt_imports(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = vec![];

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(&import.vis) {
                    visibilities.push(import.vis.clone());
                }
            }
        }

        let mut tys = vec![];

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                tys.clear();

                for (ty, import) in imports {
                    if *vis == import.vis {
                        tys.push(ty);
                    }
                }

                if !tys.is_empty() {
                    if let Some(vis) = &vis {
                        write!(fmt, "{vis} ")?;
                    }

                    write!(fmt, "use {path}::")?;

                    match tys.len() {
                        0 => {}
                        1 => {
                            writeln!(fmt, "{};", tys[0])?;
                        }
                        _ => {
                            write!(fmt, "{{")?;

                            for (i, ty) in tys.iter().enumerate() {
                                if i != 0 {
                                    write!(fmt, ", ")?;
                                }
                                write!(fmt, "{ty}")?;
                            }

                            writeln!(fmt, "}};")?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();

        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }

        write!(f, "{ret}")
    }
}
