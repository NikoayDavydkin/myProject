#[allow(dead_code)]
pub mod attribute_ids {
    use uuid::Uuid;
    pub const AMAZON_ASIN: Uuid = Uuid::from_bytes([
        //05d5fdb2-19f8-451d-a960-7e648457d746
        0x05, 0xd5, 0xfd, 0xb2, 0x19, 0xf8, 0x45, 0x1d, 0xa9, 0x60, 0x7e, 0x64, 0x84, 0x57, 0xd7,
        0x46,
    ]);
    pub const AMAZON_FEATURES: Uuid = Uuid::from_bytes([
        //a73cbcf9-4695-47b2-a787-9ea92b93e12d
        0xa7, 0x3c, 0xbc, 0xf9, 0x46, 0x95, 0x47, 0xb2, 0xa7, 0x87, 0x9e, 0xa9, 0x2b, 0x93, 0xe1,
        0x2d,
    ]);
    pub const ASPECTS: Uuid = Uuid::from_bytes([
        //d32e01e6-cdea-4223-b636-986e9353814d
        0xd3, 0x2e, 0x01, 0xe6, 0xcd, 0xea, 0x42, 0x23, 0xb6, 0x36, 0x98, 0x6e, 0x93, 0x53, 0x81,
        0x4d,
    ]);
    pub const BATTERY_CAPACITY: Uuid = Uuid::from_bytes([
        //f7753b1c-4d40-47ea-be13-46917491807a
        0xf7, 0x75, 0x3b, 0x1c, 0x4d, 0x40, 0x47, 0xea, 0xbe, 0x13, 0x46, 0x91, 0x74, 0x91, 0x80,
        0x7a,
    ]);
    pub const BODY: Uuid = Uuid::from_bytes([
        //b17183e2-ffe3-4a94-b3f6-9518ef4b5a6b
        0xb1, 0x71, 0x83, 0xe2, 0xff, 0xe3, 0x4a, 0x94, 0xb3, 0xf6, 0x95, 0x18, 0xef, 0x4b, 0x5a,
        0x6b,
    ]);
    pub const BRAND: Uuid = Uuid::from_bytes([
        //5ae6876c-039f-4c58-8832-54de8dea1d3c
        0x5a, 0xe6, 0x87, 0x6c, 0x03, 0x9f, 0x4c, 0x58, 0x88, 0x32, 0x54, 0xde, 0x8d, 0xea, 0x1d,
        0x3c,
    ]);
    pub const CPU: Uuid = Uuid::from_bytes([
        //1616a0b4-2ac3-4b7b-b23e-238cce1306b2
        0x16, 0x16, 0xa0, 0xb4, 0x2a, 0xc3, 0x4b, 0x7b, 0xb2, 0x3e, 0x23, 0x8c, 0xce, 0x13, 0x06,
        0xb2,
    ]);
    pub const CPU_CACHE: Uuid = Uuid::from_bytes([
        //b1f9fe5f-f465-4923-8999-e531d48d0d97
        0xb1, 0xf9, 0xfe, 0x5f, 0xf4, 0x65, 0x49, 0x23, 0x89, 0x99, 0xe5, 0x31, 0xd4, 0x8d, 0x0d,
        0x97,
    ]);
    pub const CPU_CLOCK_SPEED: Uuid = Uuid::from_bytes([
        //fbdc1e45-1b39-4e0b-a70f-ec8f29f5cdfe
        0xfb, 0xdc, 0x1e, 0x45, 0x1b, 0x39, 0x4e, 0x0b, 0xa7, 0x0f, 0xec, 0x8f, 0x29, 0xf5, 0xcd,
        0xfe,
    ]);
    pub const CPU_CLOCK_SPEED_MAX: Uuid = Uuid::from_bytes([
        //54558544-85ec-4a09-81f8-34d5b3a59877
        0x54, 0x55, 0x85, 0x44, 0x85, 0xec, 0x4a, 0x09, 0x81, 0xf8, 0x34, 0xd5, 0xb3, 0xa5, 0x98,
        0x77,
    ]);
    pub const CPU_GROUP: Uuid = Uuid::from_bytes([
        //2fbdb9ba-3ea0-4b34-9b13-8599a26b4175
        0x2f, 0xbd, 0xb9, 0xba, 0x3e, 0xa0, 0x4b, 0x34, 0x9b, 0x13, 0x85, 0x99, 0xa2, 0x6b, 0x41,
        0x75,
    ]);
    pub const CPU_ID: Uuid = Uuid::from_bytes([
        //02cfdfe8-ff7c-484f-ba97-dc3a923a0793
        0x02, 0xcf, 0xdf, 0xe8, 0xff, 0x7c, 0x48, 0x4f, 0xba, 0x97, 0xdc, 0x3a, 0x92, 0x3a, 0x07,
        0x93,
    ]);
    pub const CPU_NUM_CORES: Uuid = Uuid::from_bytes([
        //adff4eae-94c3-4839-9c64-044775a84ccb
        0xad, 0xff, 0x4e, 0xae, 0x94, 0xc3, 0x48, 0x39, 0x9c, 0x64, 0x04, 0x47, 0x75, 0xa8, 0x4c,
        0xcb,
    ]);
    pub const CPU_PASSMARK: Uuid = Uuid::from_bytes([
        //7f34f783-561a-4d99-92ee-8e556c41c73f
        0x7f, 0x34, 0xf7, 0x83, 0x56, 0x1a, 0x4d, 0x99, 0x92, 0xee, 0x8e, 0x55, 0x6c, 0x41, 0xc7,
        0x3f,
    ]);
    pub const CPU_PASSMARK_SINGLE_THREAD: Uuid = Uuid::from_bytes([
        //2541e561-80b2-44e6-aab2-da3cb94e67d3
        0x25, 0x41, 0xe5, 0x61, 0x80, 0xb2, 0x44, 0xe6, 0xaa, 0xb2, 0xda, 0x3c, 0xb9, 0x4e, 0x67,
        0xd3,
    ]);
    pub const CPU_TDP: Uuid = Uuid::from_bytes([
        //f980d917-ebb1-40c5-83fc-2c5d3873b293
        0xf9, 0x80, 0xd9, 0x17, 0xeb, 0xb1, 0x40, 0xc5, 0x83, 0xfc, 0x2c, 0x5d, 0x38, 0x73, 0xb2,
        0x93,
    ]);
    pub const CPU_TDP_DOWN: Uuid = Uuid::from_bytes([
        //384c8456-b6f9-4f1a-aa4b-4b88f54069c3
        0x38, 0x4c, 0x84, 0x56, 0xb6, 0xf9, 0x4f, 0x1a, 0xaa, 0x4b, 0x4b, 0x88, 0xf5, 0x40, 0x69,
        0xc3,
    ]);
    pub const CPU_TDP_DOWN_FREQUENCY: Uuid = Uuid::from_bytes([
        //a30679b5-328c-4353-9a0e-524b5159625b
        0xa3, 0x06, 0x79, 0xb5, 0x32, 0x8c, 0x43, 0x53, 0x9a, 0x0e, 0x52, 0x4b, 0x51, 0x59, 0x62,
        0x5b,
    ]);
    pub const CPU_TDP_UP: Uuid = Uuid::from_bytes([
        //7832ea70-338a-44da-901f-65d69a75991c
        0x78, 0x32, 0xea, 0x70, 0x33, 0x8a, 0x44, 0xda, 0x90, 0x1f, 0x65, 0xd6, 0x9a, 0x75, 0x99,
        0x1c,
    ]);
    pub const CPU_TDP_UP_FREQUENCY: Uuid = Uuid::from_bytes([
        //79e83429-66c5-481b-9f5d-b74d8ee2f32b
        0x79, 0xe8, 0x34, 0x29, 0x66, 0xc5, 0x48, 0x1b, 0x9f, 0x5d, 0xb7, 0x4d, 0x8e, 0xe2, 0xf3,
        0x2b,
    ]);
    pub const CREATED: Uuid = Uuid::from_bytes([
        //a238cd97-51b2-42f1-92d3-db947eaa6263
        0xa2, 0x38, 0xcd, 0x97, 0x51, 0xb2, 0x42, 0xf1, 0x92, 0xd3, 0xdb, 0x94, 0x7e, 0xaa, 0x62,
        0x63,
    ]);
    pub const DEFAULT_SORT_ORDER: Uuid = Uuid::from_bytes([
        //ae5b918c-270e-467a-be0a-0863255f1932
        0xae, 0x5b, 0x91, 0x8c, 0x27, 0x0e, 0x46, 0x7a, 0xbe, 0x0a, 0x08, 0x63, 0x25, 0x5f, 0x19,
        0x32,
    ]);
    pub const DESCRIPTION: Uuid = Uuid::from_bytes([
        //40255a31-13d7-4db8-8b06-5cb0b0e1b510
        0x40, 0x25, 0x5a, 0x31, 0x13, 0xd7, 0x4d, 0xb8, 0x8b, 0x06, 0x5c, 0xb0, 0xb0, 0xe1, 0xb5,
        0x10,
    ]);
    pub const EAN: Uuid = Uuid::from_bytes([
        //9d91d9ad-f0c1-4d2b-bceb-a3f4b538f6df
        0x9d, 0x91, 0xd9, 0xad, 0xf0, 0xc1, 0x4d, 0x2b, 0xbc, 0xeb, 0xa3, 0xf4, 0xb5, 0x38, 0xf6,
        0xdf,
    ]);
    pub const EBAY_IDS: Uuid = Uuid::from_bytes([
        //c6c87cf3-f47e-4b50-b942-1df41d15b7cb
        0xc6, 0xc8, 0x7c, 0xf3, 0xf4, 0x7e, 0x4b, 0x50, 0xb9, 0x42, 0x1d, 0xf4, 0x1d, 0x15, 0xb7,
        0xcb,
    ]);
    pub const EMMC_SIZE: Uuid = Uuid::from_bytes([
        //2b453399-c6e1-4c06-a0dd-d0df37c29068
        0x2b, 0x45, 0x33, 0x99, 0xc6, 0xe1, 0x4c, 0x06, 0xa0, 0xdd, 0xd0, 0xdf, 0x37, 0xc2, 0x90,
        0x68,
    ]);
    pub const FAMILY: Uuid = Uuid::from_bytes([
        //2217e973-7dde-4eae-8639-33ed8825d28c
        0x22, 0x17, 0xe9, 0x73, 0x7d, 0xde, 0x4e, 0xae, 0x86, 0x39, 0x33, 0xed, 0x88, 0x25, 0xd2,
        0x8c,
    ]);
    pub const FOR_CATEGORIES: Uuid = Uuid::from_bytes([
        //96917063-e02d-46fa-bbfb-89dd6fd553c3
        0x96, 0x91, 0x70, 0x63, 0xe0, 0x2d, 0x46, 0xfa, 0xbb, 0xfb, 0x89, 0xdd, 0x6f, 0xd5, 0x53,
        0xc3,
    ]);
    pub const GPU: Uuid = Uuid::from_bytes([
        //42cc2d2f-5ca3-4731-8b84-877ee8cc2eb0
        0x42, 0xcc, 0x2d, 0x2f, 0x5c, 0xa3, 0x47, 0x31, 0x8b, 0x84, 0x87, 0x7e, 0xe8, 0xcc, 0x2e,
        0xb0,
    ]);
    pub const GPU_ID: Uuid = Uuid::from_bytes([
        //8286899d-2e19-4e46-8abc-63c3a500e81f
        0x82, 0x86, 0x89, 0x9d, 0x2e, 0x19, 0x4e, 0x46, 0x8a, 0xbc, 0x63, 0xc3, 0xa5, 0x00, 0xe8,
        0x1f,
    ]);
    pub const GPU_PASSMARK: Uuid = Uuid::from_bytes([
        //60fa3b2e-dda7-4db4-97ab-11933c6823de
        0x60, 0xfa, 0x3b, 0x2e, 0xdd, 0xa7, 0x4d, 0xb4, 0x97, 0xab, 0x11, 0x93, 0x3c, 0x68, 0x23,
        0xde,
    ]);
    pub const HDD_SIZE: Uuid = Uuid::from_bytes([
        //cd9c6614-3f28-432f-b7b4-e7db26ca5627
        0xcd, 0x9c, 0x66, 0x14, 0x3f, 0x28, 0x43, 0x2f, 0xb7, 0xb4, 0xe7, 0xdb, 0x26, 0xca, 0x56,
        0x27,
    ]);
    pub const IMAGE_URL: Uuid = Uuid::from_bytes([
        //2e190eec-57ae-4c68-a431-1de68aa8e0e8
        0x2e, 0x19, 0x0e, 0xec, 0x57, 0xae, 0x4c, 0x68, 0xa4, 0x31, 0x1d, 0xe6, 0x8a, 0xa8, 0xe0,
        0xe8,
    ]);
    pub const INTEL_ATTRIBUTES: Uuid = Uuid::from_bytes([
        //2d94d234-d2f9-47a6-a583-376674c2588f
        0x2d, 0x94, 0xd2, 0x34, 0xd2, 0xf9, 0x47, 0xa6, 0xa5, 0x83, 0x37, 0x66, 0x74, 0xc2, 0x58,
        0x8f,
    ]);
    pub const INTEL_PRODUCT_ID: Uuid = Uuid::from_bytes([
        //1fca7c10-000b-46ca-968c-85b3fbb4c98b
        0x1f, 0xca, 0x7c, 0x10, 0x00, 0x0b, 0x46, 0xca, 0x96, 0x8c, 0x85, 0xb3, 0xfb, 0xb4, 0xc9,
        0x8b,
    ]);
    pub const MANUFACTURER: Uuid = Uuid::from_bytes([
        //5d3074dd-ddef-435e-b0a6-fc436c57c998
        0x5d, 0x30, 0x74, 0xdd, 0xdd, 0xef, 0x43, 0x5e, 0xb0, 0xa6, 0xfc, 0x43, 0x6c, 0x57, 0xc9,
        0x98,
    ]);
    pub const MODEL: Uuid = Uuid::from_bytes([
        //284a833a-e095-4489-a2c4-a0389f56a78c
        0x28, 0x4a, 0x83, 0x3a, 0xe0, 0x95, 0x44, 0x89, 0xa2, 0xc4, 0xa0, 0x38, 0x9f, 0x56, 0xa7,
        0x8c,
    ]);
    pub const MPN: Uuid = Uuid::from_bytes([
        //e32c9237-0c67-42dc-ba9a-4a317ab80c27
        0xe3, 0x2c, 0x92, 0x37, 0x0c, 0x67, 0x42, 0xdc, 0xba, 0x9a, 0x4a, 0x31, 0x7a, 0xb8, 0x0c,
        0x27,
    ]);
    pub const OPERATING_SYSTEM: Uuid = Uuid::from_bytes([
        //3b0e2207-50c8-4d44-81af-0aa02c80f096
        0x3b, 0x0e, 0x22, 0x07, 0x50, 0xc8, 0x4d, 0x44, 0x81, 0xaf, 0x0a, 0xa0, 0x2c, 0x80, 0xf0,
        0x96,
    ]);
    pub const PASSMARK_URL: Uuid = Uuid::from_bytes([
        //19dd3552-66c9-4b14-99e1-8dcebf33e19e
        0x19, 0xdd, 0x35, 0x52, 0x66, 0xc9, 0x4b, 0x14, 0x99, 0xe1, 0x8d, 0xce, 0xbf, 0x33, 0xe1,
        0x9e,
    ]);
    pub const PRICE: Uuid = Uuid::from_bytes([
        //3b0e2207-50c8-4d44-81af-0aa02c8790f0
        0x3b, 0x0e, 0x22, 0x07, 0x50, 0xc8, 0x4d, 0x44, 0x81, 0xaf, 0x0a, 0xa0, 0x2c, 0x87, 0x90,
        0xf0,
    ]);
    pub const PRODUCT_LINE: Uuid = Uuid::from_bytes([
        //9def25c8-3637-4fb5-8a85-3501955c7272
        0x9d, 0xef, 0x25, 0xc8, 0x36, 0x37, 0x4f, 0xb5, 0x8a, 0x85, 0x35, 0x01, 0x95, 0x5c, 0x72,
        0x72,
    ]);
    pub const RAM_SIZE: Uuid = Uuid::from_bytes([
        //dbe2b3e9-8143-4a65-8564-e933d291de89
        0xdb, 0xe2, 0xb3, 0xe9, 0x81, 0x43, 0x4a, 0x65, 0x85, 0x64, 0xe9, 0x33, 0xd2, 0x91, 0xde,
        0x89,
    ]);
    pub const RAM_SPEED: Uuid = Uuid::from_bytes([
        //00433b8d-b37c-4b2f-a5bf-fd9575f73459
        0x00, 0x43, 0x3b, 0x8d, 0xb3, 0x7c, 0x4b, 0x2f, 0xa5, 0xbf, 0xfd, 0x95, 0x75, 0xf7, 0x34,
        0x59,
    ]);
    pub const RAM_TYPE: Uuid = Uuid::from_bytes([
        //e043ec8c-c962-482d-b55b-be6e3a462bb0
        0xe0, 0x43, 0xec, 0x8c, 0xc9, 0x62, 0x48, 0x2d, 0xb5, 0x5b, 0xbe, 0x6e, 0x3a, 0x46, 0x2b,
        0xb0,
    ]);
    pub const SCREEN_RESOLUTION: Uuid = Uuid::from_bytes([
        //7b129088-c28e-404f-9ab9-660ea8228d30
        0x7b, 0x12, 0x90, 0x88, 0xc2, 0x8e, 0x40, 0x4f, 0x9a, 0xb9, 0x66, 0x0e, 0xa8, 0x22, 0x8d,
        0x30,
    ]);
    pub const SCREEN_SIZE: Uuid = Uuid::from_bytes([
        //899e496e-3ee8-403f-aa6f-b5188b5f92db
        0x89, 0x9e, 0x49, 0x6e, 0x3e, 0xe8, 0x40, 0x3f, 0xaa, 0x6f, 0xb5, 0x18, 0x8b, 0x5f, 0x92,
        0xdb,
    ]);
    pub const SKU: Uuid = Uuid::from_bytes([
        //69c2af33-a516-4aca-9e41-9d2e347bee72
        0x69, 0xc2, 0xaf, 0x33, 0xa5, 0x16, 0x4a, 0xca, 0x9e, 0x41, 0x9d, 0x2e, 0x34, 0x7b, 0xee,
        0x72,
    ]);
    pub const SOURCE_NAME: Uuid = Uuid::from_bytes([
        //ebe75535-f3a7-4719-8313-106c489cae27
        0xeb, 0xe7, 0x55, 0x35, 0xf3, 0xa7, 0x47, 0x19, 0x83, 0x13, 0x10, 0x6c, 0x48, 0x9c, 0xae,
        0x27,
    ]);
    pub const SOURCES: Uuid = Uuid::from_bytes([
        //70eed22b-957e-42b8-8b63-924fe290c2e6
        0x70, 0xee, 0xd2, 0x2b, 0x95, 0x7e, 0x42, 0xb8, 0x8b, 0x63, 0x92, 0x4f, 0xe2, 0x90, 0xc2,
        0xe6,
    ]);
    pub const SPAM: Uuid = Uuid::from_bytes([
        //ca3d9ff1-8a9a-4f48-a098-3893f13bdf4a
        0xca, 0x3d, 0x9f, 0xf1, 0x8a, 0x9a, 0x4f, 0x48, 0xa0, 0x98, 0x38, 0x93, 0xf1, 0x3b, 0xdf,
        0x4a,
    ]);
    pub const SSD_SIZE: Uuid = Uuid::from_bytes([
        //caa95107-017b-4245-a5f4-cd8e7059d438
        0xca, 0xa9, 0x51, 0x07, 0x01, 0x7b, 0x42, 0x45, 0xa5, 0xf4, 0xcd, 0x8e, 0x70, 0x59, 0xd4,
        0x38,
    ]);
    pub const SUMMARY: Uuid = Uuid::from_bytes([
        //9be81c4b-1be3-466d-adc7-38bd7adba714
        0x9b, 0xe8, 0x1c, 0x4b, 0x1b, 0xe3, 0x46, 0x6d, 0xad, 0xc7, 0x38, 0xbd, 0x7a, 0xdb, 0xa7,
        0x14,
    ]);
    pub const SYNTH_BENCH: Uuid = Uuid::from_bytes([
        //d946429a-63ae-4717-b360-582dc621d26d
        0xd9, 0x46, 0x42, 0x9a, 0x63, 0xae, 0x47, 0x17, 0xb3, 0x60, 0x58, 0x2d, 0xc6, 0x21, 0xd2,
        0x6d,
    ]);
    pub const TITLE: Uuid = Uuid::from_bytes([
        //b6925c05-5667-4f28-803d-34db6b7a5b6a
        0xb6, 0x92, 0x5c, 0x05, 0x56, 0x67, 0x4f, 0x28, 0x80, 0x3d, 0x34, 0xdb, 0x6b, 0x7a, 0x5b,
        0x6a,
    ]);
    pub const TITLE_INPUT: Uuid = Uuid::from_bytes([
        //5e165a01-9ed4-46a0-afe7-5a59a7a9d76c
        0x5e, 0x16, 0x5a, 0x01, 0x9e, 0xd4, 0x46, 0xa0, 0xaf, 0xe7, 0x5a, 0x59, 0xa7, 0xa9, 0xd7,
        0x6c,
    ]);
    pub const UPC: Uuid = Uuid::from_bytes([
        //bc45420b-7879-4cab-89dc-ad5df8071816
        0xbc, 0x45, 0x42, 0x0b, 0x78, 0x79, 0x4c, 0xab, 0x89, 0xdc, 0xad, 0x5d, 0xf8, 0x07, 0x18,
        0x16,
    ]);
    pub const UPDATED: Uuid = Uuid::from_bytes([
        //96722141-886a-4299-a7b0-347c94c3a05f
        0x96, 0x72, 0x21, 0x41, 0x88, 0x6a, 0x42, 0x99, 0xa7, 0xb0, 0x34, 0x7c, 0x94, 0xc3, 0xa0,
        0x5f,
    ]);
    pub const VIDEO_CARD: Uuid = Uuid::from_bytes([
        //53085d1c-e3f8-4854-b6a3-95b226ef7c59
        0x53, 0x08, 0x5d, 0x1c, 0xe3, 0xf8, 0x48, 0x54, 0xb6, 0xa3, 0x95, 0xb2, 0x26, 0xef, 0x7c,
        0x59,
    ]);
}

