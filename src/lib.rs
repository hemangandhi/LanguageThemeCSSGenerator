pub mod plist_reader {
    extern crate xml;

    use std::fs::File;
    use std::error::Error;
    use std::io::BufReader;
    use std::io::Error as IOError;
    use std::collections::HashMap;

    use self::xml::reader::{EventReader, XmlEvent};
    use self::xml::reader::Error as XMLError;

    pub enum PList {
        Dictionary(HashMap<String, PList>),
        StringProp(String),
        Array(Vec<PList>)
    }

    pub enum PListError{
        IOError(IOError),
        XmlError(XMLError),
        EarlyEOF
    }

    enum PListState {
        BeforeRoot,
        Root,
        GatherArray(PList, Box<PListState>),
        GatherDict(PList, Box<PListState>),
        GatherDictKey(PList, String, Box<PListState>),
        // ^ that PListState would always be GatherDict, but which one matters.
        Done(PList)
    }

    fn fix_plist(accum: PListState, event: XmlEvent) -> PListState {
        return accum;

    }

    pub fn try_read_file(path: String) -> Result<PList, PListError> {
        let mut input_file = match File::open(path) {
            Ok(handle) => handle,
            Err(io_error) => return Result::Err(PListError::IOError(io_error)),
        };
        let reader = EventReader::new(&mut input_file);

        return match reader.into_iter().fold(Result::Ok(PListState::BeforeRoot),
                                             |state_res, read| {
            state_res.and_then(|state| {
                read.and_then(|value| Result::Ok(fix_plist(state, value)))
            })
        }) {
            Ok(state) => match state {
                PListState::Done(val) => Result::Ok(val),
                _ => Result::Err(PListError::EarlyEOF),
            },
            Err(err) => Result::Err(PListError::XmlError(err)),
        }
    }
}
