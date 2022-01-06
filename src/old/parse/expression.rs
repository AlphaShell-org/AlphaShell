use super::{
  general::{find_next_of},
  ParseHelper, Result,  TokenType, node::Node, error::Error,
};

fn child_comma_count( expr: Node ) -> usize
{
	let cc;
	if let Some(left) = expr.left {
		if left.type_() == Grammar::Expression  {
		let e = *left as Expression;
		if let Some(operator) = e.operator  {
			if operator.value.0 == TokenType::Comma {
				cc += child_comma_count( e );
			}}
	}
}
	if( expr.m_rhs && expr.m_rhs.m_type == GRAM_EXPR ) {
		const stmt_expr_t * e = ( const stmt_expr_t * )expr.m_rhs;
		if( e.m_oper != nullptr && e.m_oper.m_val.type == TOK_COMMA ) child_comma_count( e, cc );
	}
	if( expr.m_oper && expr.m_oper.m_val.type == TOK_COMMA ) ++cc;
	cc
}

pub fn parse_expression(ph: &mut ParseHelper, end: Option<usize>) -> Result<Node> {
  let data: Vec<Node> = Vec::new(); // std::vector< stmt_base_t * >
	let stack = Vec::new(); // std::vector< stmt_simple_t * >


  let end = end.unwrap_or_else(|| ph.tokens.len() - 1);

	let triple_dot_ctr = 0;
	let start = ph.index;

	while let Some( current) = ph.peak(0) {

		if current == TokenType::Invalid || current == TokenType::Semicolon {break;}

		if(
			(current == TokenType::Identifier|| current == TokenType::String) // collection or string
			&& ph.peak(1) == Some(TokenType::LBracket)
		)||
		(
			current == TokenType::LBracket
			&& !data.is_empty()	&& data.last().unwrap().type_ == Grammar::FN_STRUCT_SUBSCR_CALL
		)
		{
			if current == TokenType::LBracket && data.len() > 0
			{
				let rbrack_loc = find_next_of(ph, &[TokenType::RBracket], TokenType::LBracket)?;
				
				ph.advance();
				let subscr = parse_expression( ph, Some(rbrack_loc))?;
				
				data.last_mut().unwrap().args.push(subscr);
			}
			else
			{
				let name = ph.peak(0);
				let tok_val = ph.index;
				ph.advance();

				let rbrack_loc = find_next_of(ph, &[TokenType::RBracket], TokenType::LBracket)?;

				ph.advance();

				let subscr = parse_expression( ph, Some(rbrack_loc))?;

				
				let subscr_call = stmt_func_struct_subscr_call_t(stmt_simple_t(SIMPLE_TOKEN, name, tok_val),{subscr.expr}, tok_val);

				subscr_call.m_ctype = CT_SUBSCR;
				if let Some(last) = stack.last() {if last.m_val.type_ == TokenType::Dot	{	subscr_call.m_post_dot = true;}}
				data.push(subscr_call);
			}
		}
		else if current == TokenType::Identifier && ph.peak(1) == Some(TokenType::LParen)
		{
			let name = current;
			let tok_val = ph.index;

			ph.advance();

			let rbrack_loc = find_next_of(ph, &[TokenType::RParen], TokenType::LParen)?;
			
			ph.advance();

			let fn_args = parse_expression( ph, rparen_loc)?;

			
			let fn_call = stmt_func_struct_subscr_call_t(stmt_simple_t(SIMPLE_TOKEN, name, tok_val),{fn_args.expr}, tok_val);
			if !stack.empty() && stack.last().unwrap().m_val.type_ == TokenType::Dot{	fn_call.m_post_dot = true;}
			data.push_back(fn_call);
		}
		else if current == TokenType::LBracket || current == TokenType::LBrace
		{
			let tok_val = ph.index;
			let eq = if current == TokenType::LBrace { TokenType::RBrace} else{ TokenType::RBracket};
		
			let r_loc = find_next_of(ph, &[eq], current)?;
			
			
			ph.advance();

			let collect = parse_expression( ph, Some(r_loc))?;

			
			let child_cc = child_comma_count(collect);
			let args = child_cc + 1;
			
			if current == TokenType::LBrace && args % 2 != 0
			{
				return Err(Error::new("map requires arguments in the multiples of 2 for keys and their values", None));
			}

			let collect_call =  stmt_collection_t(collect, tok_val, ph.at(tok_val).line, ph.at(tok_val).col);
			if current == TokenType::LBrace	{collect_call.m_is_map = true;}
			collect_call.m_arg_count = args;
			data.push(collect_call);
		}
		else
		{
			if token_is_data(current)
			{
				if (triple_dot_ctr > 0)
				{
					PARSE_FAIL("no data can come after the '...' argument");
					goto fail;
				}
				if (ph.peak(0) == TOK_TDOT)
					++triple_dot_ctr;

				stmt_simple_t *sim = new stmt_simple_t(
						ph.peak().data == TokStrs[ph.peak(0)] &&
										ph.peak(0) != TOK_IDEN &&
										ph.peak(0) != TOK_STR
								? SIMPLE_KEYWORD
								: SIMPLE_TOKEN,
						ph.peak(), ph.tok_ctr());
				if (!stack.empty() && stack.back().m_val.type == TOK_DOT)
					sim.m_post_dot = true;
				data.push_back(sim);
				ph.advance();
				continue;
			}
			if !token_is_oper(current)
			{
				if current == TokenType::Invalid
				{
					return Err(Error::new("failed parsing expression, possibly missing semicolon after this", Some(ph.token())));
				}
				
					return Err(Error::new(&format!("invalid token '{:?}' while parsing expression", ph.peak(0)), Some(ph.token())));
				
			}
			// handle parentheses
			if current == TokenType::RParen || current ==  TokenType::RBrace || current ==  TokenType::RBracket
			{
				let found = false;
				let eq = match current {
					TokenType::RParen => TokenType::LParen,
					TokenType::RBrace => TokenType::LBrace,
					TokenType::RBracket=>TokenType::LBracket,
					_ => panic!("Unexpected token")
				};

				while let Some(last )= stack.last()
				{
					if last.m_val.type_ == eq
					{
						stack.pop();
						found = true;
						break;
					}
					data.push(stack.pop());
					
				}

				if !found
				{
					return Err(Error::new("could not find equivalent beginning for '{eq}'", None));
				}
			}
			// handle unary +/-
			if
				(current == TokenType::Add || current == TokenType::Sub)
				&&
				(
					ph.index == start
					|| (
						ph.index > start
						&& token_is_oper(ph.peak(-1))
						&& ph.peak(-1) != Some(TokenType::RParen)
						&& ph.peak(-1) != Some(TokenType::RBrace)
						&& ph.peak(-1) != Some(TokenType::RBracket)
					)
				)
			
			{
				if current == TokenType::Sub
				{
					ph.tokens[ph.index].0 = TokenType::UnarySub;
				}
				else if current == TokenType::Add
				{
					ph.tokens[ph.index].0 = TokenType::UnaryAdd;
				}
			}

			// do the actual work
			let prec = oper_prec(current);
			if !stack.is_empty()
			{
				while let  Some(stack_operator) =stack.last(){
					if stack_operator.m_val.type_ == TokenType::LParen
					|| stack_operator.m_val.type_ == TokenType::LBrace
					|| stack_operator.m_val.type_ == TokenType::LBracket
					|| prec >= oper_prec(stack_operator.m_val)
				{break};
					data.push(stack_operator);
					stack.pop();
					
				}
				if let Some(stack_operator) = stack.last(){
					if stack_operator.m_val.type_ == TokenType::LParen
					|| stack_operator.m_val.type_ == TokenType::LBrace
					|| stack_operator.m_val.type_ == TokenType::LBracket
					|| prec == oper_prec(stack_operator.m_val)
				{break};
					if (oper_assoc(stack_operator.m_val) == LTR)
					{
						data.push(stack_operator);
						stack.pop();
					}
				}
			}
			stack.push(stmt_simple_t(SIMPLE_OPER, ph.peak(), ph.tok_ctr()));
		}
		ph.advance();
	}

	for s in stack
	{
		if s.m_val.type_ == TokenType::LParen
		{
			return Err(Error::new("could not find ending parentheses", None));
		}
	}

	while let Some(last) = stack.pop()
	{
		data.push(last);
	}

	if data.empty()
	{
		return Err(Error::new("No data in expression", None));
	}

	let res = gen_tree( ph, data )?;

	// if (is_top)
	// {
	// 	res.m_is_top_expr = true;
	// }

	// if triple_dot_ctr > 0
	// {
	// 	res.m_triple_dot = true;
	// }

	Ok(res)
}

