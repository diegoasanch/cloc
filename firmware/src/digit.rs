/// Representation of a digit on a seven segment display.
///
/// decimal: The decimal value of the digit.
/// dot: Whether the dot is on or off.
/// on: Whether the digit is on or off.
pub struct Digit {
    decimal: u8,
    on: bool,
    dot: bool,
}

impl Digit {
    pub fn new(decimal: u8, dot: bool, on: bool) -> Self {
        Self { decimal, dot, on }
    }

    /// Converts the digit to a binary representation of the segments.
    /// If the digit is off, returns 0b00000000.
    /// Otherwise, returns the binary representation of the segments.
    pub fn to_binary(&self) -> u8 {
        if !self.on {
            return self.segments_to_binary(NULL);
        }

        let segments = self.digit_to_segments();
        self.segments_to_binary(segments)
    }

    fn digit_to_segments(&self) -> &[Segment] {
        match self.decimal {
            0 => ZERO,
            1 => ONE,
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
            _ => NULL,
        }
    }

    fn segments_to_binary(&self, segments: &[Segment]) -> u8 {
        let mut binary = 0b00000000;
        for segment in segments {
            binary |= 1 << segment.to_index();
        }

        if self.dot {
            binary |= 1 << DP.to_index();
        }
        binary
    }
}

enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    DP,
}

impl Segment {
    /// Maps the segment to the index of the bit in the binary representation.
    /// This is dependent on the wiring between the microcontroller and the seven segment display.
    pub fn to_index(&self) -> usize {
        match self {
            Segment::A => 4,
            Segment::B => 3,
            Segment::C => 2,
            Segment::D => 1,
            Segment::E => 0,
            Segment::F => 6,
            Segment::G => 7,
            Segment::DP => 5,
        }
    }
}

const DP: Segment = Segment::DP;
const NULL: &[Segment] = &[];
const ZERO: &[Segment] = &[
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
];
const ONE: &[Segment] = &[Segment::B, Segment::C];
const TWO: &[Segment] = &[Segment::A, Segment::B, Segment::D, Segment::E, Segment::G];
const THREE: &[Segment] = &[Segment::A, Segment::B, Segment::C, Segment::D, Segment::G];
const FOUR: &[Segment] = &[Segment::B, Segment::C, Segment::F, Segment::G];
const FIVE: &[Segment] = &[Segment::A, Segment::C, Segment::D, Segment::F, Segment::G];
const SIX: &[Segment] = &[
    Segment::A,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];
const SEVEN: &[Segment] = &[Segment::A, Segment::B, Segment::C];
const EIGHT: &[Segment] = &[
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];
const NINE: &[Segment] = &[
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::F,
    Segment::G,
];
