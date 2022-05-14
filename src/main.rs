// use chumsky::prelude::*;

// use crochet::parser::*;
// use crochet::lexer::*;
// use crochet::context::{Context, Env};
// use crochet::infer::*;

// fn main() {
//     println!("Hello, world!");

//     let env: Env = HashMap::new();
//     let ctx = Context::from(env);
//     let result = lexer().parse("5 + 10").unwrap();
//     let spans: Vec<_> = result.iter().map(|(_, s)| s.to_owned()).collect();
//     let tokens: Vec<_> = result.iter().map(|(t, _)| t.to_owned()).collect();
//     let prog = token_parser(&spans).parse(tokens).unwrap();
//     let stmt = prog.body.get(0).unwrap();
//     let result = infer_stmt(&ctx, &stmt);

//     assert_eq!(format!("{}", result), "number");

//     let foo = "var add = (a, b => {\na + b\n};";
//     println!("foo = {}", foo);
//     println!("foo (debug) = {:?}", foo);
// }

use std::collections::HashMap;
use std::{path::Path, sync::Arc};

use swc_atoms::{js_word, JsWord};
use swc_common::{BytePos, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_parser::{error::Error, parse_file_as_module, Syntax, TsConfig};
use swc_ecma_visit::*;

// impl VisitMut for MatchExample {
//     fn visit_mut_callee(&mut self, callee: &mut Callee) {
//         callee.visit_mut_children_with(self);

//         if let Callee::Expr(expr) = callee {
//             // expr is `Box<Expr>`
//             if let Expr::Ident(i) = &mut **expr {
//                 i.sym = "foo".into();
//             }
//         }
//     }
// }

#[derive(Debug)]
pub struct InterfaceCollector {
    names: HashMap<BytePos, JsWord>,
    current_interface: Option<JsWord>,
    array_methods: Vec<JsWord>,
}

impl Visit for InterfaceCollector {
    // call this if we don't want to visit TypeScript type nodes
    // noop_visit_type!();

    // fn visit_ident(&mut self, ident: &Ident) {
    //     self.names.insert(ident.span.lo, ident.sym.clone());
    // }

    fn visit_ts_interface_decl(&mut self, decl: &TsInterfaceDecl) {
        self.names.insert(decl.id.span.lo, decl.id.sym.clone());
        self.current_interface = Some(decl.id.sym.clone());
        decl.visit_children_with(self);
        self.current_interface = None;
    }

    fn visit_ts_type_element(&mut self, elem: &TsTypeElement) {
        if let Some(word) = &self.current_interface {
            if word == &js_word!("Array") {
                match elem {
                    TsTypeElement::TsCallSignatureDecl(decl) => {}
                    TsTypeElement::TsConstructSignatureDecl(decl) => {}
                    TsTypeElement::TsPropertySignature(sig) => {}
                    TsTypeElement::TsGetterSignature(sig) => {}
                    TsTypeElement::TsSetterSignature(sig) => {}
                    TsTypeElement::TsMethodSignature(sig) => {
                        if sig.computed {
                            panic!("unexpected computed property in TypElement")
                        }
                        let _ = match sig.key.as_ref() {
                            Expr::Ident(Ident {sym, ..}) => {
                                self.array_methods.push(sym.clone())
                            }
                            _ => {}
                        };
                        println!("key: {:#?}", sig.key.as_ref());
                        // TODO: convert sig.params to crochet type
                        for param in &sig.params {
                            match param {
                                TsFnParam::Ident(ident) => {
                                    let word = &ident.id.sym;
                                    let type_ann = ident.type_ann.clone().unwrap();
                                    println!("{word}: {:?}", type_ann);
                                },
                                _ => {}
                            }
                        }
                    }
                    TsTypeElement::TsIndexSignature(sig) => {}
                }
                // self.array_methods.push(elem)
                // println!("{:?}", elem);
            }
        }
    }
}

fn main() {
    let cm = Arc::<SourceMap>::default();
    let fm = cm
        .load_file(Path::new("node_modules/typescript/lib/lib.es5.d.ts"))
        .expect("failed to load file");

    let mut errors: Vec<Error> = vec![];
    let module = parse_file_as_module(
        &fm,
        Syntax::Typescript(TsConfig {
            tsx: false,
            dts: true,
            decorators: false,
            no_early_errors: false,
        }),
        EsVersion::Es2020,
        None, // comments
        &mut errors,
    );

    match module {
        Ok(module) => {
            let mut v = InterfaceCollector {
                names: Default::default(),
                current_interface: None,
                array_methods: vec![],
            };
            module.visit_with(&mut v);
            println!("lib.es5.d.ts interfaces");
            println!("v.array_methods = {:#?}", v.array_methods);
            println!("errors = {:#?}", errors);
        }
        Err(err) => println!("err = {:?}", err),
    }
}
