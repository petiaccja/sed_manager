//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use crate::token_stream::{Detokenize, Detokenizer, MessageError, Tokenize, Tokenizer};

pub type MaxBytes<const N: usize> = heapless::Vec<u8, N>;

impl<const N: usize> Tokenize for heapless::Vec<u8, N> {
    fn tokenize<T: Tokenizer>(&self, tokenizer: &mut T) -> Result<(), T::Error> {
        tokenizer.tokenize_bytes(self.as_slice())
    }
}

impl<const N: usize> Detokenize for heapless::Vec<u8, N> {
    fn detokenize<D: Detokenizer>(detokenizer: &mut D) -> Result<Self, D::Error> {
        let bytes = detokenizer.detokenize_bytes()?;
        let count = bytes.len();
        Self::try_from(bytes.as_slice())
            .map_err(|_| D::Error::message(format!("the number of bytes {count} exceeds limit of {N}")))
    }
}
