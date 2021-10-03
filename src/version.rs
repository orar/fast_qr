use crate::encode::Mode;
use crate::vecl::ECL;

pub enum Version {
    V1 = 1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
    V32,
    V33,
    V34,
    V35,
    V36,
    V37,
    V38,
    V39,
    V40,
}

impl Version {
    pub const fn get(mode: Mode, ecl: ECL, len: usize) -> Option<Self> {
        use Version::*;

        match mode {
            Mode::Numeric => match ecl {
                ECL::L => match len {
                    0..=41 => Some(V1),
                    0..=77 => Some(V2),
                    0..=127 => Some(V3),
                    0..=187 => Some(V4),
                    0..=255 => Some(V5),
                    0..=322 => Some(V6),
                    0..=370 => Some(V7),
                    0..=461 => Some(V8),
                    0..=552 => Some(V9),
                    0..=652 => Some(V10),
                    0..=772 => Some(V11),
                    0..=883 => Some(V12),
                    0..=1022 => Some(V13),
                    0..=1101 => Some(V14),
                    0..=1250 => Some(V15),
                    0..=1408 => Some(V16),
                    0..=1548 => Some(V17),
                    0..=1725 => Some(V18),
                    0..=1903 => Some(V19),
                    0..=2061 => Some(V20),
                    0..=2232 => Some(V21),
                    0..=2409 => Some(V22),
                    0..=2620 => Some(V23),
                    0..=2812 => Some(V24),
                    0..=3057 => Some(V25),
                    0..=3283 => Some(V26),
                    0..=3517 => Some(V27),
                    0..=3669 => Some(V28),
                    0..=3909 => Some(V29),
                    0..=4158 => Some(V30),
                    0..=4417 => Some(V31),
                    0..=4686 => Some(V32),
                    0..=4965 => Some(V33),
                    0..=5253 => Some(V34),
                    0..=5529 => Some(V35),
                    0..=5836 => Some(V36),
                    0..=6153 => Some(V37),
                    0..=6479 => Some(V38),
                    0..=6743 => Some(V39),
                    0..=7089 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=34 => Some(V1),
                    0..=63 => Some(V2),
                    0..=101 => Some(V3),
                    0..=149 => Some(V4),
                    0..=202 => Some(V5),
                    0..=255 => Some(V6),
                    0..=293 => Some(V7),
                    0..=365 => Some(V8),
                    0..=432 => Some(V9),
                    0..=513 => Some(V10),
                    0..=604 => Some(V11),
                    0..=691 => Some(V12),
                    0..=796 => Some(V13),
                    0..=871 => Some(V14),
                    0..=991 => Some(V15),
                    0..=1082 => Some(V16),
                    0..=1212 => Some(V17),
                    0..=1346 => Some(V18),
                    0..=1500 => Some(V19),
                    0..=1600 => Some(V20),
                    0..=1708 => Some(V21),
                    0..=1872 => Some(V22),
                    0..=2059 => Some(V23),
                    0..=2188 => Some(V24),
                    0..=2395 => Some(V25),
                    0..=2544 => Some(V26),
                    0..=2701 => Some(V27),
                    0..=2857 => Some(V28),
                    0..=3035 => Some(V29),
                    0..=3289 => Some(V30),
                    0..=3486 => Some(V31),
                    0..=3693 => Some(V32),
                    0..=3909 => Some(V33),
                    0..=4134 => Some(V34),
                    0..=4343 => Some(V35),
                    0..=4588 => Some(V36),
                    0..=4775 => Some(V37),
                    0..=5039 => Some(V38),
                    0..=5313 => Some(V39),
                    0..=5596 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=27 => Some(V1),
                    0..=48 => Some(V2),
                    0..=77 => Some(V3),
                    0..=111 => Some(V4),
                    0..=144 => Some(V5),
                    0..=178 => Some(V6),
                    0..=207 => Some(V7),
                    0..=259 => Some(V8),
                    0..=312 => Some(V9),
                    0..=364 => Some(V10),
                    0..=427 => Some(V11),
                    0..=489 => Some(V12),
                    0..=580 => Some(V13),
                    0..=621 => Some(V14),
                    0..=703 => Some(V15),
                    0..=775 => Some(V16),
                    0..=876 => Some(V17),
                    0..=948 => Some(V18),
                    0..=1063 => Some(V19),
                    0..=1159 => Some(V20),
                    0..=1224 => Some(V21),
                    0..=1358 => Some(V22),
                    0..=1468 => Some(V23),
                    0..=1588 => Some(V24),
                    0..=1718 => Some(V25),
                    0..=1804 => Some(V26),
                    0..=1933 => Some(V27),
                    0..=2085 => Some(V28),
                    0..=2181 => Some(V29),
                    0..=2358 => Some(V30),
                    0..=2473 => Some(V31),
                    0..=2670 => Some(V32),
                    0..=2805 => Some(V33),
                    0..=2949 => Some(V34),
                    0..=3081 => Some(V35),
                    0..=3244 => Some(V36),
                    0..=3417 => Some(V37),
                    0..=3599 => Some(V38),
                    0..=3791 => Some(V39),
                    0..=3993 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=17 => Some(V1),
                    0..=34 => Some(V2),
                    0..=58 => Some(V3),
                    0..=82 => Some(V4),
                    0..=106 => Some(V5),
                    0..=139 => Some(V6),
                    0..=154 => Some(V7),
                    0..=202 => Some(V8),
                    0..=235 => Some(V9),
                    0..=288 => Some(V10),
                    0..=331 => Some(V11),
                    0..=374 => Some(V12),
                    0..=427 => Some(V13),
                    0..=468 => Some(V14),
                    0..=530 => Some(V15),
                    0..=602 => Some(V16),
                    0..=674 => Some(V17),
                    0..=746 => Some(V18),
                    0..=813 => Some(V19),
                    0..=919 => Some(V20),
                    0..=969 => Some(V21),
                    0..=1056 => Some(V22),
                    0..=1108 => Some(V23),
                    0..=1228 => Some(V24),
                    0..=1286 => Some(V25),
                    0..=1425 => Some(V26),
                    0..=1501 => Some(V27),
                    0..=1581 => Some(V28),
                    0..=1677 => Some(V29),
                    0..=1782 => Some(V30),
                    0..=1897 => Some(V31),
                    0..=2022 => Some(V32),
                    0..=2157 => Some(V33),
                    0..=2301 => Some(V34),
                    0..=2361 => Some(V35),
                    0..=2524 => Some(V36),
                    0..=2625 => Some(V37),
                    0..=2735 => Some(V38),
                    0..=2927 => Some(V39),
                    0..=3057 => Some(V40),
                    _ => None,
                },
            },
            Mode::Alphanumeric => match ecl {
                ECL::L => match len {
                    0..=25 => Some(V1),
                    0..=47 => Some(V2),
                    0..=77 => Some(V3),
                    0..=114 => Some(V4),
                    0..=154 => Some(V5),
                    0..=195 => Some(V6),
                    0..=224 => Some(V7),
                    0..=279 => Some(V8),
                    0..=335 => Some(V9),
                    0..=395 => Some(V10),
                    0..=468 => Some(V11),
                    0..=535 => Some(V12),
                    0..=619 => Some(V13),
                    0..=667 => Some(V14),
                    0..=758 => Some(V15),
                    0..=854 => Some(V16),
                    0..=938 => Some(V17),
                    0..=1046 => Some(V18),
                    0..=1153 => Some(V19),
                    0..=1249 => Some(V20),
                    0..=1352 => Some(V21),
                    0..=1460 => Some(V22),
                    0..=1588 => Some(V23),
                    0..=1704 => Some(V24),
                    0..=1853 => Some(V25),
                    0..=1990 => Some(V26),
                    0..=2132 => Some(V27),
                    0..=2223 => Some(V28),
                    0..=2369 => Some(V29),
                    0..=2520 => Some(V30),
                    0..=2677 => Some(V31),
                    0..=2840 => Some(V32),
                    0..=3009 => Some(V33),
                    0..=3183 => Some(V34),
                    0..=3351 => Some(V35),
                    0..=3537 => Some(V36),
                    0..=3729 => Some(V37),
                    0..=3927 => Some(V38),
                    0..=4087 => Some(V39),
                    0..=4296 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=20 => Some(V1),
                    0..=38 => Some(V2),
                    0..=61 => Some(V3),
                    0..=90 => Some(V4),
                    0..=122 => Some(V5),
                    0..=154 => Some(V6),
                    0..=178 => Some(V7),
                    0..=221 => Some(V8),
                    0..=262 => Some(V9),
                    0..=311 => Some(V10),
                    0..=366 => Some(V11),
                    0..=419 => Some(V12),
                    0..=483 => Some(V13),
                    0..=528 => Some(V14),
                    0..=600 => Some(V15),
                    0..=656 => Some(V16),
                    0..=734 => Some(V17),
                    0..=816 => Some(V18),
                    0..=909 => Some(V19),
                    0..=970 => Some(V20),
                    0..=1035 => Some(V21),
                    0..=1134 => Some(V22),
                    0..=1248 => Some(V23),
                    0..=1326 => Some(V24),
                    0..=1451 => Some(V25),
                    0..=1542 => Some(V26),
                    0..=1637 => Some(V27),
                    0..=1732 => Some(V28),
                    0..=1839 => Some(V29),
                    0..=1994 => Some(V30),
                    0..=2113 => Some(V31),
                    0..=2238 => Some(V32),
                    0..=2369 => Some(V33),
                    0..=2506 => Some(V34),
                    0..=2632 => Some(V35),
                    0..=2780 => Some(V36),
                    0..=2894 => Some(V37),
                    0..=3054 => Some(V38),
                    0..=3220 => Some(V39),
                    0..=3391 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=16 => Some(V1),
                    0..=29 => Some(V2),
                    0..=47 => Some(V3),
                    0..=67 => Some(V4),
                    0..=87 => Some(V5),
                    0..=108 => Some(V6),
                    0..=125 => Some(V7),
                    0..=157 => Some(V8),
                    0..=189 => Some(V9),
                    0..=221 => Some(V10),
                    0..=259 => Some(V11),
                    0..=296 => Some(V12),
                    0..=352 => Some(V13),
                    0..=376 => Some(V14),
                    0..=426 => Some(V15),
                    0..=470 => Some(V16),
                    0..=531 => Some(V17),
                    0..=574 => Some(V18),
                    0..=644 => Some(V19),
                    0..=702 => Some(V20),
                    0..=742 => Some(V21),
                    0..=823 => Some(V22),
                    0..=890 => Some(V23),
                    0..=963 => Some(V24),
                    0..=1041 => Some(V25),
                    0..=1094 => Some(V26),
                    0..=1172 => Some(V27),
                    0..=1263 => Some(V28),
                    0..=1322 => Some(V29),
                    0..=1429 => Some(V30),
                    0..=1499 => Some(V31),
                    0..=1618 => Some(V32),
                    0..=1700 => Some(V33),
                    0..=1787 => Some(V34),
                    0..=1867 => Some(V35),
                    0..=1966 => Some(V36),
                    0..=2071 => Some(V37),
                    0..=2181 => Some(V38),
                    0..=2298 => Some(V39),
                    0..=2420 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=10 => Some(V1),
                    0..=20 => Some(V2),
                    0..=35 => Some(V3),
                    0..=50 => Some(V4),
                    0..=64 => Some(V5),
                    0..=84 => Some(V6),
                    0..=93 => Some(V7),
                    0..=122 => Some(V8),
                    0..=143 => Some(V9),
                    0..=174 => Some(V10),
                    0..=200 => Some(V11),
                    0..=227 => Some(V12),
                    0..=259 => Some(V13),
                    0..=283 => Some(V14),
                    0..=321 => Some(V15),
                    0..=365 => Some(V16),
                    0..=408 => Some(V17),
                    0..=452 => Some(V18),
                    0..=493 => Some(V19),
                    0..=557 => Some(V20),
                    0..=587 => Some(V21),
                    0..=640 => Some(V22),
                    0..=672 => Some(V23),
                    0..=744 => Some(V24),
                    0..=779 => Some(V25),
                    0..=864 => Some(V26),
                    0..=910 => Some(V27),
                    0..=958 => Some(V28),
                    0..=1016 => Some(V29),
                    0..=1080 => Some(V30),
                    0..=1150 => Some(V31),
                    0..=1226 => Some(V32),
                    0..=1307 => Some(V33),
                    0..=1394 => Some(V34),
                    0..=1431 => Some(V35),
                    0..=1530 => Some(V36),
                    0..=1591 => Some(V37),
                    0..=1658 => Some(V38),
                    0..=1774 => Some(V39),
                    0..=1852 => Some(V40),
                    _ => None,
                },
            },
            Mode::Byte => match ecl {
                ECL::L => match len {
                    0..=17 => Some(V1),
                    0..=32 => Some(V2),
                    0..=53 => Some(V3),
                    0..=78 => Some(V4),
                    0..=106 => Some(V5),
                    0..=134 => Some(V6),
                    0..=154 => Some(V7),
                    0..=192 => Some(V8),
                    0..=230 => Some(V9),
                    0..=271 => Some(V10),
                    0..=321 => Some(V11),
                    0..=367 => Some(V12),
                    0..=425 => Some(V13),
                    0..=458 => Some(V14),
                    0..=520 => Some(V15),
                    0..=586 => Some(V16),
                    0..=644 => Some(V17),
                    0..=718 => Some(V18),
                    0..=792 => Some(V19),
                    0..=858 => Some(V20),
                    0..=929 => Some(V21),
                    0..=1003 => Some(V22),
                    0..=1091 => Some(V23),
                    0..=1171 => Some(V24),
                    0..=1273 => Some(V25),
                    0..=1367 => Some(V26),
                    0..=1465 => Some(V27),
                    0..=1528 => Some(V28),
                    0..=1628 => Some(V29),
                    0..=1732 => Some(V30),
                    0..=1840 => Some(V31),
                    0..=1952 => Some(V32),
                    0..=2068 => Some(V33),
                    0..=2188 => Some(V34),
                    0..=2303 => Some(V35),
                    0..=2431 => Some(V36),
                    0..=2563 => Some(V37),
                    0..=2699 => Some(V38),
                    0..=2809 => Some(V39),
                    0..=2953 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=14 => Some(V1),
                    0..=26 => Some(V2),
                    0..=42 => Some(V3),
                    0..=62 => Some(V4),
                    0..=84 => Some(V5),
                    0..=106 => Some(V6),
                    0..=122 => Some(V7),
                    0..=152 => Some(V8),
                    0..=180 => Some(V9),
                    0..=213 => Some(V10),
                    0..=251 => Some(V11),
                    0..=287 => Some(V12),
                    0..=331 => Some(V13),
                    0..=362 => Some(V14),
                    0..=412 => Some(V15),
                    0..=450 => Some(V16),
                    0..=504 => Some(V17),
                    0..=560 => Some(V18),
                    0..=624 => Some(V19),
                    0..=666 => Some(V20),
                    0..=711 => Some(V21),
                    0..=779 => Some(V22),
                    0..=857 => Some(V23),
                    0..=911 => Some(V24),
                    0..=997 => Some(V25),
                    0..=1059 => Some(V26),
                    0..=1125 => Some(V27),
                    0..=1190 => Some(V28),
                    0..=1264 => Some(V29),
                    0..=1370 => Some(V30),
                    0..=1452 => Some(V31),
                    0..=1538 => Some(V32),
                    0..=1628 => Some(V33),
                    0..=1722 => Some(V34),
                    0..=1809 => Some(V35),
                    0..=1911 => Some(V36),
                    0..=1989 => Some(V37),
                    0..=2099 => Some(V38),
                    0..=2213 => Some(V39),
                    0..=2331 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=11 => Some(V1),
                    0..=20 => Some(V2),
                    0..=32 => Some(V3),
                    0..=46 => Some(V4),
                    0..=60 => Some(V5),
                    0..=74 => Some(V6),
                    0..=86 => Some(V7),
                    0..=108 => Some(V8),
                    0..=130 => Some(V9),
                    0..=151 => Some(V10),
                    0..=177 => Some(V11),
                    0..=203 => Some(V12),
                    0..=241 => Some(V13),
                    0..=258 => Some(V14),
                    0..=292 => Some(V15),
                    0..=322 => Some(V16),
                    0..=364 => Some(V17),
                    0..=394 => Some(V18),
                    0..=442 => Some(V19),
                    0..=482 => Some(V20),
                    0..=509 => Some(V21),
                    0..=565 => Some(V22),
                    0..=611 => Some(V23),
                    0..=661 => Some(V24),
                    0..=715 => Some(V25),
                    0..=751 => Some(V26),
                    0..=805 => Some(V27),
                    0..=868 => Some(V28),
                    0..=908 => Some(V29),
                    0..=982 => Some(V30),
                    0..=1030 => Some(V31),
                    0..=1112 => Some(V32),
                    0..=1168 => Some(V33),
                    0..=1228 => Some(V34),
                    0..=1283 => Some(V35),
                    0..=1351 => Some(V36),
                    0..=1423 => Some(V37),
                    0..=1499 => Some(V38),
                    0..=1579 => Some(V39),
                    0..=1663 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=7 => Some(V1),
                    0..=14 => Some(V2),
                    0..=24 => Some(V3),
                    0..=34 => Some(V4),
                    0..=44 => Some(V5),
                    0..=58 => Some(V6),
                    0..=64 => Some(V7),
                    0..=84 => Some(V8),
                    0..=98 => Some(V9),
                    0..=119 => Some(V10),
                    0..=137 => Some(V11),
                    0..=155 => Some(V12),
                    0..=177 => Some(V13),
                    0..=194 => Some(V14),
                    0..=220 => Some(V15),
                    0..=250 => Some(V16),
                    0..=280 => Some(V17),
                    0..=310 => Some(V18),
                    0..=338 => Some(V19),
                    0..=382 => Some(V20),
                    0..=403 => Some(V21),
                    0..=439 => Some(V22),
                    0..=461 => Some(V23),
                    0..=511 => Some(V24),
                    0..=535 => Some(V25),
                    0..=593 => Some(V26),
                    0..=625 => Some(V27),
                    0..=658 => Some(V28),
                    0..=698 => Some(V29),
                    0..=742 => Some(V30),
                    0..=790 => Some(V31),
                    0..=842 => Some(V32),
                    0..=898 => Some(V33),
                    0..=958 => Some(V34),
                    0..=983 => Some(V35),
                    0..=1051 => Some(V36),
                    0..=1093 => Some(V37),
                    0..=1139 => Some(V38),
                    0..=1219 => Some(V39),
                    0..=1273 => Some(V40),
                    _ => None,
                },
            },
        }
    }

