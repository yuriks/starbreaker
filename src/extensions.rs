#[feature(macro_registrar, managed_boxes)];

extern mod syntax;

use syntax::ast::{
	LitUint,
	Name,
	TokenTree,
	TyU16,
};
use syntax::codemap::Span;
use syntax::ext::base::{
	ExtCtxt,
	MRExpr,
	MacResult,
	NormalTT,
	SyntaxExpanderTT,
	SyntaxExpanderTTExpanderWithoutContext,
	SyntaxExtension,
	get_single_str_from_tts,
};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

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
	let string = get_single_str_from_tts(cx, sp, tts, "expand_ucs2_from_str");

	let characters = string.to_utf16() + ~[0u16];
	let mut char_vec = characters.iter().map(|&codepoint| {
		cx.expr_lit(sp, LitUint(codepoint as u64, TyU16))
	});

	MRExpr(cx.expr_vec(sp, char_vec.to_owned_vec()))
}
