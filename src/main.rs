mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;

fn main() -> anyhow::Result<()> {
    match 1 {
        01 => d01::solve(),
        02 => d02::solve(),
        03 => d03::solve(),
        04 => d04::solve(),
        05 => d05::solve(),
        06 => d06::solve(),
        07 => d07::solve(),
        08 => d08::solve(),
        09 => d09::solve(),
        10 => d10::solve(),
        11 => d11::solve(),
        12 => d12::solve(),
        13 => d13::solve(),
        14 => d14::solve(),
        15 => d15::solve(),
        16 => d16::solve(),
        17 => d17::solve(),
        18 => d18::solve(),
        19 => d19::solve(),
        20 => d20::solve(),
        21 => d21::solve(),
        22 => d22::solve(),
        23 => d23::solve(),
        24 => d24::solve(),
        _ => unreachable!(),
    }
}
