
/* 
use crate::parser::{ParserState, CombinatoryParser, CombinatoryParserImpl, Never};
use crate::jessie_types::*;
use crate::jessie_parser::*;

#[derive(Debug, PartialEq, Clone)]
pub struct JessieParser(CombinatoryParserImpl);
impl ParserState for JessieParser {
    const IS_PURE_JSON_RULE: bool = false;
    const IS_JESSIE_RULE: bool = true;
    const IS_TESSIE_RULE: bool = false;



    type Expr = Expr<Self>;

    fn expr(state: &mut Self) -> Result<Self::Expr, String> {
       expr(state)
    } 
    
    fn pure_expr(state: &mut Self) -> Result<Self::Expr, String> {
        pure_expr(state)
    }

    fn primary_expr(state: &mut Self) -> Result<Self::Expr, String> {
        primary_expr(state)
    }

    type Function = Function<Self>;

    fn arrow_function(state: &mut Self) -> Result<Self::Function, String> {
        arrow_func(state)
    }

    fn function_decl(state: &mut Self) -> Result<Self::Function, String> {
        function_decl(state)
    }

    fn function_expr(state: &mut Self) -> Result<Self::Function, String> {
        function_expr(state)
    }

    type Element = Element<Self>;

    fn element(state: &mut Self) -> Result<Self::Element, String> {
        element(state)
    }

    type PropDef = PropDef<Self>;

    fn prop_def(state: &mut Self) -> Result<Self::PropDef, String> {
        prop_def(state)
    }

    fn pure_prop_def(state: &mut Self) -> Result<Self::PropDef, String> {
        pure_prop_def(state)
    }


    type PropName = String;

    fn prop_name(state: &mut Self) -> Result<Self::PropName, String> {
        Self::ident(state) // TODO
    }

    type Variable = String;

    fn variable(state: &mut Self) -> Result<Self::Variable, String> {
        Self::ident(state) // TODO
    }

    type TypeAnn = Never;

    fn optional_type_ann(state: &mut Self) -> Option<Self::TypeAnn> {
        None
    }
}
*/