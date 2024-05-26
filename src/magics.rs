use crate::{
    bitboard::{Bitboard, EMPTY_BB},
    square::Square,
};

#[derive(Clone, Copy)]
pub struct MagicNumber {
    magic: u64,
    offset: usize,
}

pub fn generate_blocker_mask(square: Square, directions: [(i32, i32); 4]) -> Bitboard {
    let mut blockers = EMPTY_BB;

    let start_rank = (square.index() / 8) as i32;
    let start_file = (square.index() % 8) as i32;

    for (rank_offset, file_offset) in directions {
        let mut rank = start_rank;
        let mut file = start_file;

        loop {
            let next_square = Square::new(
                (rank as usize).try_into().unwrap(),
                (file as usize).try_into().unwrap(),
            )
            .bitboard();

            rank += rank_offset;
            file += file_offset;

            if (0..=7).contains(&rank) && (0..=7).contains(&file) {
                blockers |= next_square;
            } else {
                break;
            }
        }
    }

    blockers.clear_bit(square);
    blockers
}

pub const ROOK_MAGICS: [MagicNumber; 64] = [
    MagicNumber {
        magic: 0x0040004094200000,
        offset: 0,
    },
    MagicNumber {
        magic: 0x0080106001400001,
        offset: 4096,
    },
    MagicNumber {
        magic: 0x0004121008098000,
        offset: 6144,
    },
    MagicNumber {
        magic: 0x2010020090040001,
        offset: 8192,
    },
    MagicNumber {
        magic: 0x05000800A4030000,
        offset: 10240,
    },
    MagicNumber {
        magic: 0x0080122005040000,
        offset: 12288,
    },
    MagicNumber {
        magic: 0x0080054020820000,
        offset: 14336,
    },
    MagicNumber {
        magic: 0x4080004028811000,
        offset: 16384,
    },
    MagicNumber {
        magic: 0xD129101100400000,
        offset: 20480,
    },
    MagicNumber {
        magic: 0xA200604009200002,
        offset: 22528,
    },
    MagicNumber {
        magic: 0x5380080220010000,
        offset: 23552,
    },
    MagicNumber {
        magic: 0x0206040010020000,
        offset: 24576,
    },
    MagicNumber {
        magic: 0x0108200800040000,
        offset: 25600,
    },
    MagicNumber {
        magic: 0x00288048D21C0000,
        offset: 26624,
    },
    MagicNumber {
        magic: 0x2880202080F20000,
        offset: 27648,
    },
    MagicNumber {
        magic: 0x4101080181000200,
        offset: 28672,
    },
    MagicNumber {
        magic: 0x0580004801900000,
        offset: 30720,
    },
    MagicNumber {
        magic: 0x0580004801900000,
        offset: 32768,
    },
    MagicNumber {
        magic: 0x4080001002E00008,
        offset: 33792,
    },
    MagicNumber {
        magic: 0x2000052BD0020000,
        offset: 34816,
    },
    MagicNumber {
        magic: 0x00C0004804004000,
        offset: 35840,
    },
    MagicNumber {
        magic: 0x0400080024102022,
        offset: 36864,
    },
    MagicNumber {
        magic: 0x0041056002004400,
        offset: 37888,
    },
    MagicNumber {
        magic: 0x00430E2008004000,
        offset: 38912,
    },
    MagicNumber {
        magic: 0x0100004020200000,
        offset: 40960,
    },
    MagicNumber {
        magic: 0x2000052BD0020000,
        offset: 43008,
    },
    MagicNumber {
        magic: 0x0005005802080000,
        offset: 44032,
    },
    MagicNumber {
        magic: 0x0800002100100000,
        offset: 45056,
    },
    MagicNumber {
        magic: 0x080000C20A046000,
        offset: 46080,
    },
    MagicNumber {
        magic: 0x0501200380140000,
        offset: 47104,
    },
    MagicNumber {
        magic: 0x00000001C0020800,
        offset: 48128,
    },
    MagicNumber {
        magic: 0x00000001C0020800,
        offset: 49152,
    },
    MagicNumber {
        magic: 0x0100004020200000,
        offset: 51200,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 53248,
    },
    MagicNumber {
        magic: 0x4080001002E00008,
        offset: 54272,
    },
    MagicNumber {
        magic: 0x1500004012100001,
        offset: 55296,
    },
    MagicNumber {
        magic: 0x00C0004804004000,
        offset: 56320,
    },
    MagicNumber {
        magic: 0x4000010008820000,
        offset: 57344,
    },
    MagicNumber {
        magic: 0x4000010008820000,
        offset: 58368,
    },
    MagicNumber {
        magic: 0x0600080052100400,
        offset: 59392,
    },
    MagicNumber {
        magic: 0x80000C9614140010,
        offset: 61440,
    },
    MagicNumber {
        magic: 0x80000C9614140010,
        offset: 63488,
    },
    MagicNumber {
        magic: 0x2801092000410008,
        offset: 64512,
    },
    MagicNumber {
        magic: 0x00C0004804004000,
        offset: 65536,
    },
    MagicNumber {
        magic: 0x0000019040040000,
        offset: 66560,
    },
    MagicNumber {
        magic: 0x080000C20A046000,
        offset: 67584,
    },
    MagicNumber {
        magic: 0x080000C20A046000,
        offset: 68608,
    },
    MagicNumber {
        magic: 0x0009115009048000,
        offset: 69632,
    },
    MagicNumber {
        magic: 0xC000068446080050,
        offset: 71680,
    },
    MagicNumber {
        magic: 0x1080041441002210,
        offset: 73728,
    },
    MagicNumber {
        magic: 0x1000003001002108,
        offset: 74752,
    },
    MagicNumber {
        magic: 0x1002500180080280,
        offset: 75776,
    },
    MagicNumber {
        magic: 0x8010000861100400,
        offset: 76800,
    },
    MagicNumber {
        magic: 0x4408000218400500,
        offset: 77824,
    },
    MagicNumber {
        magic: 0x8010000861100400,
        offset: 78848,
    },
    MagicNumber {
        magic: 0x12C0000959000080,
        offset: 79872,
    },
    MagicNumber {
        magic: 0x2000004129008001,
        offset: 81920,
    },
    MagicNumber {
        magic: 0x2600011080220102,
        offset: 86016,
    },
    MagicNumber {
        magic: 0x8800100941A08202,
        offset: 88064,
    },
    MagicNumber {
        magic: 0x0240000400604084,
        offset: 90112,
    },
    MagicNumber {
        magic: 0x0000002090068244,
        offset: 92160,
    },
    MagicNumber {
        magic: 0x0020004A04088009,
        offset: 94208,
    },
    MagicNumber {
        magic: 0x80004486480B1004,
        offset: 96256,
    },
    MagicNumber {
        magic: 0x0002000102441082,
        offset: 98304,
    },
];

