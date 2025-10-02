use crate::palette::Palette;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn optimise_single(&mut self) {
        // Do nothing if already single
    }
}
