use vstd::prelude::*;

verus! {

pub struct Zipper {
    pub left: Seq<nat>,
    pub focus: nat,
    pub right: Seq<nat>,
}

pub open spec fn zipper_to_seq(z: Zipper) -> Seq<nat> {
    z.left.add(seq![z.focus]).add(z.right)
}

pub open spec fn move_left(z: Zipper) -> Option<Zipper> {
    if z.left.len() == 0 {
        None
    } else {
        Some(Zipper {
            left: z.left.drop_last(),
            focus: z.left.last(),
            right: seq![z.focus].add(z.right),
        })
    }
}

pub open spec fn move_right(z: Zipper) -> Option<Zipper> {
    if z.right.len() == 0 {
        None
    } else {
        Some(Zipper {
            left: z.left.push(z.focus),
            focus: z.right.first(),
            right: z.right.drop_first(),
        })
    }
}

pub proof fn move_preserves_content(z: Zipper)
    ensures (match move_left(z) { Some(zl) => zipper_to_seq(zl) =~= zipper_to_seq(z), None => true }) && (match move_right(z) { Some(zr) => zipper_to_seq(zr) =~= zipper_to_seq(z), None => true })
{
    if z.left.len() > 0 {
        let zl = Zipper {
            left: z.left.drop_last(),
            focus: z.left.last(),
            right: seq![z.focus].add(z.right),
        };
        assert(zl.left.add(seq![zl.focus]) =~= z.left);
    }
    if z.right.len() > 0 {
        let zr = Zipper {
            left: z.left.push(z.focus),
            focus: z.right.first(),
            right: z.right.drop_first(),
        };
        assert(seq![zr.focus].add(zr.right) =~= z.right);
    }
}

} // verus!
