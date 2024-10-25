use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'stmt> {
    /// Which records to delete
    pub selection: Query<'stmt>,
}

impl<'stmt> Delete<'stmt> {}

impl<'stmt> From<Delete<'stmt>> for Statement<'stmt> {
    fn from(src: Delete<'stmt>) -> Statement<'stmt> {
        Statement::Delete(src)
    }
}

impl<'stmt> Node<'stmt> for Delete<'stmt> {
    fn map<V: Map<'stmt>>(&self, visit: &mut V) -> Self {
        visit.map_stmt_delete(self)
    }

    fn visit<V: Visit<'stmt>>(&self, mut visit: V) {
        visit.visit_stmt_delete(self);
    }

    fn visit_mut<V: VisitMut<'stmt>>(&mut self, mut visit: V) {
        visit.visit_stmt_delete_mut(self);
    }
}
