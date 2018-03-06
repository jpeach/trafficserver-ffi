#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        assert_ne!(TSParseResult_TS_PARSE_DONE, TSParseResult_TS_PARSE_ERROR)
    }
}
