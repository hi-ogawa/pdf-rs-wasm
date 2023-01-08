// Considering whether to impl Object and IndirectObject here.
//

use crate::crypt::Decoder;
use crate::error::*;
use crate::object::*;
use crate::parser::{lexer::*, MAX_DEPTH};
use crate::parser::{parse_stream_with_lexer, parse_with_lexer_ctx, Context, ParseFlags};
use crate::primitive::{PdfStream, Primitive};

/// Parses an Object starting at the current position of `lexer`. Almost as
/// `Reader::parse_object`, but this function does not take `Reader`, at the expense that it
/// cannot dereference

pub fn parse_indirect_object(
    lexer: &mut Lexer,
    r: &impl Resolve,
    decoder: Option<&Decoder>,
    flags: ParseFlags,
) -> Result<(PlainRef, Primitive)> {
    let id = PlainRef {
        id: t!(lexer.next()).to::<ObjNr>()?,
        gen: t!(lexer.next()).to::<GenNr>()?,
    };
    lexer.next_expect("obj")?;

    let ctx = Context { decoder, id };
    let obj = t!(parse_with_lexer_ctx(lexer, r, Some(&ctx), flags, MAX_DEPTH));

    if r.options().allow_missing_endobj {
        let pos = lexer.get_pos();
        if let Err(e) = lexer.next_expect("endobj") {
            warn!("error parsing obj {} {}: {:?}", id.id, id.gen, e);
            lexer.set_pos(pos);
        }
    } else {
        t!(lexer.next_expect("endobj"));
    }

    Ok((id, obj))
}
pub fn parse_indirect_stream(
    lexer: &mut Lexer,
    r: &impl Resolve,
    decoder: Option<&Decoder>,
) -> Result<(PlainRef, PdfStream)> {
    let id = PlainRef {
        id: t!(lexer.next()).to::<ObjNr>()?,
        gen: t!(lexer.next()).to::<GenNr>()?,
    };
    lexer.next_expect("obj")?;

    let ctx = Context { decoder, id };
    let stm = t!(parse_stream_with_lexer(lexer, r, &ctx));

    t!(lexer.next_expect("endobj"));

    Ok((id, stm))
}
