





pub trait ElementOperation<V> {
    fn execute(&self, context: &ElementContext<V>) -> Result<(), MatricalError>;
}

pub trait ElementStrategy<V> {
    fn execute(
        &self,
        element: &Element<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}


