use super::{
  general::{is_next_valid, Node}, statement::SimpleType,
  ParseHelper, Result, Token, TokenType,
};

struct FunctionNode {
  name: SimpleType,
  args: Vec<Token>,
  
}

impl Node for FunctionNode {}

pub fn parse_function(ph: &mut ParseHelper) -> Result<FunctionNode> {
  int tok_ctr = ph->tok_ctr();

	bool is_member_func = false;
	if( ph->peak()->type == TOK_MFN ) is_member_func = true;

	std::vector< GrammarTypes > parents;

	std::vector< stmt_simple_t * > mem_types;
	if( is_member_func ) {
		NEXT_VALID( TOK_LT );
	begin_mfn_type:
		NEXT_VALID2( TOK_IDEN, TOK_STR );
		mem_types.push_back( new stmt_simple_t( SIMPLE_TOKEN, ph->peak(), ph->tok_ctr() ) );
		NEXT_VALID2( TOK_GT, TOK_COMMA );
		if( ph->peak()->type == TOK_COMMA ) goto begin_mfn_type;
	}

	const tok_t * name = nullptr;
	NEXT_VALID2( TOK_IDEN, TOK_STR );
	name = ph->peak();

	NEXT_VALID( TOK_LPAREN );

	expr_res_t arg_expr = { 0, nullptr };
	int arg_expr_end;
	int err;
	stmt_block_t * block = nullptr;

	NEXT_VALID3( TOK_RPAREN, TOK_IDEN, TOK_TDOT );

	if( ph->peak()->type == TOK_RPAREN ) goto end_args;
	err = find_next_of( ph, arg_expr_end, { TOK_RPAREN }, TOK_LPAREN );
	if( err < 0 ) {
		if( err == -1 ) {
			PARSE_FAIL( "could not find the right parenthesis for function '%s'",
				    name->data.c_str() );
		} else if( err == -2 ) {
			PARSE_FAIL( "found beginning of block (left braces) before the ')' for arguments of function '%s'",
				    name->data.c_str() );
		}
		goto fail;
	}
	arg_expr = parse_expr( src, ph, arg_expr_end, false );
	if( arg_expr.res != 0 ) goto fail;
	ph->set_tok_ctr( arg_expr_end );
end_args:
	NEXT_VALID_FAIL( TOK_LBRACE );
	parents.push_back( GRAM_FUNC );
	block = parse_block( src, ph, parents );
	parents.pop_back();
	if( block == nullptr ) goto fail;
	return new stmt_func_t( new stmt_simple_t( SIMPLE_TOKEN, name, tok_ctr + 1 ),
				arg_expr.expr, block, mem_types, tok_ctr );
fail:
	if( arg_expr.expr ) delete arg_expr.expr;
	if( block ) delete block;
	for( auto & mem_type : mem_types ) delete mem_type;
	return nullptr;

  return Ok(FunctionNode { target });
}