    pub const fn cci_bits(&self, mode: Mode) -> usize {
        use Version::*;

        match mode {
            Mode::Numeric => match self {
                V1 => 10,
                V2 => 10,
                V3 => 10,
                V4 => 10,
                V5 => 10,
                V6 => 10,
                V7 => 10,
                V8 => 10,
                V9 => 10,
                V10 => 12,
                V11 => 12,
                V12 => 12,
                V13 => 12,
                V14 => 12,
                V15 => 12,
                V16 => 12,
                V17 => 12,
                V18 => 12,
                V19 => 12,
                V20 => 12,
                V21 => 12,
                V22 => 12,
                V23 => 12,
                V24 => 12,
                V25 => 12,
                V26 => 12,
                V27 => 14,
                V28 => 14,
                V29 => 14,
                V30 => 14,
                V31 => 14,
                V32 => 14,
                V33 => 14,
                V34 => 14,
                V35 => 14,
                V36 => 14,
                V37 => 14,
                V38 => 14,
                V39 => 14,
                V40 => 14,
            },
            Mode::Alphanumeric => match self {
                V1 => 9,
                V2 => 9,
                V3 => 9,
                V4 => 9,
                V5 => 9,
                V6 => 9,
                V7 => 9,
                V8 => 9,
                V9 => 9,
                V10 => 11,
                V11 => 11,
                V12 => 11,
                V13 => 11,
                V14 => 11,
                V15 => 11,
                V16 => 11,
                V17 => 11,
                V18 => 11,
                V19 => 11,
                V20 => 11,
                V21 => 11,
                V22 => 11,
                V23 => 11,
                V24 => 11,
                V25 => 11,
                V26 => 11,
                V27 => 13,
                V28 => 13,
                V29 => 13,
                V30 => 13,
                V31 => 13,
                V32 => 13,
                V33 => 13,
                V34 => 13,
                V35 => 13,
                V36 => 13,
                V37 => 13,
                V38 => 13,
                V39 => 13,
                V40 => 13,
            },
            Mode::Byte => match self {
                V1 => 8,
                V2 => 8,
                V3 => 8,
                V4 => 8,
                V5 => 8,
                V6 => 8,
                V7 => 8,
                V8 => 8,
                V9 => 8,
                V10 => 16,
                V11 => 16,
                V12 => 16,
                V13 => 16,
                V14 => 16,
                V15 => 16,
                V16 => 16,
                V17 => 16,
                V18 => 16,
                V19 => 16,
                V20 => 16,
                V21 => 16,
                V22 => 16,
                V23 => 16,
                V24 => 16,
                V25 => 16,
                V26 => 16,
                V27 => 16,
                V28 => 16,
                V29 => 16,
                V30 => 16,
                V31 => 16,
                V32 => 16,
                V33 => 16,
                V34 => 16,
                V35 => 16,
                V36 => 16,
                V37 => 16,
                V38 => 16,
                V39 => 16,
                V40 => 16,
            },
        }
    }

