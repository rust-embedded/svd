

use xmltree::Element;
use either::Either;

use types::{Parse};

use ::error::SVDError;
use ::svd::register::Register;
use ::svd::cluster::Cluster;

pub fn cluster_register_parse(tree: &Element) -> Result<Either<Register, Cluster>, SVDError> {
    if tree.name == "register" {
        Ok(Either::Left(Register::parse(tree)?))
    } else if tree.name == "cluster" {
        Ok(Either::Right(Cluster::parse(tree)?))
    } else {
        unreachable!()
    }
}
