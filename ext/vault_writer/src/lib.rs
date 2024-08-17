use magnus::{function, prelude::*, Error, Ruby};
use nom::{bytes::complete::{tag, take_until}, combinator::{map, verify}, number::complete::{le_u32, le_u8}, sequence::tuple, IResult};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("VaultWriter")?;
    module.define_singleton_method("mark_saved", function!(mark_saved, 1))?;
    Ok(())
}

fn save_marker_offset(input: &[u8]) -> IResult<&[u8], usize> {
    map(
        tuple((
            take_until("DATAAUTO"),            
            tag("DATAAUTO"),
            verify(le_u32, |n| *n == 1),
            verify(le_u32, |n| *n == 1),
            verify(le_u32, |n| *n == 0),
            verify(le_u8, |n| *n == 0 || *n == 1)
        )),
        |(slice, _, _, _, _, _): (&[u8], &[u8], u32, u32, u32, u8)| slice.len() + 20
    )(input)
}

fn mark_saved(mut input: Vec<u8>) -> Vec<u8> {
    let (_, offset) = save_marker_offset(&input).unwrap();
    input[offset] = 0;
    input
}
