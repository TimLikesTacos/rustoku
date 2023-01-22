#[derive(Clone)]
pub(crate) enum House<T: Clone> {
    Row(T),
    Col(T),
    Box(T),
}
