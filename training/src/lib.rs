extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;

use rustling::*;
use rustling_ontology_moment::*;
pub use rustling_ontology_rules::dimension::*;
pub use rustling_ontology_rules::output::*;
pub use rustling_ontology_rules::output::ParsingContext;

macro_rules! example {
    ($v:expr, $check:expr, $($ex:expr),*) => {
        $( $v.push($crate::rustling::Example::new($ex, Box::new($check))); )*
    };
}

#[macro_use]
mod macros;
pub mod en;
pub mod es;
pub mod fr;


macro_rules! lang {
    ($lang:ident, [$($example:ident),*]) => {
        pub fn $lang() -> Vec<::rustling::train::Example<Dimension>> {
            let mut v = vec![];
            $( $lang::$example(&mut v); )*
            v
        }
    }
}

lang!(en, [examples_numbers, examples_time]);
lang!(fr, [examples_numbers]);
lang!(es, [examples_numbers]);

#[derive(Debug)]
pub struct CheckInteger {
    pub value: i64,
}

impl Check<Dimension> for CheckInteger {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        IntegerValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_integer(v: i64) -> CheckInteger {
    CheckInteger { value: v }
}

#[derive(Debug)]
pub struct CheckOrdinal {
    pub value: i64,
}

impl Check<Dimension> for CheckOrdinal {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        OrdinalValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_ordinal(v: i64) -> CheckOrdinal {
    CheckOrdinal { value: v }
}

#[derive(Debug)]
pub struct CheckFloat {
    pub value: f32,
}

impl Check<Dimension> for CheckFloat {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        FloatValue::attempt_from(pn.value.clone())
            .map(|v| v.value == self.value)
            .unwrap_or(false)
    }
}

pub fn check_float(v: f32) -> CheckFloat {
    CheckFloat { value: v }
}

#[derive(Debug)]
pub struct CheckMoment {
    pub direction: Option<Direction>,
    pub precision: Precision,
    pub interval: Interval,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMoment {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        let check_value = self.context.resolve(&pn.value)
            .and_then(|v| TimeOutput::attempt_from(v))
            .map(|v| v.0.start == self.interval.start && v.0.grain == self.interval.grain)
            .unwrap_or(false);
        let time_value = TimeValue::attempt_from(pn.value.clone());
        let check_direction = time_value.clone()
            .map(|tv| tv.direction == self.direction)
            .unwrap_or(false);
        let check_precision = time_value.clone()
            .map(|tv| tv.direction == self.direction)
            .unwrap_or(false);
        check_value && check_precision && check_direction

    }
}

pub fn check_moment(context: ParsingContext, moment: Moment, grain: Grain, precision: Precision, direction: Option<Direction>)
                      -> CheckMoment {
    CheckMoment { 
        direction: direction,
        precision: precision,
        interval: Interval::starting_at(moment, grain),
        context: context
    }
}

#[derive(Debug)]
pub struct CheckMomentSpan {
    pub interval: Interval,
    pub context: ParsingContext,
}

impl Check<Dimension> for CheckMomentSpan {
    fn check(&self, pn: &ParsedNode<Dimension>) -> bool {
        self.context.resolve(&pn.value)
            .and_then(|v| TimeOutput::attempt_from(v))
            .map(|v| v.0.start == self.interval.start && v.0.end == self.interval.end)
            .unwrap_or(false)
    }
}

pub fn check_moment_span(context: ParsingContext, start: Moment, end: Moment, grain: Grain)
                      -> CheckMomentSpan {
    CheckMomentSpan { interval: Interval::new(start, Some(end), grain), context: context }
}
