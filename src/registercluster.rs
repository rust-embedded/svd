extern crate xmltree;
extern crate either;

use xmltree::Element;
use either::Either;

use ElementExt;

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
