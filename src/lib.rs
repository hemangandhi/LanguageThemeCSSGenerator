pub mod plist_reader {
    extern crate xml;

    use std::fs::File;
    use std::error::Error;
    use std::io::BufReader;
    use std::io::Error as IOError;
    use std::collections::HashMap;

    use self::xml::reader::{EventReader, XmlEvent};
    use self::xml::reader::Error as XMLError;
    use self::xml::named::OwnedName;

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

    fn fix_plist(accum: PListState, event: XmlEvent) -> Result<PListState, PListError> {
        return match event {
            XmlEvent::ProcessingInstruction{..} |
                XmlEvent::CData(_) |
                XmlEvent::Whitespace(_) |
                XmlEvent::Comment(_) |
                XmlEvent::StartDocument{..} |
                XmlEvent::EndDocument => Result::Ok(accum),
            XmlEvent::StartElement{ name, .. } => {
                let real_name = name.local_name;
                Result::Ok(accum)
                // match accum {
                //     BeforeRoot => if name == "plist" {
                //             Result::Ok(Root)
                //         } else {
                //             Result::Ok(BeforeRoot)
                //         },
                //     Root => 
                // }
            }
            _ => Result::Ok(accum),
        };
    }

    pub fn try_read_file(path: String) -> Result<PList, PListError> {
        let mut input_file = match File::open(path) {
            Ok(handle) => handle,
            Err(io_error) => return Result::Err(PListError::IOError(io_error)),
        };
        let reader = EventReader::new(&mut input_file);

        return reader.into_iter().fold(Result::Ok(PListState::BeforeRoot),
                                             |state_res, read| {
            state_res.and_then(|state| {
                read.or_else(|xml_err| Result::Err(PListError::XmlError(xml_err)))
                    .and_then(|value| fix_plist(state, value))
            })
        }).and_then(|result| match result {
            PListState::Done(value) => Result::Ok(value),
            _ => Result::Err(PListError::EarlyEOF),
        })
    }
}
