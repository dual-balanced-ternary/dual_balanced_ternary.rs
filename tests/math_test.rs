use dual_balanced_ternary::complex::ComplextXy;
use dual_balanced_ternary::{
  create_dual_balanced_ternary_from_pair, ternary, DualBalancedTernaryDigit::*,
};

#[test]
fn equality() {
  assert_eq!(ternary("&1.1"), ternary("&1.1"));
  assert_eq!(ternary("&1.1"), ternary("&1.15"));
  assert_eq!(ternary("&1.1"), ternary("&51.1"));
  assert_eq!(ternary("&1.5"), ternary("&1"));
  assert_ne!(ternary("&1.1"), ternary("&1.5"));
  assert_eq!(ternary("&."), ternary("&.5"));
  assert_eq!(ternary("&."), ternary("&5."));
  assert_eq!(ternary("&."), ternary("&5.5"));
}

#[test]
fn parse_and_format() {
  assert_eq!(format!("{}", ternary("&1.1")), "&1.1");
  assert_eq!(format!("{}", ternary("&1.15")), "&1.1");
  assert_eq!(format!("{}", ternary("&51.1")), "&1.1");
  assert_eq!(format!("{}", ternary("&51.15")), "&1.1");

  assert_eq!(format!("{}", ternary("&1.1").move_by(1)), "&11");
  assert_eq!(format!("{}", ternary("&1.1").move_by(-1)), "&.11");
}

#[test]
fn try_negate() {
  assert_eq!(ternary("&1.1").negate(), ternary("&9.9"));
  assert_eq!(ternary("&5").negate(), ternary("&5"));
  assert_eq!(ternary("&4").negate(), ternary("&6"));
  assert_eq!(ternary("&4.63").negate(), ternary("&6.47"));
}

#[test]
fn test_add_at() {
  assert_eq!(ternary("&1.1").add_at(0, Dbt1), ternary("&19.1"));
  assert_eq!(ternary("&1.1").add_at(-1, Dbt1), ternary("&19.9"));
  assert_eq!(ternary("&1.1").add_at(-1, Dbt9), ternary("&1."));
  assert_eq!(ternary("&1.1").add_at(-1, Dbt5), ternary("&1.1"));
  assert_eq!(ternary("&1.1").add_at(0, Dbt7), ternary("&6.1"));
  assert_eq!(ternary("&1.1").add_at(1, Dbt7), ternary("&71.1"));
  assert_eq!(ternary("&1.1").add_at(2, Dbt7), ternary("&751.1"));
  assert_eq!(ternary("&1.1").add_at(-2, Dbt7), ternary("&1.17"));
  assert_eq!(ternary("&1.1").add_at(-3, Dbt7), ternary("&1.157"));

  assert_eq!(ternary("&6.6").add_at(0, Dbt6), ternary("&64.6"));
  assert_eq!(ternary("&6.6").add_at(1, Dbt6), ternary("&66.6"));
  assert_eq!(ternary("&6.6").add_at(-1, Dbt6), ternary("&64.4"));
  assert_eq!(ternary("&6.6").add_at(-2, Dbt6), ternary("&6.66"));

  assert_eq!(ternary("&19.9").add_at(0, Dbt1), ternary("&15.9"));
}

#[test]
fn test_add() {
  assert_eq!(ternary("&1.1") + ternary("&9.9"), ternary("&5"));
  assert_eq!(ternary("&1.1") + ternary("&1.1"), ternary("&15.9"));
  assert_eq!(ternary("&1.6") + ternary("&1.6"), ternary("&17.4"));
}

