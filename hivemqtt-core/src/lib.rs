pub mod v5;
pub mod constants;
#[cfg(test)]
mod retest_utils;


// pub(crate) trait FromLeBytes {
//     fn from_le_bytes(bytes: &[u8]) -> Self;
// }

// macro_rules! impl_from_le_bytes {
//     ($($ty:ty),*) => {
//         $(
//             impl FromLeBytes for $ty {
//                 fn (bytes: &[u8]) -> Self {
//                     <$ty>::from_le_bytes(bytes.try_into().unwrap())
//                 }
//             }
//         )*
//     };
// }

// // impl_from_le_bytes!(u8, u16, u32);