    pub const fn data_codewords(&self, ecl: ECL) -> usize {
        use Version::*;

        match ecl {
            ECL::L => match self {
                V1 => 19,
                V2 => 34,
                V3 => 55,
                V4 => 80,
                V5 => 108,
                V6 => 136,
                V7 => 156,
                V8 => 194,
                V9 => 232,
                V10 => 274,
                V11 => 324,
                V12 => 370,
                V13 => 428,
                V14 => 461,
                V15 => 523,
                V16 => 589,
                V17 => 647,
                V18 => 721,
                V19 => 795,
                V20 => 861,
                V21 => 932,
                V22 => 1006,
                V23 => 1094,
                V24 => 1174,
                V25 => 1276,
                V26 => 1370,
                V27 => 1468,
                V28 => 1531,
                V29 => 1631,
                V30 => 1735,
                V31 => 1843,
                V32 => 1955,
                V33 => 2071,
                V34 => 2191,
                V35 => 2306,
                V36 => 2434,
                V37 => 2566,
                V38 => 2702,
                V39 => 2812,
                V40 => 2956,
            },
            ECL::M => match self {
                V1 => 16,
                V2 => 28,
                V3 => 44,
                V4 => 64,
                V5 => 86,
                V6 => 108,
                V7 => 124,
                V8 => 154,
                V9 => 182,
                V10 => 216,
                V11 => 254,
                V12 => 290,
                V13 => 334,
                V14 => 365,
                V15 => 415,
                V16 => 453,
                V17 => 507,
                V18 => 563,
                V19 => 627,
                V20 => 669,
                V21 => 714,
                V22 => 782,
                V23 => 860,
                V24 => 914,
                V25 => 1000,
                V26 => 1062,
                V27 => 1128,
                V28 => 1193,
                V29 => 1267,
                V30 => 1373,
                V31 => 1455,
                V32 => 1541,
                V33 => 1631,
                V34 => 1725,
                V35 => 1812,
                V36 => 1914,
                V37 => 1992,
                V38 => 2102,
                V39 => 2216,
                V40 => 2334,
            },
            ECL::Q => match self {
                V1 => 13,
                V2 => 22,
                V3 => 34,
                V4 => 48,
                V5 => 62,
                V6 => 76,
                V7 => 88,
                V8 => 110,
                V9 => 132,
                V10 => 154,
                V11 => 180,
                V12 => 206,
                V13 => 244,
                V14 => 261,
                V15 => 295,
                V16 => 325,
                V17 => 367,
                V18 => 397,
                V19 => 445,
                V20 => 485,
                V21 => 512,
                V22 => 568,
                V23 => 614,
                V24 => 664,
                V25 => 718,
                V26 => 754,
                V27 => 808,
                V28 => 871,
                V29 => 911,
                V30 => 985,
                V31 => 1033,
                V32 => 1115,
                V33 => 1171,
                V34 => 1231,
                V35 => 1286,
                V36 => 1354,
                V37 => 1426,
                V38 => 1502,
                V39 => 1582,
                V40 => 1666,
            },
            ECL::H => match self {
                V1 => 9,
                V2 => 16,
                V3 => 26,
                V4 => 36,
                V5 => 46,
                V6 => 60,
                V7 => 66,
                V8 => 86,
                V9 => 100,
                V10 => 122,
                V11 => 140,
                V12 => 158,
                V13 => 180,
                V14 => 197,
                V15 => 223,
                V16 => 253,
                V17 => 283,
                V18 => 313,
                V19 => 341,
                V20 => 385,
                V21 => 406,
                V22 => 442,
                V23 => 464,
                V24 => 514,
                V25 => 538,
                V26 => 596,
                V27 => 628,
                V28 => 661,
                V29 => 701,
                V30 => 745,
                V31 => 793,
                V32 => 845,
                V33 => 901,
                V34 => 961,
                V35 => 986,
                V36 => 1054,
                V37 => 1096,
                V38 => 1142,
                V39 => 1222,
                V40 => 1276,
            },
        }
    }

    pub const fn data_bits(&self, ecl: ECL) -> usize {
        self.data_codewords(ecl) * 8
    }
}