use crate::bitboard::Bitboard;

#[derive(Clone, Copy)]
pub struct MagicNumber {
    magic: u64,
    shift: u8,
    offset: usize,
}

impl MagicNumber {
    pub fn get_magic_index(&self, blockers: Bitboard) -> usize {
        let hash = self.magic.wrapping_mul(blockers.0);
        let index = (hash >> self.shift) as usize;
        index + self.offset
    }
}

pub const ROOK_MAGICS: [MagicNumber; 64] = [
    MagicNumber {
        magic: 0x0080004001802010,
        shift: 52,
        offset: 0,
    },
    MagicNumber {
        magic: 0x0040100840012000,
        shift: 53,
        offset: 4096,
    },
    MagicNumber {
        magic: 0x0080182000809000,
        shift: 53,
        offset: 6144,
    },
    MagicNumber {
        magic: 0x0080100005080080,
        shift: 53,
        offset: 8192,
    },
    MagicNumber {
        magic: 0x0200100420020008,
        shift: 53,
        offset: 10240,
    },
    MagicNumber {
        magic: 0x0100050008140002,
        shift: 53,
        offset: 12288,
    },
    MagicNumber {
        magic: 0x1880108006000100,
        shift: 53,
        offset: 14336,
    },
    MagicNumber {
        magic: 0x8200060421028444,
        shift: 52,
        offset: 16384,
    },
    MagicNumber {
        magic: 0x89048000C0028C61,
        shift: 53,
        offset: 20480,
    },
    MagicNumber {
        magic: 0x0452804000200080,
        shift: 54,
        offset: 22528,
    },
    MagicNumber {
        magic: 0x0489004020010011,
        shift: 54,
        offset: 23552,
    },
    MagicNumber {
        magic: 0x8402000820120044,
        shift: 54,
        offset: 24576,
    },
    MagicNumber {
        magic: 0x8402000820120044,
        shift: 54,
        offset: 25600,
    },
    MagicNumber {
        magic: 0x10220010080A0005,
        shift: 54,
        offset: 26624,
    },
    MagicNumber {
        magic: 0x0001002100040200,
        shift: 54,
        offset: 27648,
    },
    MagicNumber {
        magic: 0x0002000102441082,
        shift: 53,
        offset: 28672,
    },
    MagicNumber {
        magic: 0x0000848008400062,
        shift: 53,
        offset: 30720,
    },
    MagicNumber {
        magic: 0x0800404010002000,
        shift: 54,
        offset: 32768,
    },
    MagicNumber {
        magic: 0x00A0010020104108,
        shift: 54,
        offset: 33792,
    },
    MagicNumber {
        magic: 0x0008808018005000,
        shift: 54,
        offset: 34816,
    },
    MagicNumber {
        magic: 0x008400800C080080,
        shift: 54,
        offset: 35840,
    },
    MagicNumber {
        magic: 0x0204008022001480,
        shift: 54,
        offset: 36864,
    },
    MagicNumber {
        magic: 0x0C040400082D8210,
        shift: 54,
        offset: 37888,
    },
    MagicNumber {
        magic: 0x0059020000804405,
        shift: 53,
        offset: 38912,
    },
    MagicNumber {
        magic: 0x0080400080008060,
        shift: 53,
        offset: 40960,
    },
    MagicNumber {
        magic: 0x80300044400C2000,
        shift: 54,
        offset: 43008,
    },
    MagicNumber {
        magic: 0x802000A080100080,
        shift: 54,
        offset: 44032,
    },
    MagicNumber {
        magic: 0x1002500180080280,
        shift: 54,
        offset: 45056,
    },
    MagicNumber {
        magic: 0x02A0080080800C00,
        shift: 54,
        offset: 46080,
    },
    MagicNumber {
        magic: 0x4804420080800400,
        shift: 54,
        offset: 47104,
    },
    MagicNumber {
        magic: 0x4603410400080210,
        shift: 54,
        offset: 48128,
    },
    MagicNumber {
        magic: 0x0808004200010084,
        shift: 53,
        offset: 49152,
    },
    MagicNumber {
        magic: 0x0080082000400A40,
        shift: 53,
        offset: 51200,
    },
    MagicNumber {
        magic: 0x0000884002802000,
        shift: 54,
        offset: 53248,
    },
    MagicNumber {
        magic: 0x20A0002080801000,
        shift: 54,
        offset: 54272,
    },
    MagicNumber {
        magic: 0x0030801000802800,
        shift: 54,
        offset: 55296,
    },
    MagicNumber {
        magic: 0x02A0080080800C00,
        shift: 54,
        offset: 56320,
    },
    MagicNumber {
        magic: 0x0002800401800200,
        shift: 54,
        offset: 57344,
    },
    MagicNumber {
        magic: 0x088008810400500E,
        shift: 54,
        offset: 58368,
    },
    MagicNumber {
        magic: 0x40C000442A000091,
        shift: 53,
        offset: 59392,
    },
    MagicNumber {
        magic: 0x0884208040008000,
        shift: 53,
        offset: 61440,
    },
    MagicNumber {
        magic: 0x2002008310420020,
        shift: 54,
        offset: 63488,
    },
    MagicNumber {
        magic: 0x0000442001030015,
        shift: 54,
        offset: 64512,
    },
    MagicNumber {
        magic: 0x000A00401222000A,
        shift: 54,
        offset: 65536,
    },
    MagicNumber {
        magic: 0x8000040008008080,
        shift: 54,
        offset: 66560,
    },
    MagicNumber {
        magic: 0x10220010080A0005,
        shift: 54,
        offset: 67584,
    },
    MagicNumber {
        magic: 0x0080D00801040042,
        shift: 54,
        offset: 68608,
    },
    MagicNumber {
        magic: 0x40051084005A0001,
        shift: 53,
        offset: 69632,
    },
    MagicNumber {
        magic: 0x084000801821C580,
        shift: 53,
        offset: 71680,
    },
    MagicNumber {
        magic: 0x0452804000200080,
        shift: 54,
        offset: 73728,
    },
    MagicNumber {
        magic: 0x0000A00040150100,
        shift: 54,
        offset: 74752,
    },
    MagicNumber {
        magic: 0x1002500180080280,
        shift: 54,
        offset: 75776,
    },
    MagicNumber {
        magic: 0x0202808802040080,
        shift: 54,
        offset: 76800,
    },
    MagicNumber {
        magic: 0x8002000904301200,
        shift: 54,
        offset: 77824,
    },
    MagicNumber {
        magic: 0x0000020508107400,
        shift: 54,
        offset: 78848,
    },
    MagicNumber {
        magic: 0x1844009401015200,
        shift: 53,
        offset: 79872,
    },
    MagicNumber {
        magic: 0x1020245502408001,
        shift: 52,
        offset: 81920,
    },
    MagicNumber {
        magic: 0x0002418023001202,
        shift: 53,
        offset: 86016,
    },
    MagicNumber {
        magic: 0x8800100941A08202,
        shift: 53,
        offset: 88064,
    },
    MagicNumber {
        magic: 0x0002300009002005,
        shift: 53,
        offset: 90112,
    },
    MagicNumber {
        magic: 0x2011001008000403,
        shift: 53,
        offset: 92160,
    },
    MagicNumber {
        magic: 0x22C1000204000801,
        shift: 53,
        offset: 94208,
    },
    MagicNumber {
        magic: 0x80004486480B1004,
        shift: 53,
        offset: 96256,
    },
    MagicNumber {
        magic: 0x0020408224004102,
        shift: 52,
        offset: 98304,
    },
];

