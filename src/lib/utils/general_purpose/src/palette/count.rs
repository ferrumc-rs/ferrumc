use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    /// Retrieves the count of a specific value in the palette.
    ///
    /// # Arguments
    /// * `value` - A reference to the value whose count is to be determined.
    ///
    /// # Returns
    /// * `usize` - The count of the specified value in the palette.
    ///
    /// # Variants
    /// The behavior depends on the `PaletteType`:
    /// - `Single`: Returns the length if the value matches, otherwise 0.
    /// - `Indirect`: Searches for the value in the palette and returns its count. If not found, returns 0.
    /// - `Direct`: Counts the occurrences of the value in the list of values.
    pub fn get_count(&self, value: &T) -> usize
    where
        T: Eq,
    {
        match &self.palette_type {
            // Single variant: Check if the value matches the stored value.
            PaletteType::Single(v) => self.count_single(value),
            // Indirect variant: Search for the value in the palette and return its count.
            PaletteType::Indirect { palette, .. } => self.count_indirect(value),
            // Direct variant: Count the occurrences of the value in the list of values.
            PaletteType::Direct(values) => self.count_direct(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn test_single_variant_match() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_count(&42), 5);
    }

    #[test]
    fn test_single_variant_no_match() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_count(&7), 0);
    }

    #[test]
    fn test_indirect_variant() {
        let palette = Palette::from([1, 2, 1].to_vec());
        assert_eq!(palette.get_count(&1), 2);
        assert_eq!(palette.get_count(&2), 1);
        assert_eq!(palette.get_count(&3), 0);
    }

    #[test]
    fn test_direct_variant() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 1, 3]),
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_count(&1), 2);
        assert_eq!(palette.get_count(&2), 1);
        assert_eq!(palette.get_count(&3), 1);
        assert_eq!(palette.get_count(&4), 0);
    }
}