const ROOK_ATTACK_TABLE_SIZE: usize = 102400;

pub const BISHOP_MAGICS: [MagicNumber; 64] = [
    MagicNumber {
        magic: 0x8084085004042000,
        offset: 0,
    },
    MagicNumber {
        magic: 0x04A8108146200010,
        offset: 64,
    },
    MagicNumber {
        magic: 0x9001020600500008,
        offset: 96,
    },
    MagicNumber {
        magic: 0x0404484008000000,
        offset: 128,
    },
    MagicNumber {
        magic: 0x0008440000160040,
        offset: 160,
    },
    MagicNumber {
        magic: 0x0202020200900108,
        offset: 192,
    },
    MagicNumber {
        magic: 0x4183440208000800,
        offset: 224,
    },
    MagicNumber {
        magic: 0x240104004E280090,
        offset: 256,
    },
    MagicNumber {
        magic: 0x2012149022020008,
        offset: 320,
    },
    MagicNumber {
        magic: 0x2012149022020008,
        offset: 352,
    },
    MagicNumber {
        magic: 0x04A8108146200010,
        offset: 384,
    },
    MagicNumber {
        magic: 0x8000A40040082020,
        offset: 416,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 448,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 480,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 512,
    },
    MagicNumber {
        magic: 0x0180010048040046,
        offset: 544,
    },
    MagicNumber {
        magic: 0x8000080655080000,
        offset: 576,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 608,
    },
    MagicNumber {
        magic: 0x0000442800740008,
        offset: 640,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 768,
    },
    MagicNumber {
        magic: 0x0100000100C01014,
        offset: 896,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 1024,
    },
    MagicNumber {
        magic: 0x02002C5088180804,
        offset: 1152,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 1184,
    },
    MagicNumber {
        magic: 0x8000A40040082020,
        offset: 1216,
    },
    MagicNumber {
        magic: 0x0000900020AC0090,
        offset: 1248,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 1280,
    },
    MagicNumber {
        magic: 0x00C0004804004000,
        offset: 1408,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 1920,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 2432,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 2560,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 2592,
    },
    MagicNumber {
        magic: 0x8000080655080000,
        offset: 2624,
    },
    MagicNumber {
        magic: 0x8000080655080000,
        offset: 2656,
    },
    MagicNumber {
        magic: 0x0800002100100000,
        offset: 2688,
    },
    MagicNumber {
        magic: 0x400000044000C002,
        offset: 2816,
    },
    MagicNumber {
        magic: 0x00C0004804004000,
        offset: 3328,
    },
    MagicNumber {
        magic: 0x8000A40040082020,
        offset: 3840,
    },
    MagicNumber {
        magic: 0x5000310200232084,
        offset: 3968,
    },
    MagicNumber {
        magic: 0x5000310200232084,
        offset: 4000,
    },
    MagicNumber {
        magic: 0x0050021006000010,
        offset: 4032,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 4064,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 4096,
    },
    MagicNumber {
        magic: 0x0000020110808040,
        offset: 4224,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 4352,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 4480,
    },
    MagicNumber {
        magic: 0x04A8108146200010,
        offset: 4608,
    },
    MagicNumber {
        magic: 0x0420008C00500000,
        offset: 4640,
    },
    MagicNumber {
        magic: 0x02002C5088180804,
        offset: 4672,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 4704,
    },
    MagicNumber {
        magic: 0x240104004E280090,
        offset: 4736,
    },
    MagicNumber {
        magic: 0x0010100000540408,
        offset: 4768,
    },
    MagicNumber {
        magic: 0x1004040004070860,
        offset: 4800,
    },
    MagicNumber {
        magic: 0x00800A0801080080,
        offset: 4832,
    },
    MagicNumber {
        magic: 0x0280200401404080,
        offset: 4864,
    },
    MagicNumber {
        magic: 0x4C400C808244A018,
        offset: 4896,
    },
    MagicNumber {
        magic: 0x080000B202024800,
        offset: 4928,
    },
    MagicNumber {
        magic: 0x000C900104101408,
        offset: 4992,
    },
    MagicNumber {
        magic: 0x1000040000845018,
        offset: 5024,
    },
    MagicNumber {
        magic: 0x0016804085808209,
        offset: 5056,
    },
    MagicNumber {
        magic: 0x0010100000540408,
        offset: 5088,
    },
    MagicNumber {
        magic: 0x000C900104101408,
        offset: 5120,
    },
    MagicNumber {
        magic: 0x4080002006020040,
        offset: 5152,
    },
    MagicNumber {
        magic: 0x0000020082008100,
        offset: 5184,
    },
];

const BISHOP_ATTACK_TABLE_SIZE: usize = 5248;