pub const ROOK_ATTACK_TABLE_SIZE: usize = 102400;

pub const BISHOP_MAGICS: [MagicNumber; 64] = [
    MagicNumber {
        magic: 0x0A18211014004080,
        shift: 58,
        offset: 0,
    },
    MagicNumber {
        magic: 0x0260488503082406,
        shift: 59,
        offset: 64,
    },
    MagicNumber {
        magic: 0x04100C004C404000,
        shift: 59,
        offset: 96,
    },
    MagicNumber {
        magic: 0x80840C0080000000,
        shift: 59,
        offset: 128,
    },
    MagicNumber {
        magic: 0x0004052002418089,
        shift: 59,
        offset: 160,
    },
    MagicNumber {
        magic: 0x18408884400008A0,
        shift: 59,
        offset: 192,
    },
    MagicNumber {
        magic: 0x1400460610404000,
        shift: 59,
        offset: 224,
    },
    MagicNumber {
        magic: 0x092104A484044000,
        shift: 58,
        offset: 256,
    },
    MagicNumber {
        magic: 0x4210103090014044,
        shift: 59,
        offset: 320,
    },
    MagicNumber {
        magic: 0x0608092202860201,
        shift: 59,
        offset: 352,
    },
    MagicNumber {
        magic: 0x0000212904009000,
        shift: 59,
        offset: 384,
    },
    MagicNumber {
        magic: 0x0200082080620948,
        shift: 59,
        offset: 416,
    },
    MagicNumber {
        magic: 0x1400460610404000,
        shift: 59,
        offset: 448,
    },
    MagicNumber {
        magic: 0x0010820910080002,
        shift: 59,
        offset: 480,
    },
    MagicNumber {
        magic: 0x4052008C14024143,
        shift: 59,
        offset: 512,
    },
    MagicNumber {
        magic: 0x0204004202012000,
        shift: 59,
        offset: 544,
    },
    MagicNumber {
        magic: 0x8048021420082200,
        shift: 59,
        offset: 576,
    },
    MagicNumber {
        magic: 0x02100408222800C0,
        shift: 59,
        offset: 608,
    },
    MagicNumber {
        magic: 0x0410005200881100,
        shift: 57,
        offset: 640,
    },
    MagicNumber {
        magic: 0x0006002020214200,
        shift: 57,
        offset: 768,
    },
    MagicNumber {
        magic: 0x0011000820080040,
        shift: 57,
        offset: 896,
    },
    MagicNumber {
        magic: 0x8002000109100210,
        shift: 57,
        offset: 1024,
    },
    MagicNumber {
        magic: 0x1002284082100200,
        shift: 59,
        offset: 1152,
    },
    MagicNumber {
        magic: 0x00020008410C4107,
        shift: 59,
        offset: 1184,
    },
    MagicNumber {
        magic: 0x1010880540883B12,
        shift: 59,
        offset: 1216,
    },
    MagicNumber {
        magic: 0x0408208408650502,
        shift: 59,
        offset: 1248,
    },
    MagicNumber {
        magic: 0x1900818010040080,
        shift: 57,
        offset: 1280,
    },
    MagicNumber {
        magic: 0x10A0080001004008,
        shift: 55,
        offset: 1408,
    },
    MagicNumber {
        magic: 0x8029840000802010,
        shift: 55,
        offset: 1920,
    },
    MagicNumber {
        magic: 0x0003010003300800,
        shift: 57,
        offset: 2432,
    },
    MagicNumber {
        magic: 0x8104006D84020600,
        shift: 59,
        offset: 2560,
    },
    MagicNumber {
        magic: 0x080208A008C40221,
        shift: 59,
        offset: 2592,
    },
    MagicNumber {
        magic: 0x4002200400608940,
        shift: 59,
        offset: 2624,
    },
    MagicNumber {
        magic: 0x0802122000100100,
        shift: 59,
        offset: 2656,
    },
    MagicNumber {
        magic: 0x5004004800040520,
        shift: 57,
        offset: 2688,
    },
    MagicNumber {
        magic: 0x4009020080080080,
        shift: 55,
        offset: 2816,
    },
    MagicNumber {
        magic: 0x800E108400020120,
        shift: 55,
        offset: 3328,
    },
    MagicNumber {
        magic: 0x6201900500108188,
        shift: 57,
        offset: 3840,
    },
    MagicNumber {
        magic: 0x0024285601038080,
        shift: 59,
        offset: 3968,
    },
    MagicNumber {
        magic: 0x0024285601038080,
        shift: 59,
        offset: 4000,
    },
    MagicNumber {
        magic: 0x0022025040001400,
        shift: 59,
        offset: 4032,
    },
    MagicNumber {
        magic: 0x400401440A101000,
        shift: 59,
        offset: 4064,
    },
    MagicNumber {
        magic: 0x0220202428081004,
        shift: 57,
        offset: 4096,
    },
    MagicNumber {
        magic: 0x000202C010408208,
        shift: 57,
        offset: 4224,
    },
    MagicNumber {
        magic: 0x0400202009010380,
        shift: 57,
        offset: 4352,
    },
    MagicNumber {
        magic: 0x2081025482000100,
        shift: 57,
        offset: 4480,
    },
    MagicNumber {
        magic: 0x00085000C0800600,
        shift: 59,
        offset: 4608,
    },
    MagicNumber {
        magic: 0x0010110200801020,
        shift: 59,
        offset: 4640,
    },
    MagicNumber {
        magic: 0x1400460610404000,
        shift: 59,
        offset: 4672,
    },
    MagicNumber {
        magic: 0x0005005802080000,
        shift: 59,
        offset: 4704,
    },
    MagicNumber {
        magic: 0xA020002308220010,
        shift: 59,
        offset: 4736,
    },
    MagicNumber {
        magic: 0x092104A484044000,
        shift: 59,
        offset: 4768,
    },
    MagicNumber {
        magic: 0x2012149022020008,
        shift: 59,
        offset: 4800,
    },
    MagicNumber {
        magic: 0x2012149022020008,
        shift: 59,
        offset: 4832,
    },
    MagicNumber {
        magic: 0x40A00250020088A0,
        shift: 59,
        offset: 4864,
    },
    MagicNumber {
        magic: 0x0260488503082406,
        shift: 59,
        offset: 4896,
    },
    MagicNumber {
        magic: 0x092104A484044000,
        shift: 58,
        offset: 4928,
    },
    MagicNumber {
        magic: 0x0204004202012000,
        shift: 59,
        offset: 4992,
    },
    MagicNumber {
        magic: 0x0080840142080448,
        shift: 59,
        offset: 5024,
    },
    MagicNumber {
        magic: 0x0020081100420209,
        shift: 59,
        offset: 5056,
    },
    MagicNumber {
        magic: 0x0001400110020209,
        shift: 59,
        offset: 5088,
    },
    MagicNumber {
        magic: 0x490400A4200C2102,
        shift: 59,
        offset: 5120,
    },
    MagicNumber {
        magic: 0x4210103090014044,
        shift: 59,
        offset: 5152,
    },
    MagicNumber {
        magic: 0x0A18211014004080,
        shift: 58,
        offset: 5184,
    },
];

pub const BISHOP_ATTACK_TABLE_SIZE: usize = 5248;
