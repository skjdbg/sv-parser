use crate::parser::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ProgramInstantiation<'a> {
    pub nodes: (
        ProgramIdentifier<'a>,
        Option<ParameterValueAssignment<'a>>,
        HierarchicalInstance<'a>,
        Vec<(Symbol<'a>, HierarchicalInstance<'a>)>,
        Symbol<'a>,
    ),
}

// -----------------------------------------------------------------------------

pub fn program_instantiation(s: Span) -> IResult<Span, ProgramInstantiation> {
    let (s, a) = program_identifier(s)?;
    let (s, b) = opt(parameter_value_assignment)(s)?;
    let (s, c) = hierarchical_instance(s)?;
    let (s, d) = many0(pair(symbol(","), hierarchical_instance))(s)?;
    let (s, e) = symbol(";")(s)?;
    Ok((
        s,
        ProgramInstantiation {
            nodes: (a, b, c, d, e),
        },
    ))
}

// -----------------------------------------------------------------------------