
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use crate::ast::{Expression, NodeF};

pub fn parse_expr<F: NodeF>(source: &str) -> F::Expression {
    let cm: Lrc<SourceMap> = Default::default();
        /*
        let handler = 
            Handler::with_emitter(true, false,
            Some(cm.clone()));
    */
        // Real usage
        // let fm = cm
        //     .load_file(Path::new("test.js"))
        //     .expect("failed to load test.js");
        let fm = cm.new_source_file(
            FileName::Custom("test.js".into()),
            source.into(),
        );
        let lexer = Lexer::new(
            // We want to parse ecmascript
            Syntax::Es(Default::default()),
            // EsVersion defaults to es5
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
    
        let mut parser = Parser::new_from(lexer);
    
        for e in parser.take_errors() {
            panic!("failed to parse module: {:?}", e);
        }
    
        let swcexpr = parser.parse_expr()
            .map_err(|mut e| {
                // Unrecoverable fatal error occurred
                panic!("failed to parse module: {:?}", e);
            })
            .expect("failed to parser module");

        let astexpr: Expression<F> = (*swcexpr).into();
        
        astexpr.into()
}
