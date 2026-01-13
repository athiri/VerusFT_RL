use vstd::prelude::*;

verus! {

pub open spec fn oneof_outputs<T>(gens: Seq<Set<T>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int| 0 <= i < gens.len() && gens[i].contains(x)
    )
}

pub open spec fn gen_suit_outputs() -> Set<Suit> {
    set![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
}


pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}


pub proof fn gen_suit_is_oneof()
    ensures gen_suit_outputs() =~= oneof_outputs(seq![
        set![Suit::Hearts],
        set![Suit::Diamonds],
        set![Suit::Clubs],
        set![Suit::Spades],
    ])
{
    let gens = seq![
        set![Suit::Hearts],
        set![Suit::Diamonds],
        set![Suit::Clubs],
        set![Suit::Spades],
    ];

    assert forall|s: Suit| gen_suit_outputs().contains(s) <==>
        oneof_outputs(gens).contains(s) by {
        match s {
            Suit::Hearts => {
                assert(gens[0].contains(s));
            }
            Suit::Diamonds => {
                assert(gens[1].contains(s));
            }
            Suit::Clubs => {
                assert(gens[2].contains(s));
            }
            Suit::Spades => {
                assert(gens[3].contains(s));
            }
        }
    }
}

} // verus!