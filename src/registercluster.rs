
use xmltree::Element;
use either::Either;

use helpers::ParseElem;
use register::Register;
use cluster::Cluster;

pub fn cluster_register_parse(tree: &Element) -> Either<Register, Cluster> {
    if tree.name == "register" {
        Either::Left(Register::parse(tree))
    } else if tree.name == "cluster" {
        Either::Right(Cluster::parse(tree))
    } else {
        unreachable!()
    }
}
