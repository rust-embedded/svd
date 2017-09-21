extern crate xmltree;
extern crate either;

use xmltree::Element;
use either::Either;

#[macro_use]
use elementext::*;

use helpers::*;
use register::*;
use cluster::*;

pub fn cluster_register_parse(tree: &Element) -> Either<Register, Cluster> {
    if tree.name == "register" {
        Either::Left(Register::parse(tree))
    } else if tree.name == "cluster" {
        Either::Right(Cluster::parse(tree))
    } else {
        unreachable!()
    }
}
