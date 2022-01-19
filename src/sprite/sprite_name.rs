


















use crate::SpriteData;

#[derive(Clone, Copy, Debug)]
pub enum Sprite {
    #[allow(non_camel_case_types)]
    arrow,
    #[allow(non_camel_case_types)]
    bullet1,
    #[allow(non_camel_case_types)]
    bullet2,
    #[allow(non_camel_case_types)]
    bullet3,
    #[allow(non_camel_case_types)]
    bullet4,
    #[allow(non_camel_case_types)]
    enemy1,
    #[allow(non_camel_case_types)]
    enemy2,
    #[allow(non_camel_case_types)]
    enemy3,
    #[allow(non_camel_case_types)]
    enemy4,
    #[allow(non_camel_case_types)]
    heart,
    #[allow(non_camel_case_types)]
    ship1,
    #[allow(non_camel_case_types)]
    ship2,
    #[allow(non_camel_case_types)]
    ship3,
    #[allow(non_camel_case_types)]
    ship4,
    #[allow(non_camel_case_types)]
    ship5,
    #[allow(non_camel_case_types)]
    ship6,
    #[allow(non_camel_case_types)]
    ship7,
    #[allow(non_camel_case_types)]
    ship8,
    #[allow(non_camel_case_types)]
    ship9,
    #[allow(non_camel_case_types)]
    turret,
}

