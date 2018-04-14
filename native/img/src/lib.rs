#[macro_use]
extern crate rustler;
#[allow(unused_imports)]
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate image;
use std::fs::File;
use std::path::Path;
use rustler::{NifEncoder, NifEnv, NifError, NifResult, NifTerm};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.Image",
    [("add", 2, add),("flip", 2, flip)],
    None
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn flip<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let input: String = args[0].decode()?;
    let output: String = args[1].decode()?;
    let img =
        image::open(&Path::new(&input)).map_err(|_e| NifError::Atom("Cannot open input file"))?;
    let filtered = img.fliph();
    let mut out =
        File::create(&Path::new(&output)).map_err(|_e| NifError::Atom("Cannot create a new file"))?;
    let _ = filtered
        .save(&mut out, image::PNG)
        .map_err(|_e| NifError::Atom("Cannot save new file in PNG"))?;
    Ok((atoms::ok(), output).encode(env))
}
