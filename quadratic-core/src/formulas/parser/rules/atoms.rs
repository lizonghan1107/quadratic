use super::*;

/// Matches a string literal.
#[derive(Debug, Copy, Clone)]
pub struct StringLiteral;
impl_display!(for StringLiteral, "string literal");
impl SyntaxRule for StringLiteral {
    type Output = ast::AstNode;

    fn prefix_matches(&self, mut p: Parser<'_>) -> bool {
        p.next() == Some(Token::StringLiteral)
    }
    fn consume_match(&self, p: &mut Parser<'_>) -> FormulaResult<Self::Output> {
        if p.next() != Some(Token::StringLiteral) {
            return p.expected(self);
        }
        // Use IIFE for error handling.
        || -> Option<Self::Output> {
            let mut string_contents = String::new();
            let mut chars = p.token_str().chars().peekable();
            let quote = chars.next()?;
            // Read characters.
            loop {
                match chars.next()? {
                    '\\' => string_contents.push(chars.next()?),
                    c if c == quote => break,
                    c => string_contents.push(c),
                }
            }

            if chars.next().is_none() {
                // End of token, as expected.
                Some(Spanned {
                    span: p.span(),
                    inner: ast::AstNodeContents::String(string_contents),
                })
            } else {
                // Why is there more after the closing quote?
                None
            }
        }()
        .ok_or_else(|| internal_error_value!("error in string literal parsing"))
    }
}

/// Matches an numeric literal.
#[derive(Debug, Copy, Clone)]
pub struct NumericLiteral;
impl_display!(for NumericLiteral, "numeric literal, such as '42' or '6.022e23'");
impl SyntaxRule for NumericLiteral {
    type Output = AstNode;

    fn prefix_matches(&self, mut p: Parser<'_>) -> bool {
        p.next() == Some(Token::NumericLiteral)
    }
    fn consume_match(&self, p: &mut Parser<'_>) -> FormulaResult<Self::Output> {
        match p.next() {
            Some(Token::NumericLiteral) => {
                let Ok(n) = p.token_str().parse::<f64>() else {
                    return Err(FormulaErrorMsg::BadNumber.with_span(p.span()));
                };
                Ok(AstNode {
                    span: p.span(),
                    inner: ast::AstNodeContents::Number(n),
                })
            }
            _ => p.expected(self),
        }
    }
}

/// Matches a cell reference.
pub struct CellReference;
impl_display!(for CellReference, "cell reference, such as 'A6' or '$ZB$3'");
impl SyntaxRule for CellReference {
    type Output = AstNode;

    fn prefix_matches(&self, mut p: Parser<'_>) -> bool {
        p.next() == Some(Token::CellRef)
    }
    fn consume_match(&self, p: &mut Parser<'_>) -> FormulaResult<Self::Output> {
        p.next();
        let Some(cell_ref) = CellRef::parse_a1(p.token_str(), p.loc) else {
            return Err(FormulaErrorMsg::BadCellReference.with_span(p.span()));
        };
        Ok(AstNode {
            span: p.span(),
            inner: ast::AstNodeContents::CellRef(cell_ref),
        })
    }
}
