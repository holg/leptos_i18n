use proc_macro2::Ident;
use syn::token::Comma;
use syn::{Expr, Token};

use super::interpolate::InterpolatedValue;

pub enum Keys {
    SingleKey(Ident),
    Subkeys(Vec<Ident>),
    Namespace(Ident, Vec<Ident>),
}

pub struct ParsedInput {
    pub context: Expr,
    pub keys: Keys,
    pub interpolations: Option<Vec<InterpolatedValue>>,
}

impl syn::parse::Parse for ParsedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let context = input.parse()?;
        input.parse::<Comma>()?;
        let keys = input.parse()?;
        let comma = input.parse::<Comma>();
        let interpolations = match comma {
            Ok(_) => {
                let interpolations = input
                    .parse_terminated(InterpolatedValue::parse, Comma)?
                    .into_iter()
                    .collect();
                Some(interpolations)
            }
            Err(_) if input.is_empty() => None,
            Err(err) => return Err(err),
        };
        Ok(ParsedInput {
            context,
            keys,
            interpolations,
        })
    }
}

fn parse_subkeys(input: syn::parse::ParseStream, keys: &mut Vec<Ident>) -> syn::Result<()> {
    keys.push(input.parse()?);
    while input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        keys.push(input.parse()?);
    }
    Ok(())
}

impl syn::parse::Parse for Keys {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first_key = input.parse()?;
        if input.peek(Token![::]) {
            input.parse::<Token![::]>()?;
            let mut keys = vec![];
            parse_subkeys(input, &mut keys)?;
            Ok(Keys::Namespace(first_key, keys))
        } else if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let mut keys = vec![first_key];
            parse_subkeys(input, &mut keys)?;
            Ok(Keys::Subkeys(keys))
        } else {
            Ok(Keys::SingleKey(first_key))
        }
    }
}
