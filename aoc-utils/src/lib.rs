use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::borrow::Borrow;
use std::fmt::Display;

pub trait MyItertools: Iterator {
    #[inline]
    fn position_of<Q>(&mut self, query: &Q) -> Option<usize>
    where
        Self: Sized,
        Self::Item: Borrow<Q>,
        Q: PartialEq,
    {
        self.position(|item| item.borrow() == query)
    }

    fn ordered_combinations_with_repetition(
        self,
        length: usize,
    ) -> impl Iterator<Item = Vec<Self::Item>>
    where
        Self: Clone,
        Self::Item: Clone,
    {
        std::iter::repeat_n(self, length).multi_cartesian_product()
    }

    fn cartesian_product_self(self) -> impl Iterator<Item = (Self::Item, Self::Item)>
    where
        Self: Clone,
        Self::Item: Clone,
    {
        self.clone().cartesian_product(self)
    }

    fn cartesian_product_self_skip_same(self) -> impl Iterator<Item = (Self::Item, Self::Item)>
    where
        Self: Clone,
        Self::Item: Clone + Eq,
    {
        self.cartesian_product_self().filter(|(a, b)| a != b)
    }
}

impl<T> MyItertools for T where T: Iterator + ?Sized {}

pub fn print_matrix<T: Display>(matrix: &Array2<T>) {
    for y in 0..matrix.len_of(Axis(1)) {
        for x in 0..matrix.len_of(Axis(0)) {
            print!("{}", matrix[(x, y)]);
        }
        println!();
    }
}
