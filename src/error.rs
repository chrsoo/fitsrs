quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum Error {
        /// General error case
        StaticError(message: &'static str) {
            from()
            display("{}", message)
        }
        BitpixBadValue {
            display("Bitpix value found is not valid. Standard values are: -64, -32, 8, 16, 32 and 64.")
        }
        /// Fits file is not a multiple of 80 bytes long
        FailReadingNextBytes {
            display("A 80 bytes card could not be read. A fits file must have a multiple of 80 characters.")
        }
        FailFindingKeyword(keyword: String) {
            display("{} keyword has not been found.", keyword)
        }
        ValueBadParsing {
            display("A value could not be parsed correctly")
        }
        NotSupportedXtensionType(extension: String) {
            display("`{}` extension is not supported. Only BINTABLE, TABLE and IMAGE are.", extension)
        }
        Nom {
            from(nom::Err<nom::error::Error<&[u8]>>)
            display("Nom could not parse header values")
        }
        Utf8 {
            from(std::str::Utf8Error)
            display("Fail to parse a keyword as a utf8 string")
        }
    }
}