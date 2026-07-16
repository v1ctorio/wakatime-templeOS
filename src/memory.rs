// 446F635301000000FFFFFFFFFFFFFF7F
const DocS_MAGIC_N: [u8; 16] = 
                     [0x44, 0x6F, 0x63, 0x53, // 'DocS'
                      0x01, 0x00, 0x00, 0x00,
                      0xFF, 0xFF, 0xFF, 0xFF,
                      0xFF, 0xFF, 0xFF, 0x7F ];

const FILENAME_OFFSET: u32 = 6 * 4;
const FILENAME_SIZE: u32 = ( (16*4) + 2) * 4; //? I'm not sure about this one

const MEM_DUMP_SIZE: usize = 553780499;

type MemDump = [u8; MEM_DUMP_SIZE];


