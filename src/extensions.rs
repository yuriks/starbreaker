#[feature(macro_registrar, managed_boxes)];

extern mod syntax;

use syntax::ast;
use syntax::ast::{
	ExprLit,
	ExprVec,
	LitStr,
	Name,
	TokenTree,
	MutImmutable,
	Expr,
	LitUint,
	TyU16,

};
use syntax::codemap;
use syntax::codemap::{Span, dummy_spanned};
use syntax::ext::base::{
	SyntaxExtension,
	SyntaxExpanderTT,
	SyntaxExpanderTTExpanderWithoutContext,
	NormalTT,
	ExtCtxt,
	MacResult,
	MRExpr,
};
use syntax::parse::token;
use syntax::parse;

#[macro_registrar]
pub fn macro_registrar(register: |Name, SyntaxExtension|) {
	register(token::intern(&"ucs2_from_str"),
		NormalTT(~SyntaxExpanderTT {
				expander: SyntaxExpanderTTExpanderWithoutContext(expand_ucs2_from_str),
				span: None,
			}, None)
		);
}

pub fn expand_ucs2_from_str(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> MacResult {
	let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_owned());

	let expr = parser.parse_expr();

	let string = match expr.node {
		ExprLit(lit) => {
			match lit.node {
				LitStr(s, _) => s,
				_ => cx.span_fatal(expr.span, "expected string literal"),
			}
		}
		_ => cx.span_fatal(expr.span, "expected string literal"),
	};

	if parser.token != token::EOF {
		cx.span_fatal(parser.span, "unexpected garbage");
	}

	let characters = string.to_utf16() + ~[0u16];
	let mut char_vec = characters.iter().map(|&codepoint| {
		 @Expr {
			 id: ast::DUMMY_NODE_ID,
			 node: ExprLit(@dummy_spanned(LitUint(codepoint as u64, TyU16))),
			 span: codemap::DUMMY_SP,
		 }
	});

	MRExpr(@Expr {
		id: ast::DUMMY_NODE_ID,
		node: ExprVec(char_vec.to_owned_vec(), MutImmutable),
		span: sp,
	})
}
