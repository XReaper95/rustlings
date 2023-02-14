// try_from_into.rs
// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for a hint.

use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    /// Incorrect length of slice
    BadLen,
    /// Integer conversion error
    IntConversion,
}

// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need to create an implementation for a tuple of three integers,
// an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile time,
// but the slice implementation needs to check the slice length!
// Also note that correct RGB color values must be integers in the 0..=255 range.

// mod first_naive_impl {
//     use super::*;
//
//     fn in_bounds(i: i16) -> bool {
//         return i > 0 && i <= 255;
//     }
//
//     // Tuple implementation
//     impl TryFrom<(i16, i16, i16)> for Color {
//         type Error = IntoColorError;
//         fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
//             match tuple {
//                 (r, g, b) if u8::try_from(r) && in_bounds(g) && in_bounds(b) => {
//                     Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
//                 }
//                 _ => Err(IntoColorError::IntConversion)
//             }
//         }
//     }
//
//     // Array implementation
//     impl TryFrom<[i16; 3]> for Color {
//         type Error = IntoColorError;
//         fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
//             match arr {
//                 [r, g, b] if in_bounds(r) && in_bounds(g) && in_bounds(b) => {
//                     Ok(Color { red: r as u8, green: g as u8, blue: b as u8 })
//                 }
//                 _ => Err(IntoColorError::IntConversion)
//             }
//         }
//     }
//
//     // Slice implementation
//     impl TryFrom<&[i16]> for Color {
//         type Error = IntoColorError;
//         fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
//             match slice {
//                 [r, g, b] if in_bounds(*r) && in_bounds(*g) && in_bounds(*b) => {
//                     Ok(Color { red: *r as u8, green: *g as u8, blue: *b as u8 })
//                 }
//                 [ _, _, _ ] => Err(IntoColorError::IntConversion),
//                 _ => Err(IntoColorError::BadLen)
//             }
//         }
//     }
// }

mod generic_impl {
    use super::*;

    impl From<TryFromIntError> for IntoColorError {
    fn from(_: TryFromIntError) -> Self {
        IntoColorError::IntConversion
    }
}
    // Tuple implementation GENERIC
    impl<T> TryFrom<(T, T, T)> for Color
    where
        T: TryInto<u8, Error=TryFromIntError>
    {
        type Error = IntoColorError;
        fn try_from((r, g, b): (T, T, T)) -> Result<Self, Self::Error> {
            Ok(Color {
                red: r.try_into()?,
                green: g.try_into()?,
                blue: b.try_into()?
            })
        }
    }

    // Array implementation GENERIC
    impl<T> TryFrom<[T; 3]> for Color
    where
        T: TryInto<u8, Error=TryFromIntError>
    {
        type Error = IntoColorError;
        fn try_from([r, g, b]: [T; 3]) -> Result<Self, Self::Error> {
            (r, g, b).try_into()
        }
    }

    // Slice implementation GENERIC
    impl<T> TryFrom<&[T]> for Color
    where
        T: TryInto<u8, Error=TryFromIntError> + Copy
    {
        type Error = IntoColorError;
        fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
            match slice {
                [r, g, b] => {
                    (*r, *g, *b).try_into()
                }
                _ => Err(IntoColorError::BadLen)
            }
        }
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
