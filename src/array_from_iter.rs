use std::iter::FromIterator;
use std::mem::{self, MaybeUninit};

pub trait IterIntoArray<T>: Sized {
    fn collect_into_array<const N: usize>(
        &mut self,
        buf: &mut [MaybeUninit<T>; N],
    ) -> Option<&mut [T; N]>;

    fn collect_array<const N: usize>(&mut self) -> Option<[T; N]> {
        let mut buf = MaybeUninit::uninit_array::<N>();
        // self.collect_into_array(&mut buf).map(|buf| {
        //     mem::replace(buf, unsafe {
        //         mem::transmute_copy(&MaybeUninit::<T>::uninit_array::<N>())
        //     })
        // })
        self.collect_into_array(&mut buf)?;
        unsafe { mem::transmute_copy(&buf) }
    }
}

impl<T, I> IterIntoArray<T> for I
where
    I: Iterator<Item = T>,
{
    #[allow(clippy::transmute_ptr_to_ptr)]
    fn collect_into_array<const N: usize>(
        &mut self,
        buf: &mut [MaybeUninit<T>; N],
    ) -> Option<&mut [T; N]> {
        for b in buf.iter_mut() {
            b.write(self.next()?);
        }
        Some(unsafe { mem::transmute(buf) })
    }
    fn collect_array<const N: usize>(&mut self) -> Option<[T; N]> {
        let mut buf = MaybeUninit::uninit_array::<N>();
        for b in buf.iter_mut() {
            b.write(self.next()?);
        }
        Some(unsafe { mem::transmute_copy(&buf) })
    }
}

// #[derive(Deref, DerefMut, Into, From)]
pub struct ArrayFromIter<T, const N: usize>(Option<[T; N]>);
impl<T, const N: usize> ArrayFromIter<T, N> {
    pub fn into_array(ArrayFromIter(arr): Self) -> Option<[T; N]> {
        arr
    }
}
impl<T, const N: usize> FromIterator<T> for ArrayFromIter<T, N> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        ArrayFromIter(
            try {
                let mut buf = MaybeUninit::uninit_array::<N>();
                let mut iter = iter.into_iter();
                if false {
                    for b in buf.iter_mut() {
                        b.write(iter.next()?);
                    }
                    unsafe { mem::transmute_copy(&buf) }
                } else {
                    // iter.collect_into_array(&mut buf).map(|buf| {
                    //     mem::replace(buf, unsafe {
                    //         mem::transmute_copy(&MaybeUninit::<T>::uninit_array::<N>())
                    //     })
                    // })?
                    iter.collect_into_array(&mut buf)?;
                    unsafe { mem::transmute_copy(&buf) }
                }
            },
        )
    }
}
