#[derive(Debug, Default)]
pub struct D1QueryResult;

impl Extend<D1QueryResult> for D1QueryResult {
    fn extend<T: IntoIterator<Item = D1QueryResult>>(&mut self, iter: T) {
        todo!()
    }
}