impl SpriteName {
    pub fn get(self) -> SpriteData {
        use SpriteName::*;
        match self {
            arrow => SpriteData {
                width: 8,
                height: 8,
                flags: 0,
                data: vec![ 0x60,0x70,0x78,0x7c,0x7c,0x78,0x70,0x60 ],
            },
            bullet1 => SpriteData {
                width: 4,
                height: 4,
                flags: 1,
                data: vec![ 0x18,0x64,0xa4,0x20 ],
            },
            bullet2 => SpriteData {
                width: 4,
                height: 4,
                flags: 1,
                data: vec![ 0x10,0x68,0x94,0x28 ],
            },
            bullet3 => SpriteData {
                width: 4,
                height: 4,
                flags: 1,
                data: vec![ 0x10,0x69,0x26,0x18 ],
            },
            bullet4 => SpriteData {
                width: 4,
                height: 4,
                flags: 1,
                data: vec![ 0x18,0x99,0x64,0x20 ],
            },
            enemy1 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x40,0x08,0x00,0x01,0x40,0x0a,0x00,0x05,0x40,0x0b,0x80,0x05,0x80,0x0a,0xc0,0x16,0x60,0x3b,0xf0,0x15,0xaa,0xae,0xf0,0x16,0xaa,0xab,0xf0,0x15,0xaa,0xaa,0xf0,0x16,0x6a,0xab,0xf0,0x15,0xa8,0xbb,0xf0,0x05,0x68,0xaf,0xc0,0x01,0x68,0xaf,0x00,0x00,0x50,0x2c,0x00,0x00,0x10,0x20,0x00,0x00,0x10,0x20,0x00,0x00,0x10,0x20,0x00 ],
            },
            enemy2 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x00,0x01,0x00,0x00,0x80,0x01,0x00,0x00,0x80,0x01,0x10,0x02,0x80,0x01,0x70,0x0a,0x50,0x00,0x3c,0x2a,0x00,0x00,0x00,0x20,0x01,0x5c,0x00,0x02,0x05,0x57,0xc0,0x29,0x95,0x55,0xc0,0x0a,0x54,0x40,0x00,0x02,0x94,0x07,0xc0,0x02,0x94,0x17,0xc0,0x00,0xa0,0x07,0x00,0x00,0x20,0x04,0x00,0x00,0x20,0x00,0x00,0x00,0x00,0x00,0x00 ],
            },
            enemy3 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x80,0x01,0x00,0x00,0x08,0x00,0x80,0x00,0x2c,0x00,0x00,0x00,0x0f,0x50,0x00,0x20,0x00,0x40,0x02,0x80,0x00,0x04,0x00,0x8b,0xc0,0x58,0x00,0x22,0xc0,0x14,0x00,0x00,0x00,0x00,0x20,0x02,0xf0,0x00,0x68,0x0a,0x30,0x00,0x28,0x00,0xc0,0x00,0x10,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00 ],
            },
            enemy4 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x00,0x00,0x40,0x00,0x00,0x00,0x04,0x02,0x00,0x00,0x00,0x00,0x40,0x00,0x00,0x20,0x00,0x14,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x20,0x00,0x01,0x00,0x10,0x00,0x04,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x40,0x00,0xf0,0x02,0x00,0x04,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x01,0x00,0x00,0x00,0x00,0x00 ],
            },
            heart => SpriteData {
                width: 8,
                height: 8,
                flags: 1,
                data: vec![ 0x14,0x50,0x69,0xa4,0x6a,0xa4,0x6a,0xa4,0x1a,0x90,0x06,0x40,0x01,0x00,0x00,0x00 ],
            },
            ship1 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x05,0x80,0x00,0x00,0x05,0x80,0x00,0x00,0x15,0xa0,0x00,0x00,0x16,0xa0,0x00,0x00,0x56,0xa8,0x00,0x00,0x55,0xa8,0x00,0x00,0x55,0xa8,0x00,0x01,0x55,0xaa,0x00,0x05,0x56,0x6a,0x80,0x15,0x99,0xab,0xb0,0x56,0x66,0x6e,0xec,0x58,0x09,0x80,0xbc,0x60,0x00,0x00,0x3c ],
            },
            ship2 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x05,0x80,0x00,0x00,0x05,0x80,0x00,0x00,0x16,0xa0,0x00,0x00,0x16,0xa0,0x00,0x00,0x5a,0xa8,0x00,0x00,0x5a,0xac,0x00,0x00,0x5a,0xbc,0x00,0x01,0x5a,0xaf,0x00,0x01,0x66,0xbf,0x00,0x05,0x9a,0xaf,0xc0,0x15,0x66,0xbf,0xf0,0x16,0x0a,0xc3,0xf0,0x18,0x00,0x00,0xf0 ],
            },
            ship3 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x05,0x80,0x00,0x00,0x06,0x80,0x00,0x00,0x16,0xb0,0x00,0x00,0x1a,0xb0,0x00,0x00,0x1a,0xf0,0x00,0x00,0x5a,0xfc,0x00,0x00,0x6b,0xfc,0x00,0x00,0x6b,0xfc,0x00,0x01,0xae,0xff,0x00,0x01,0xab,0xff,0x00,0x06,0xae,0xff,0xc0,0x06,0x8b,0xcf,0xc0,0x06,0x00,0x03,0xc0 ],
            },
            ship4 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x05,0x40,0x00,0x00,0x05,0x40,0x00,0x00,0x15,0x60,0x00,0x00,0x15,0xa0,0x00,0x00,0x55,0xa8,0x00,0x00,0x55,0xa8,0x00,0x00,0x55,0x68,0x00,0x01,0x55,0xaa,0x00,0x01,0x56,0x6a,0x00,0x05,0x55,0x9a,0xc0,0x15,0x56,0x6e,0xb0,0x16,0x09,0x82,0xe0,0x18,0x00,0x00,0xb0 ],
            },
            ship5 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x05,0x40,0x00,0x00,0x05,0x40,0x00,0x00,0x15,0x50,0x00,0x00,0x15,0x50,0x00,0x00,0x15,0x60,0x00,0x00,0x55,0x58,0x00,0x00,0x55,0x68,0x00,0x00,0x55,0x98,0x00,0x01,0x55,0x6a,0x00,0x01,0x55,0x9a,0x00,0x05,0x56,0x6a,0x80,0x05,0x45,0x8a,0xc0,0x05,0x00,0x02,0xc0 ],
            },
            ship6 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x00,0x40,0x00,0x00,0x00,0x40,0x00,0x00,0x00,0x40,0x00,0x00,0x01,0x40,0x00,0x00,0x01,0x60,0x00,0x00,0x05,0x60,0x00,0x00,0x01,0xa8,0x00,0x00,0x40,0xaa,0x00,0x00,0x14,0x2a,0x00,0x00,0x15,0x00,0x00,0x00,0x55,0xa2,0x00,0x01,0x55,0x62,0x80,0x15,0x86,0x4b,0xa0,0x19,0x05,0x8e,0xf8,0x18,0x08,0x00,0x3c,0x40,0x00,0x00,0x0c ],
            },
            ship7 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x00,0x40,0x00,0x00,0x00,0x40,0x00,0x00,0x00,0x60,0x00,0x00,0x01,0x60,0x00,0x00,0x05,0x40,0x00,0x00,0x04,0x82,0x00,0x00,0x00,0x02,0x00,0x00,0x10,0x0a,0x80,0x00,0x05,0x0a,0x00,0x00,0x01,0x40,0x00,0x00,0x15,0x00,0x80,0x15,0x05,0xa0,0xa0,0x16,0x00,0x62,0xe0,0x10,0x01,0x83,0x8b,0x60,0x06,0x00,0x0c,0x00,0x08,0x00,0x0c ],
            },
            ship8 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x00,0x00,0x00,0x00,0x00,0x10,0x00,0x00,0x00,0x90,0x00,0x00,0x08,0x80,0x00,0x00,0x08,0x41,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x00,0x00,0x00,0x20,0x01,0x10,0x00,0x0a,0x00,0x40,0x00,0x88,0x00,0x00,0x02,0x00,0x00,0x10,0x21,0x08,0x40,0x04,0x00,0x00,0x80,0x00,0x90,0x00,0x41,0x00,0x00,0x08,0x03,0x47,0x00,0x00,0x00,0x0c ],
            },
            ship9 => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0x40,0x00,0x00,0x00,0x00,0x02,0x00,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x10,0x02,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x00,0x04,0x00,0x00,0x40,0x00,0x10,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x01,0x00,0x00,0x00,0x00,0x04 ],
            },
            turret => SpriteData {
                width: 16,
                height: 16,
                flags: 1,
                data: vec![ 0x01,0x50,0x1b,0x00,0x00,0x60,0x28,0x00,0x01,0x90,0x2b,0x00,0x01,0x6f,0xee,0x00,0x01,0x5a,0xbb,0x00,0x05,0x6a,0xef,0xc0,0x15,0x5a,0xbb,0xf0,0x55,0x6b,0xaf,0xfc,0x01,0x5f,0xef,0x00,0x00,0x6b,0xac,0x00,0x00,0x1a,0xb0,0x00,0x00,0x06,0x80,0x00,0x00,0x06,0x80,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00 ],
            },
        }
    }
}


