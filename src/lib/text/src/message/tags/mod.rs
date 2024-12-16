use crate::{
    message::{ast::ParsedTag, Context, ParserOptions, ResolveError},
    TextComponentBuilder,
};

pub mod color_resolver;
pub mod reset_resolver;

pub trait TagResolver {
    fn can_process(&self, tag: &ParsedTag<'_>, opts: &ParserOptions) -> bool {
        let can_process = self.can_process_tag(tag.name);
        /*((can_process && !opts.strict)
            || (can_process && opts.strict && self.strict_allowed(tag.name)))
            && (!tag.has_end_tag || (tag.has_end_tag && self.has_end_tag(tag.name)))*/
        can_process && (!opts.strict || (self.strict_allowed(tag.name) && self.has_end_tag(tag.name) == tag.has_end_tag))
    }

    fn call<'a>(
        &self,
        context: &mut Context<'a>,
        opts: &ParserOptions,
    ) -> Result<bool, ResolveError> {
        if let Context::Parse { tag, builder } = context {
            if self.can_process(tag, opts) {
                let name = tag.name;
                self.resolve(name, builder, opts)?;
                return Ok(true);
            }
        }

        if let Context::Pre { tag, .. } = context {
            if self.can_process(tag, opts) {
                let name = tag.name;
                self.preresolve(name, context, opts)?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// called in the parse step if this resolver [can_process_tag] and if not in strict mode
    /// unless strict mode is allowed [strict_allowed] which by default is strict is allowed.
    ///
    fn resolve(
        &self,
        name: &str,
        _builder: &mut TextComponentBuilder,
        _opts: &ParserOptions,
    ) -> Result<(), ResolveError> {
        Err(ResolveError::ProcessUnimplemented(name.to_string()))
    }

    fn preresolve<'a>(
        &self,
        name: &str,
        _context: &mut Context<'a>,
        _opts: &ParserOptions,
    ) -> Result<(), ResolveError> {
        Err(ResolveError::ProcessUnimplemented(name.to_string()))
    }

    fn can_process_tag(&self, _name: &str) -> bool {
        true
    }

    fn strict_allowed(&self, _name: &str) -> bool {
        true
    }

    fn has_end_tag(&self, _name: &str) -> bool {
        true
    }
}

impl<F> TagResolver for F
where
    F: for<'a, 'b, 'c> Fn(&'b mut Context<'a>, &'c ParserOptions) -> Result<bool, ResolveError>,
{
    fn call<'a>(
        &self,
        context: &mut Context<'a>,
        opts: &ParserOptions,
    ) -> Result<bool, ResolveError> {
        self(context, opts)
    }
}
