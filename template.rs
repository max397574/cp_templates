// TODO: Link

// this template and prewritten code here:
// https://github.com/max397574/cp_templates
#![allow(unused_imports)]
#![allow(dead_code)]
fn solve(wr: &mut Writer<Stdout>, r: &mut Reader) {
    let n: usize = r.r();
    writeln!(wr.buf, "{n}").unwrap();
}

fn main() {
    let mut r = Reader::new(std::io::stdin());
    let mut wr = Writer::new(std::io::stdout());

    #[cfg(debug_assertions)]
    let start = Instant::now();

    let t: usize = r.r();
    for _ in 0..t {
        solve(&mut wr, &mut r);
    }

    #[cfg(debug_assertions)]
    println!("Elapsed: {:?}", start.elapsed());

    wr.fl();
}

use std::{
    io::{BufReader, BufWriter, Stdin, Stdout},
    time::Instant,
};

use writer::*;
mod writer {
    pub use std::io::Write;
    use std::io::{self};

    pub struct Writer<W: Sized + Write> {
        pub buf: io::BufWriter<W>,
    }

    impl<W: Sized + Write> Writer<W> {
        #[inline]
        pub fn new(w: W) -> Self {
            Self {
                buf: io::BufWriter::new(w),
            }
        }

        pub fn fl(&mut self) {
            self.buf.flush().expect("Couldnt' flush buffer");
        }
    }
}

use reader::*;
mod reader {
    use std::io::{self, BufRead, Read, Stdin};

    pub struct Reader {
        pub input: Stdin,
        pub current_line: String,
        pub offset: usize,
    }

    impl Reader {
        #[inline]
        pub fn new(input: Stdin) -> Self {
            let mut buf = String::new();
            input
                .read_line(&mut buf)
                .expect("Couldn't read anything when creating new Reader");
            Reader {
                input,
                current_line: buf,
                offset: 0,
            }
        }

        #[inline]
        pub fn new_line(&mut self) -> io::Result<()> {
            self.current_line.clear();
            self.input.read_line(&mut self.current_line)?;
            self.offset = 0;
            Ok(())
        }

        #[inline]
        pub fn rest(&self) -> &str {
            &self.current_line[self.offset..]
        }

        pub fn skip_whitespace(&mut self) -> io::Result<()> {
            loop {
                let whitespace_len = self
                    .rest()
                    .bytes()
                    .take_while(|&c| c.is_ascii_whitespace())
                    .count();
                self.offset += whitespace_len;
                if self.rest().is_empty() {
                    self.new_line()?;
                } else {
                    return Ok(());
                }
            }
        }

        #[inline]
        pub fn rw(&mut self) -> String {
            self.read_next_word().unwrap().to_string()
        }

        #[inline]
        pub fn read_next_word(&mut self) -> io::Result<&str> {
            self.skip_whitespace()?;
            let word_len = self
                .rest()
                .bytes()
                .take_while(|&c| !c.is_ascii_whitespace())
                .count();
            let word = &self.current_line[self.offset..(self.offset + word_len)];
            self.offset += word_len;

            Ok(word)
        }
    }
}

use read_from::*;
mod read_from {
    use std::array;
    use std::io::BufRead;
    use std::iter::repeat_with;
    use std::num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    };
    use std::str::FromStr;

    use super::reader::*;

    pub trait ReadFrom<'a, O> {
        fn read_from(reader: &'a mut Reader, options: O) -> Self;
    }

    macro_rules! impl_readform_simple {
    ( $( $t:ty ),* ) => {
        $(
            impl<'a> ReadFrom<'a, ()> for $t {
                #[inline]
                fn read_from(reader: &'a mut Reader, (): ()) -> Self {
                    let word = reader.read_next_word().expect("No new word found");
                    FromStr::from_str(word).expect("Failed to Parse")
                }
            }
        )*
    };
}

    impl_readform_simple! {
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        bool, f32, f64, char
    }

    impl<'a> ReadFrom<'a, ()> for &'a str {
        #[inline]
        fn read_from(reader: &'a mut Reader, (): ()) -> Self {
            reader.read_next_word().unwrap()
        }
    }

    impl<'a, T> ReadFrom<'a, usize> for Vec<T>
    where
        T: for<'b> ReadFrom<'b, ()>,
    {
        #[inline]
        fn read_from(reader: &'a mut Reader, len: usize) -> Self {
            repeat_with(|| T::read_from(reader, ())).take(len).collect()
        }
    }

    pub trait ReadExt {
        fn r<'a, T: ReadFrom<'a, ()>>(&'a mut self) -> T;

        fn rv<T: for<'b> ReadFrom<'b, usize>>(&mut self, len: usize) -> T;

        fn ri<T: for<'b> ReadFrom<'b, ()>>(&mut self, len: usize) -> impl Iterator<Item = T>;
    }

    impl ReadExt for Reader {
        fn r<'a, T: ReadFrom<'a, ()>>(&'a mut self) -> T {
            T::read_from(self, ())
        }

        fn rv<T: for<'b> ReadFrom<'b, usize>>(&mut self, len: usize) -> T {
            T::read_from(self, len)
        }

        fn ri<T: for<'b> ReadFrom<'b, ()>>(&mut self, len: usize) -> impl Iterator<Item = T> {
            repeat_with(move || T::read_from(self, ())).take(len)
        }
    }

    impl<'a, V, T> ReadFrom<'a, ()> for (V, T)
    where
        V: for<'b> ReadFrom<'b, ()>,
        T: for<'b> ReadFrom<'b, ()>,
    {
        fn read_from(reader: &'a mut Reader, (): ()) -> Self {
            (V::read_from(reader, ()), T::read_from(reader, ()))
        }
    }

    impl<'a, V, T, U> ReadFrom<'a, ()> for (V, T, U)
    where
        V: for<'b> ReadFrom<'b, ()> + Default + Copy,
        T: for<'b> ReadFrom<'b, ()> + Default + Copy,
        U: for<'b> ReadFrom<'b, ()> + Default + Copy,
    {
        fn read_from(reader: &mut Reader, (): ()) -> Self {
            (
                V::read_from(reader, ()),
                T::read_from(reader, ()),
                U::read_from(reader, ()),
            )
        }
    }

    impl<'a, T, const N: usize> ReadFrom<'a, ()> for [T; N]
    where
        T: for<'b> ReadFrom<'b, ()> + Default + Copy,
    {
        fn read_from(reader: &'a mut Reader, (): ()) -> Self {
            array::from_fn(|_| T::read_from(reader, ()))
        }
    }
}

use num_traits::*;
mod num_traits {
    pub trait IsInteger {}
    pub trait IsNum {}
    pub trait IsPositiveInteger {}

    macro_rules! impl_is_integer {
        ($($t:ty),*) => {
            $(impl IsInteger for $t {})*
        };
    }

    macro_rules! impl_is_num {
        ($($t:ty),*) => {
            $(impl IsNum for $t {})*
        };
    }

    macro_rules! impl_is_pos_int {
        ($($t:ty),*) => {
            $(impl IsPositiveInteger for $t {})*
        };
    }

    impl_is_integer!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );
    impl_is_pos_int!(u8, u16, u32, u64, u128, usize);
    impl_is_num!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
    );
}
