/// A struct for generating addresses based on a base public key and program ID.
#[derive(Default)]
pub struct AddressGenerator {
    base_pubkey: Pubkey,
    program_id: Pubkey,
    nth: usize,
}

impl AddressGenerator {
    /// Creates a new `AddressGenerator` with the given base public key and program ID.
    ///
    /// # Arguments
    ///
    /// * `base_pubkey` - The base public key used for generating addresses.
    /// * `program_id` - The program ID used for generating addresses.
    ///
    /// # Returns
    ///
    /// A new `AddressGenerator` instance.
    pub fn new(base_pubkey: &Pubkey, program_id: &Pubkey) -> Self {
        Self {
            base_pubkey: *base_pubkey,
            program_id: *program_id,
            nth: 0,
        }
    }

    /// Generates the nth address based on the base public key, program ID, and the given nth value.
    ///
    /// # Arguments
    ///
    /// * `nth` - The nth value used for generating the address.
    ///
    /// # Returns
    ///
    /// The generated `Pubkey`.
    pub fn nth(&self, nth: usize) -> Pubkey {
        Pubkey::create_with_seed(&self.base_pubkey, &format!("{}", nth), &self.program_id).unwrap()
    }

    /// Generates the next address based on the base public key, program ID, and the current nth value.
    ///
    /// # Returns
    ///
    /// The generated `Pubkey`.
    pub fn next(&mut self) -> Pubkey {
        let nth = self.nth;
        self.nth += 1;
        self.nth(nth)
    }
}
