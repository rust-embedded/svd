use xmltree;
use std::str;
use std::num;
error_chain! {
    foreign_links {
        XmlParseError(xmltree::ParseError);
        StrParseBoolError(str::ParseBoolError);
        NumParseIntError(num::ParseIntError);

    }
}
