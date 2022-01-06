use super::{node::Node, Error, ParseHelper, Result, TokenType};


pub fn is_next_valid(ph: &mut ParseHelper, valid_tokens: &[TokenType]) -> Result<()> {
    let found = ph.peak(1).unwrap_or_default();
    if !valid_tokens.contains(&found) {
        let valid_tokens_string = valid_tokens
            .iter()
            .map(|token| f!("{token}"))
            .collect::<Vec<_>>()
            .join("' or '");
        return Err(Error::new(
            &f!("expected '{valid_tokens_string}', but found {found}"),
            Some(ph.token()),
        ));
    }
    ph.advance();
    Ok(())
}

pub fn find_next_of(
    ph: &mut ParseHelper,
    types: &Vec<TokenType>,
    eq: TokenType,
    bypass_breaker: bool,
    breaker: TokenType,
) -> Result<usize> {
    let ctr = 1;
    let equi = 1;
    while ph.peak(ctr) != Some(TokenType::Invalid) {
        if ph.peak(ctr) == Some(breaker) && !types.contains(&breaker) && !bypass_breaker {
            return Err(Error::new(
                &f!("Found unexpected semicolon before '{eq}'"),
                None,
            ));
        }
        if ph.peak(ctr).is_some() && types.contains(&ph.peak(ctr).unwrap()) {
            if eq != TokenType::Invalid {
                equi -= 1;
                if equi == 0 {
                    return Ok(ph.index + ctr as usize);
                };
            } else {
                return Ok(ph.index + ctr as usize);
            }
        }
        if ph.peak(ctr) == Some(eq) {
            equi += 1;
        }
        ctr += 1;
    }
    return Err(Error::new(
        &f!("could not find the equivalent ending token for '{eq}'"),
        None,
    ));
}

// #define NEXT_VALID_FAIL( tok )							\
// do {										\
// 	if( ph.peak( 1 ).type == TOK_INVALID ) {				\
// 		PARSE_FAIL( "expected token '%s', but found <EOF>",		\
// 			    TokStrs[ tok ] );					\
// 		goto fail;							\
// 	}									\
// 	ph.next();								\
// 	if( ph.peak().type != tok ) {						\
// 		PARSE_FAIL( "expected token '%s', but found '%s'",		\
// 			    TokStrs[ tok ], TokStrs[ ph.peak().type ] );	\
// 		goto fail;							\
// 	}									\
// } while( 0 )

// #define NEXT_VALID2_FAIL( tok1, tok2 )								\
// do {												\
// 	if( ph.peak( 1 ).type == TOK_INVALID ) {						\
// 		PARSE_FAIL( "expected token '%s' or '%s', but found <EOF>",			\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ] );					\
// 		goto fail;									\
// 	}											\
// 	ph.next();										\
// 	if( ph.peak().type != tok1 && ph.peak().type != tok2 ) {				\
// 		PARSE_FAIL( "expected token '%s' or '%s', but found '%s'",			\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ], TokStrs[ ph.peak().type ] );	\
// 		goto fail;									\
// 	}											\
// } while( 0 )

// #define NEXT_VALID3_FAIL( tok1, tok2, tok3 )							\
// do {												\
// 	if( ph.peak( 1 ).type == TOK_INVALID ) {						\
// 		PARSE_FAIL( "expected token '%s' or '%s' or '%s', but found <EOF>",		\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ], TokStrs[ tok3 ] );		\
// 		goto fail;									\
// 	}											\
// 	ph.next();										\
// 	if( ph.peak().type != tok1 && ph.peak().type != tok2 && ph.peak().type != tok3 ) {\
// 		PARSE_FAIL( "expected token '%s' or '%s' or '%s', but found '%s'",		\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ],					\
// 			    TokStrs[ tok3 ], TokStrs[ ph.peak().type ] );			\
// 		goto fail;									\
// 	}											\
// } while( 0 )

// #define NEXT_VALID4_FAIL( tok1, tok2, tok3, tok4 )						\
// do {												\
// 	if( ph.peak( 1 ).type == TOK_INVALID ) {						\
// 		PARSE_FAIL( "expected token '%s' or '%s' or '%s' or '%s', but found <EOF>",	\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ], TokStrs[ tok3 ],			\
// 			    TokStrs[ tok4 ] );							\
// 		goto fail;									\
// 	}											\
// 	ph.next();										\
// 	if( ph.peak().type != tok1 && ph.peak().type != tok2 &&				\
// 	    ph.peak().type != tok3 && ph.peak().type != tok4 ) {				\
// 		PARSE_FAIL( "expected token '%s' or '%s' or '%s', but found '%s'",		\
// 			    TokStrs[ tok1 ], TokStrs[ tok2 ],					\
// 			    TokStrs[ tok3 ], TokStrs[ tok4 ],					\
// 			    TokStrs[ ph.peak().type ] );					\
// 		goto fail;									\
// 	}											\
// } while( 0 )
