extern crate xmltree;

use xmltree::Element;

#[macro_export]
macro_rules! try_get_child {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

pub trait ElementExt {
    fn get_child_text<K>(&self, k: K) -> Option<String>
    where
        String: PartialEq<K>;
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text<K>(&self, k: K) -> Option<String>
    where
        String: PartialEq<K>,
    {
        self.get_child(k).map(|c| try_get_child!(c.text.clone()))
    }

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}