#[allow(dead_code)]
pub mod category_ids {
    use uuid::Uuid;
    pub const ARTICLES: Uuid = Uuid::from_bytes([
        //02ba33c2-d364-41cf-ab96-ce46cb6821ca
        0x02, 0xba, 0x33, 0xc2, 0xd3, 0x64, 0x41, 0xcf, 0xab, 0x96, 0xce, 0x46, 0xcb, 0x68, 0x21,
        0xca,
    ]);
    pub const BENCHMARKS: Uuid = Uuid::from_bytes([
        //68228d7b-70ed-489a-914c-d03702782a4b
        0x68, 0x22, 0x8d, 0x7b, 0x70, 0xed, 0x48, 0x9a, 0x91, 0x4c, 0xd0, 0x37, 0x02, 0x78, 0x2a,
        0x4b,
    ]);
    pub const BUYING_GUIDES: Uuid = Uuid::from_bytes([
        //bffaa5cc-3183-41f2-ae4a-22b4a79edfeb
        0xbf, 0xfa, 0xa5, 0xcc, 0x31, 0x83, 0x41, 0xf2, 0xae, 0x4a, 0x22, 0xb4, 0xa7, 0x9e, 0xdf,
        0xeb,
    ]);
    pub const COMPUTER_PARTS: Uuid = Uuid::from_bytes([
        //f42aaede-d667-4e0e-b56a-372b97e0d200
        0xf4, 0x2a, 0xae, 0xde, 0xd6, 0x67, 0x4e, 0x0e, 0xb5, 0x6a, 0x37, 0x2b, 0x97, 0xe0, 0xd2,
        0x00,
    ]);
    pub const COMPUTER_SERVERS: Uuid = Uuid::from_bytes([
        //8a30170a-ee3d-4343-9945-ca301abd9a75
        0x8a, 0x30, 0x17, 0x0a, 0xee, 0x3d, 0x43, 0x43, 0x99, 0x45, 0xca, 0x30, 0x1a, 0xbd, 0x9a,
        0x75,
    ]);
    pub const CPU_PASSMARK: Uuid = Uuid::from_bytes([
        //28ff2927-f80d-4c42-92e4-a803ea7a3699
        0x28, 0xff, 0x29, 0x27, 0xf8, 0x0d, 0x4c, 0x42, 0x92, 0xe4, 0xa8, 0x03, 0xea, 0x7a, 0x36,
        0x99,
    ]);
    pub const CPUS: Uuid = Uuid::from_bytes([
        //6d1fcddb-5ecf-4254-8b86-aee15ee4c12a
        0x6d, 0x1f, 0xcd, 0xdb, 0x5e, 0xcf, 0x42, 0x54, 0x8b, 0x86, 0xae, 0xe1, 0x5e, 0xe4, 0xc1,
        0x2a,
    ]);
    pub const DATA: Uuid = Uuid::from_bytes([
        //b0a9efa2-15c6-4d7a-9dff-bba0b4741a1f
        0xb0, 0xa9, 0xef, 0xa2, 0x15, 0xc6, 0x4d, 0x7a, 0x9d, 0xff, 0xbb, 0xa0, 0xb4, 0x74, 0x1a,
        0x1f,
    ]);
    pub const DESKTOP_COMPUTERS: Uuid = Uuid::from_bytes([
        //b80f123c-f464-423d-99d8-55cb700cfaf9
        0xb8, 0x0f, 0x12, 0x3c, 0xf4, 0x64, 0x42, 0x3d, 0x99, 0xd8, 0x55, 0xcb, 0x70, 0x0c, 0xfa,
        0xf9,
    ]);
    pub const DESKTOP_CPUS: Uuid = Uuid::from_bytes([
        //ee211e70-0038-4b3e-9df0-913308e04cce
        0xee, 0x21, 0x1e, 0x70, 0x00, 0x38, 0x4b, 0x3e, 0x9d, 0xf0, 0x91, 0x33, 0x08, 0xe0, 0x4c,
        0xce,
    ]);
    pub const DESKTOP_GPUS: Uuid = Uuid::from_bytes([
        //be369f01-3fa3-42fc-9c49-6ae4c3ddf008
        0xbe, 0x36, 0x9f, 0x01, 0x3f, 0xa3, 0x42, 0xfc, 0x9c, 0x49, 0x6a, 0xe4, 0xc3, 0xdd, 0xf0,
        0x08,
    ]);
    pub const GPU_PASSMARK: Uuid = Uuid::from_bytes([
        //693f6de2-f0bf-4db0-9abe-815fde2a8fa9
        0x69, 0x3f, 0x6d, 0xe2, 0xf0, 0xbf, 0x4d, 0xb0, 0x9a, 0xbe, 0x81, 0x5f, 0xde, 0x2a, 0x8f,
        0xa9,
    ]);
    pub const GPUS: Uuid = Uuid::from_bytes([
        //0c0c2805-177d-4e91-9c4b-1edc6ca02033
        0x0c, 0x0c, 0x28, 0x05, 0x17, 0x7d, 0x4e, 0x91, 0x9c, 0x4b, 0x1e, 0xdc, 0x6c, 0xa0, 0x20,
        0x33,
    ]);
    pub const LAPTOP_COOLERS: Uuid = Uuid::from_bytes([
        //a2b2b4e6-268b-4335-ade1-54d27adfd07a
        0xa2, 0xb2, 0xb4, 0xe6, 0x26, 0x8b, 0x43, 0x35, 0xad, 0xe1, 0x54, 0xd2, 0x7a, 0xdf, 0xd0,
        0x7a,
    ]);
    pub const LAPTOP_CPUS: Uuid = Uuid::from_bytes([
        //2d6ad7c5-3105-4430-bd94-f02f128db174
        0x2d, 0x6a, 0xd7, 0xc5, 0x31, 0x05, 0x44, 0x30, 0xbd, 0x94, 0xf0, 0x2f, 0x12, 0x8d, 0xb1,
        0x74,
    ]);
    pub const LAPTOP_GPUS: Uuid = Uuid::from_bytes([
        //dec5d755-535d-4e32-b801-2dd77486e889
        0xde, 0xc5, 0xd7, 0x55, 0x53, 0x5d, 0x4e, 0x32, 0xb8, 0x01, 0x2d, 0xd7, 0x74, 0x86, 0xe8,
        0x89,
    ]);
    pub const LAPTOPS: Uuid = Uuid::from_bytes([
        //dfe0c6a8-3b02-41d5-8eab-5375ba4bc063
        0xdf, 0xe0, 0xc6, 0xa8, 0x3b, 0x02, 0x41, 0xd5, 0x8e, 0xab, 0x53, 0x75, 0xba, 0x4b, 0xc0,
        0x63,
    ]);
    pub const PRODUCTS: Uuid = Uuid::from_bytes([
        //0497ff7b-0cd5-4bc6-879e-6f7f2e9cfee0
        0x04, 0x97, 0xff, 0x7b, 0x0c, 0xd5, 0x4b, 0xc6, 0x87, 0x9e, 0x6f, 0x7f, 0x2e, 0x9c, 0xfe,
        0xe0,
    ]);
    pub const REVIEWS: Uuid = Uuid::from_bytes([
        //dd6d022f-198a-4514-9809-35a333857cdb
        0xdd, 0x6d, 0x02, 0x2f, 0x19, 0x8a, 0x45, 0x14, 0x98, 0x09, 0x35, 0xa3, 0x33, 0x85, 0x7c,
        0xdb,
    ]);
    pub const ROOT: Uuid = Uuid::from_bytes([
        //00000000-0000-0000-0000-000000000000
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);
    pub const SERVER_CPUS: Uuid = Uuid::from_bytes([
        //339d5602-a61f-4c07-b95d-8b91db2145c4
        0x33, 0x9d, 0x56, 0x02, 0xa6, 0x1f, 0x4c, 0x07, 0xb9, 0x5d, 0x8b, 0x91, 0xdb, 0x21, 0x45,
        0xc4,
    ]);
    pub const SERVER_GPUS: Uuid = Uuid::from_bytes([
        //507f5dfa-3aee-466b-a2ad-fa265ca2e8cd
        0x50, 0x7f, 0x5d, 0xfa, 0x3a, 0xee, 0x46, 0x6b, 0xa2, 0xad, 0xfa, 0x26, 0x5c, 0xa2, 0xe8,
        0xcd,
    ]);
}

#[allow(dead_code)]
pub mod sort_order_ids {
    use uuid::Uuid;
    pub const ALPHABETICAL_A_TO_Z: Uuid = Uuid::from_bytes([
        //84672c09-225c-4e32-be0a-506f5948b23a
        0x84, 0x67, 0x2c, 0x09, 0x22, 0x5c, 0x4e, 0x32, 0xbe, 0x0a, 0x50, 0x6f, 0x59, 0x48, 0xb2,
        0x3a,
    ]);
    pub const BESTSELLING: Uuid = Uuid::from_bytes([
        //50435be6-040f-44f8-91b5-d66949fff626
        0x50, 0x43, 0x5b, 0xe6, 0x04, 0x0f, 0x44, 0xf8, 0x91, 0xb5, 0xd6, 0x69, 0x49, 0xff, 0xf6,
        0x26,
    ]);
    pub const FEATURED: Uuid = Uuid::from_bytes([
        //35c69ed2-b015-4d77-b521-58cdaeea2935
        0x35, 0xc6, 0x9e, 0xd2, 0xb0, 0x15, 0x4d, 0x77, 0xb5, 0x21, 0x58, 0xcd, 0xae, 0xea, 0x29,
        0x35,
    ]);
    pub const GRAPHICS_FAST_TO_SLOW: Uuid = Uuid::from_bytes([
        //bc234f13-ede6-40fd-9c57-86f578a5d498
        0xbc, 0x23, 0x4f, 0x13, 0xed, 0xe6, 0x40, 0xfd, 0x9c, 0x57, 0x86, 0xf5, 0x78, 0xa5, 0xd4,
        0x98,
    ]);
    pub const GRAPHICS_SPEED_PER_DOLLAR: Uuid = Uuid::from_bytes([
        //0ff9af5e-3dec-49fb-b9a5-5fbd219e9ce4
        0x0f, 0xf9, 0xaf, 0x5e, 0x3d, 0xec, 0x49, 0xfb, 0xb9, 0xa5, 0x5f, 0xbd, 0x21, 0x9e, 0x9c,
        0xe4,
    ]);
    pub const ON_SALE: Uuid = Uuid::from_bytes([
        //7663c021-fb7b-4dfa-b5d5-0a670c66df2c
        0x76, 0x63, 0xc0, 0x21, 0xfb, 0x7b, 0x4d, 0xfa, 0xb5, 0xd5, 0x0a, 0x67, 0x0c, 0x66, 0xdf,
        0x2c,
    ]);
    pub const PRICE_HIGH_TO_LOW: Uuid = Uuid::from_bytes([
        //0e6e1882-5249-4cd6-b5be-eb1f595d0e97
        0x0e, 0x6e, 0x18, 0x82, 0x52, 0x49, 0x4c, 0xd6, 0xb5, 0xbe, 0xeb, 0x1f, 0x59, 0x5d, 0x0e,
        0x97,
    ]);
    pub const PRICE_LOW_TO_HIGH: Uuid = Uuid::from_bytes([
        //54945860-6ca5-4048-aaf8-b069d2369c5a
        0x54, 0x94, 0x58, 0x60, 0x6c, 0xa5, 0x40, 0x48, 0xaa, 0xf8, 0xb0, 0x69, 0xd2, 0x36, 0x9c,
        0x5a,
    ]);
    pub const PROCESSOR_FAST_TO_SLOW: Uuid = Uuid::from_bytes([
        //70feab88-8e76-470f-936f-8d555266d8f2
        0x70, 0xfe, 0xab, 0x88, 0x8e, 0x76, 0x47, 0x0f, 0x93, 0x6f, 0x8d, 0x55, 0x52, 0x66, 0xd8,
        0xf2,
    ]);
    pub const PROCESSOR_SPEED_PER_DOLLAR: Uuid = Uuid::from_bytes([
        //513aa0d9-028c-44ad-a713-71084bf6c523
        0x51, 0x3a, 0xa0, 0xd9, 0x02, 0x8c, 0x44, 0xad, 0xa7, 0x13, 0x71, 0x08, 0x4b, 0xf6, 0xc5,
        0x23,
    ]);
    pub const RELEASE_DATE_NEW_TO_OLD: Uuid = Uuid::from_bytes([
        //00e45c8d-47c2-4701-bd35-629b30c40fe0
        0x00, 0xe4, 0x5c, 0x8d, 0x47, 0xc2, 0x47, 0x01, 0xbd, 0x35, 0x62, 0x9b, 0x30, 0xc4, 0x0f,
        0xe0,
    ]);
    pub const RELEASE_DATE_OLD_TO_NEW: Uuid = Uuid::from_bytes([
        //0cb5331b-5c34-4485-ab1f-5aad97c09b13
        0x0c, 0xb5, 0x33, 0x1b, 0x5c, 0x34, 0x44, 0x85, 0xab, 0x1f, 0x5a, 0xad, 0x97, 0xc0, 0x9b,
        0x13,
    ]);
    pub const RELEVANCE: Uuid = Uuid::from_bytes([
        //75a2385f-8227-4148-818b-f1b644b522f9
        0x75, 0xa2, 0x38, 0x5f, 0x82, 0x27, 0x41, 0x48, 0x81, 0x8b, 0xf1, 0xb6, 0x44, 0xb5, 0x22,
        0xf9,
    ]);
    pub const RESALE_PROFIT_MARGIN: Uuid = Uuid::from_bytes([
        //c4bac907-7fda-474b-b9f7-ff1f4dc9d9ab
        0xc4, 0xba, 0xc9, 0x07, 0x7f, 0xda, 0x47, 0x4b, 0xb9, 0xf7, 0xff, 0x1f, 0x4d, 0xc9, 0xd9,
        0xab,
    ]);
    pub const REVIEW_SCORE: Uuid = Uuid::from_bytes([
        //9462894e-9b7b-48dd-8eb1-6bd67de947fe
        0x94, 0x62, 0x89, 0x4e, 0x9b, 0x7b, 0x48, 0xdd, 0x8e, 0xb1, 0x6b, 0xd6, 0x7d, 0xe9, 0x47,
        0xfe,
    ]);
}

#[allow(dead_code)]
pub mod source_ids {
    use uuid::Uuid;
    pub const AMAZON: Uuid = Uuid::from_bytes([
        //6c286d26-0cf0-45ba-bbd7-e63e8a041bbd
        0x6c, 0x28, 0x6d, 0x26, 0x0c, 0xf0, 0x45, 0xba, 0xbb, 0xd7, 0xe6, 0x3e, 0x8a, 0x04, 0x1b,
        0xbd,
    ]);
    pub const AMD: Uuid = Uuid::from_bytes([
        //b6ff00b0-27e3-421d-970e-9e49799f4f90
        0xb6, 0xff, 0x00, 0xb0, 0x27, 0xe3, 0x42, 0x1d, 0x97, 0x0e, 0x9e, 0x49, 0x79, 0x9f, 0x4f,
        0x90,
    ]);
    pub const BEST_BUY: Uuid = Uuid::from_bytes([
        //04d17763-dc35-43a1-8329-e0f6c013090d
        0x04, 0xd1, 0x77, 0x63, 0xdc, 0x35, 0x43, 0xa1, 0x83, 0x29, 0xe0, 0xf6, 0xc0, 0x13, 0x09,
        0x0d,
    ]);
    pub const CPU_PASSMARK: Uuid = Uuid::from_bytes([
        //6a9cb831-536a-49fc-94d8-b5feace901c4
        0x6a, 0x9c, 0xb8, 0x31, 0x53, 0x6a, 0x49, 0xfc, 0x94, 0xd8, 0xb5, 0xfe, 0xac, 0xe9, 0x01,
        0xc4,
    ]);
    pub const DEALTECH: Uuid = Uuid::from_bytes([
        //6b5df33d-c056-4de1-911e-fd8a2deae104
        0x6b, 0x5d, 0xf3, 0x3d, 0xc0, 0x56, 0x4d, 0xe1, 0x91, 0x1e, 0xfd, 0x8a, 0x2d, 0xea, 0xe1,
        0x04,
    ]);
    pub const DELL: Uuid = Uuid::from_bytes([
        //afb13766-b325-4474-9fb3-956fc68705bc
        0xaf, 0xb1, 0x37, 0x66, 0xb3, 0x25, 0x44, 0x74, 0x9f, 0xb3, 0x95, 0x6f, 0xc6, 0x87, 0x05,
        0xbc,
    ]);
    pub const EBAY: Uuid = Uuid::from_bytes([
        //ebafffff-d86f-4fa6-8e9d-fffffffffeba
        0xeb, 0xaf, 0xff, 0xff, 0xd8, 0x6f, 0x4f, 0xa6, 0x8e, 0x9d, 0xff, 0xff, 0xff, 0xff, 0xfe,
        0xba,
    ]);
    pub const GPU_PASSMARK: Uuid = Uuid::from_bytes([
        //d7b2b409-7d32-44d7-bc18-3194ea934f1c
        0xd7, 0xb2, 0xb4, 0x09, 0x7d, 0x32, 0x44, 0xd7, 0xbc, 0x18, 0x31, 0x94, 0xea, 0x93, 0x4f,
        0x1c,
    ]);
    pub const INTEL: Uuid = Uuid::from_bytes([
        //141e1fff-d86f-4fa6-8e9d-fffffff141e1
        0x14, 0x1e, 0x1f, 0xff, 0xd8, 0x6f, 0x4f, 0xa6, 0x8e, 0x9d, 0xff, 0xff, 0xff, 0xf1, 0x41,
        0xe1,
    ]);
    pub const MEDIATEK: Uuid = Uuid::from_bytes([
        //d623687d-722a-4a48-bb10-184201940a31
        0xd6, 0x23, 0x68, 0x7d, 0x72, 0x2a, 0x4a, 0x48, 0xbb, 0x10, 0x18, 0x42, 0x01, 0x94, 0x0a,
        0x31,
    ]);
    pub const NVIDIA: Uuid = Uuid::from_bytes([
        //8110bfe3-be10-4378-bb0f-832977b4c104
        0x81, 0x10, 0xbf, 0xe3, 0xbe, 0x10, 0x43, 0x78, 0xbb, 0x0f, 0x83, 0x29, 0x77, 0xb4, 0xc1,
        0x04,
    ]);
}
