use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Input<'stmt> {
    /// Source of the input
    pub(crate) source: InputSource,

    /// If needed, how to project the input
    pub(crate) project: Option<eval::Expr<'stmt>>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum InputSource {
    /// Take an entry from the variable table by value. This consumes the entry.
    Value(VarId),

    /// Read an entry from the variable table by reference. This leaves the
    /// value in the variable table. Streams are buffered.
    Ref(VarId),
}

impl<'stmt> Input<'stmt> {
    pub(crate) fn from_var(var: VarId) -> Input<'stmt> {
        Input {
            source: InputSource::Value(var),
            project: None,
        }
    }

    pub(crate) fn project_var_ref(var: VarId, expr: eval::Expr<'stmt>) -> Input<'stmt> {
        Input {
            source: InputSource::Ref(var),
            project: Some(expr),
        }
    }
}