#[test]
fn test_complex() {
  assert_eq!(ternary("&4").to_float(), ComplextXy { x: 1.0, y: -1.0 });
  assert_eq!(ternary("&5").to_float(), ComplextXy { x: 0.0, y: 0.0 });
  assert_eq!(ternary("&13").to_float(), ComplextXy { x: 1.0, y: 3.0 });
  assert_eq!(ternary("&66").to_float(), ComplextXy { x: -4.0, y: 4.0 });
  assert_eq!(
    ternary("&.1").to_float(),
    ComplextXy {
      x: 0.0,
      y: 1.0 / 3.0
    }
  );
  assert_eq!(
    ternary("&.4").to_float(),
    ComplextXy {
      x: 1.0 / 3.0,
      y: -1.0 / 3.0
    }
  );
  assert_eq!(
    ternary("&.7").to_float(),
    ComplextXy {
      x: -1.0 / 3.0,
      y: 0.0
    }
  );

  assert_eq!(
    create_dual_balanced_ternary_from_pair(4.0, 6.0),
    ternary("&143")
  );
  assert_eq!(
    create_dual_balanced_ternary_from_pair(4.0, 4.0),
    ternary("&88")
  );
  assert_eq!(
    create_dual_balanced_ternary_from_pair(-4.0, -4.0),
    ternary("&22")
  );
  assert_eq!(
    create_dual_balanced_ternary_from_pair(1.0, 7.0),
    ternary("&198")
  );

  println!("{}", create_dual_balanced_ternary_from_pair(1.1, 1.1));
  println!("{}", create_dual_balanced_ternary_from_pair(1.2, -1.2));
  println!("{}", create_dual_balanced_ternary_from_pair(-1.3, 1.3));
  println!("{}", create_dual_balanced_ternary_from_pair(-1.4, -1.4));
  assert_eq!(
    create_dual_balanced_ternary_from_pair(0.0, 0.0),
    ternary("&5")
  );
}

#[test]
fn test_sub() {
  assert_eq!(ternary("&15") - ternary("&6"), ternary("&14"));
  assert_eq!(ternary("&44") - ternary("&44"), ternary("&5"));
}

#[test]
fn test_multiply() {
  assert_eq!(ternary("&1") * ternary("&3"), ternary("&3"));
  assert_eq!(ternary("&3") * ternary("&3"), ternary("&9"));
  assert_eq!(ternary("&3") * ternary("&4"), ternary("&2"));

  assert_eq!(ternary("&35") * ternary("&4"), ternary("&25"));
  assert_eq!(ternary("&35") * ternary("&45"), ternary("&255"));

  assert_eq!(ternary("&.3") * ternary("&4"), ternary("&.2"));
  assert_eq!(ternary("&.3") * ternary("&.4"), ternary("&.52"));
  assert_eq!(ternary("&.3") * ternary("&45"), ternary("&2"));

  assert_eq!(ternary("&23") * ternary("&47"), ternary("&111"));
  assert_eq!(ternary("&616") * ternary("&751"), ternary("&743316"));
  assert_eq!(ternary("&616") * ternary("&751"), ternary("&743316"));
  assert_eq!(ternary("&751") * ternary("&616"), ternary("&743316"));

  assert_eq!(ternary("&3.3") * ternary("&1.3"), ternary("&3.49"));
  assert_eq!(ternary("&.5536") * ternary("&.5543"), ternary("&.55555928"));
}

#[test]
fn test_divide() {
  // echo "dividing...."
  assert_eq!((ternary("&15") / ternary("&1")), ternary("&15"));
  assert_eq!((ternary("&111") / ternary("&23")), ternary("&47"));
  assert_eq!((ternary("&743316") / ternary("&616")), ternary("&751"));
  assert_eq!((ternary("&743316") / ternary("&751")), ternary("&616"));

  assert_eq!(ternary("&3.49") / ternary("&3.3"), ternary("&1.3"));
  assert_eq!(ternary("&3.49") / ternary("&1.3"), ternary("&3.3"));

  assert_eq!(ternary("&.55555928") / ternary("&.5536"), ternary("&.5543"));

  // # not exact division
  println!("{}", ternary("&743317") / ternary("&616"));

  // there was a bug in mutiply conjugated values
  assert_eq!(
    ternary("&9.41658555559") / ternary("&9.51372555559"),
    ternary("&1.653732945268634852684471755515159")
  );
}

#[test]
fn test_round() {
  assert_eq!(ternary("&2.4").round(), ternary("&2"));
  assert_eq!(ternary("&2.444").round_n(0), ternary("&2"));
  assert_eq!(ternary("&2.444").round_n(1), ternary("&2.4"));
  assert_eq!(ternary("&2.444").round_n(2), ternary("&2.44"));
  assert_eq!(ternary("&2.444").round_n(3), ternary("&2.444"));
  assert_eq!(ternary("&2.444").round_n(4), ternary("&2.444"));
}

#[test]
fn test_hashes() {
  // not able to test hash

  // assert_ne!("&1".ternary, "&15".ternary);
  // assert_ne!("&1".ternary, "&.1".ternary);
  // assert_eq!("&1".ternary, "&1.5".ternary);
  // assert_eq!("&1".ternary, "&51".ternary);

  // println!("{}", (ternary("&.1")));
  // println!("{}", (ternary("&1")));
  // println!("{}", (ternary("&19")));
  // println!("{}", (ternary("&15")));
  // println!("{}", (ternary("&11")));
}