fn gen_tree(ph: &mut ParseHelper,  data: Vec<DynNode>) -> Result<Expression>
{
	let var_stack: Vec<DynNode>;
	let start_tok_ctr = data.first().m_tok_ctr;

	for node in data
	{
		if node.type_ != Grammar::Simple || (*node as SimpleNode).simple_type != Simple::Operator
		{
			var_stack.push(it);
			continue;
		}

		let op = (*node as SimpleNode).val;
		if var_stack.is_empty()
		{
			return Err(Error::new(&f!("no operands for operator '{it}'"), None));
		}

		let arg_count = oper_arg_count(op)?;
		/* if (arg_count < 0)
		{
			ph.set_tok_ctr((*it).m_tok_ctr);
			PARSE_FAIL("the operator '%s' should not be here", TokStrs[op.type]);
			goto fail;
		} */

		if var_stack.len() < arg_count
		{
			let available = 0;
			for (let it = var_stack.rbegin(); it != var_stack.rend(); ++it)
			{
				if (((stmt_simple_t *)(*it)).m_stype == SIMPLE_OPER)
					break;
				++available;
			}
			ph.set_tok_ctr((*it).m_tok_ctr);
			PARSE_FAIL("not enough arguments for operator '%s' (expected: %d, available: %d) "
								 "(possibly an extra comma or a trailing dot)",
								 TokStrs[op.type], arg_count, available);
			goto fail;
		}

		stmt_base_t *top2 = nullptr;
		if (arg_count >= 1)
		{
			top2 = var_stack.back();
			var_stack.pop_back();
		}

		stmt_base_t *top1 = nullptr;
		if (arg_count >= 2)
		{
			top1 = var_stack.back();
			var_stack.pop_back();
		}

		/*	oper
			/  \
		      top1 top2
		*/
		if (token_is_one_of_assign(op) &&
				((top1 == nullptr || ((top1.m_type == GRAM_SIMPLE &&
															 ((stmt_simple_t *)top1).m_val.type != TOK_IDEN) &&
															top1.m_type != GRAM_EXPR))))
		{
			ph.set_tok_ctr((*it).m_tok_ctr);
			PARSE_FAIL("expected an lvalue on the left of the assignment operator");
			if (arg_count >= 1)
				delete top2;
			if (arg_count >= 2)
				delete top1;
			goto fail;
		}

		stmt_expr_t *expr = new stmt_expr_t(top1, (stmt_simple_t *)*it, top2, (*it).m_tok_ctr);
		it = data.erase(it);
		var_stack.push_back(expr);
	}

	if (var_stack.empty() || var_stack.size() > 1)
	{
		ph.set_tok_ctr(start_tok_ctr);
		PARSE_FAIL("invalid expression, should generate single value, but is generating %zu", var_stack.size());
		goto fail;
	}

	if (var_stack.back().m_type != GRAM_EXPR)
	{
		stmt_base_t *data = var_stack.back();
		var_stack.pop_back();
		stmt_expr_t *expr = new stmt_expr_t(nullptr, nullptr, data, data.m_tok_ctr);
		var_stack.push_back(expr);
	}

	return (stmt_expr_t *)var_stack.back();

fail:
	for (auto &vs : var_stack)
		delete vs;
	return nullptr;
}
