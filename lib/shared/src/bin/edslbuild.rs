#![allow(unused_imports)]

extern crate alloc;

use alloc::rc::Rc;
use ordered_float::OrderedFloat;

use shared::edsl::wad::node::*;

fn build_edsl_nodey() {
    let input = input();
    let pt = pass_thru(&input);
    let added = add(&input, &pt);
    let sf = sum_filter(&added, -1, 1);
    let sf2 = sum_filter(&added, -3, 3);
    let sfadd = add(&sf, &sf2);
    let out = sfadd;
    compile(&out, "../../edslout/src/edsl_nodey.rs", "EdslNodey");
}

fn build_edsl_high_pass() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::HighPass(input.clone()));
    compile(&out, "../../edslout/src/edsl_high_pass.rs", "EdslHighPass");
}

fn build_edsl_low_pass() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::LowPass(input.clone()));
    compile(&out, "../../edslout/src/edsl_low_pass.rs", "EdslLowPass");
}

fn build_edsl_pass_thru() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::PassThru(input.clone()));
    compile(&out, "../../edslout/src/edsl_pass_thru.rs", "EdslPassThru");
}

fn build_edsl_low_pass_6() {
    let input = Rc::new(Node::Input);
    let mut n = input;
    for _ in 0..6 {
        n = Rc::new(Node::LowPass(n));
    }
    compile(&n, "../../edslout/src/edsl_low_pass_6.rs", "EdslLowPass6");
}

fn build_edsl_const() {
    let out = Rc::new(Node::Const(OrderedFloat(13f32)));
    compile(&out, "../../edslout/src/edsl_const.rs", "EdslConst");
}

fn build_edsl_linear_vibrato() {
    let input = Rc::new(Node::Input);
    let vibrato_frequency = Rc::new(Node::Const(OrderedFloat(1.0)));
    let out = Rc::new(Node::LinearVibrato(10, vibrato_frequency.clone(), input.clone()));
    compile(&out, "../../edslout/src/edsl_linear_vibrato.rs", "EdslLinearVibrato");
}

fn edsl_linear_vibrato2(max_sample_deviation: usize, vibrato_frequency: f32, inn: &Rc<Node>) -> Rc<Node> {
    let sn = sine(&konst(vibrato_frequency), &konst(max_sample_deviation as f32), &konst(0.0));
    let vs = varispeed(max_sample_deviation, &sn, inn);
    vs
}

fn build_edsl_chorus() {
    let input = Rc::new(Node::Input);

    let n: f32 = 3.0;
    let d: f32 = 0.3;

    let lv0 = Rc::new(Node::LinearVibrato(20, Rc::new(Node::Const(OrderedFloat(n-d))), input.clone()));
    let lv1 = Rc::new(Node::LinearVibrato(22, Rc::new(Node::Const(OrderedFloat(n))), input.clone()));
    let lv2 = Rc::new(Node::LinearVibrato(18, Rc::new(Node::Const(OrderedFloat(n+d))), input.clone()));

    let sum = add(&add(&lv0, &lv1), &lv2);
    let out = div(&sum, &Rc::new(Node::Const(OrderedFloat(3.0))));

    compile(&out, "../../edslout/src/edsl_chorus.rs", "EdslChorus");
}

fn build_edsl_chorus2() {
    let input = Rc::new(Node::Input);

    let n: f32 = 3.0;
    let d: f32 = 0.3;

    let lv0 = edsl_linear_vibrato2(20, n-d, &input.clone());
    let lv1 = edsl_linear_vibrato2(22, n, &input.clone());
    let lv2 = edsl_linear_vibrato2(18, n+d, &input.clone());

    let sum = add(&add(&lv0, &lv1), &lv2);
    let out = div(&sum, &Rc::new(Node::Const(OrderedFloat(3.0))));

    compile(&out, "../../edslout/src/edsl_chorus2.rs", "EdslChorus2");
}

fn build_mod_lv() {
    let max_sample_deviation = 22;

    let input = Rc::new(Node::Input);
    let msn = sine(&konst(892.7), &konst(1.0), &konst(0.0));
    let sn = sine(&msn, &konst(max_sample_deviation as f32), &konst(0.0));
    let vs = varispeed(max_sample_deviation, &sn, &input);
    let lp = Rc::new(Node::LowPass(vs.clone()));
    let mix = add(&input, &lp);
    let half = div(&mix, &Rc::new(Node::Const(OrderedFloat(2.0))));

    let out = vs;

    compile(&out, "../../edslout/src/edsl_mod_lv.rs", "EdslModLV");
}

fn build_easy_tweak() {
    let sn = sine(&konst(vibrato_frequency), &konst(max_sample_deviation as f32), &konst(0.0));
    let lv = Rc::new(Node::LinearVibrato(18, Rc::new(sn), input.clone()));
}

fn main() {
    build_edsl_nodey();
    build_edsl_high_pass();
    build_edsl_low_pass();
    build_edsl_pass_thru();
    build_edsl_low_pass_6();
    build_edsl_const();
    build_edsl_linear_vibrato();
    build_edsl_chorus();
    build_edsl_chorus2();
    build_mod_lv();
    println!("hi edsl");
}
