////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create `for_2d!` macro here.

macro_rules! for_2d {
    ($ident1:ident <$type1:ty> in $exp1:expr, $ident2:ident <$type2:ty> in $exp2:expr, $block1:block ) => {
       for $ident1 in $exp1{
        let $ident1: $type1 = $ident1;
        for $ident2 in $exp2{
            let $ident2: $type2 = $ident2;
            $block1
        }
       } 
    };
    ($ident1:ident <$type1:ty> in $exp1:ident, $ident2:ident <$type2:ty> in $exp2:ident, $block1:block ) => {
       for $ident1 in $exp1{
        let x = $ident1;
        for $ident2 in $exp2{
            let y = $ident2;
            $block1
        }
       } 
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
