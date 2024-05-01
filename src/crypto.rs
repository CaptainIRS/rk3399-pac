#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intsts: Intsts,
    intena: Intena,
    ctrl: Ctrl,
    conf: Conf,
    brdmas: Brdmas,
    btdmas: Btdmas,
    brdmal: Brdmal,
    hrdmas: Hrdmas,
    hrdmal: Hrdmal,
    _reserved9: [u8; 0x5c],
    aes_ctrl: AesCtrl,
    aes_sts: AesSts,
    aes_din_0: AesDin0,
    aes_din_1: AesDin1,
    aes_din_2: AesDin2,
    aes_din_3: AesDin3,
    aes_dout_0: AesDout0,
    aes_dout_1: AesDout1,
    aes_dout_2: AesDout2,
    aes_dout_3: AesDout3,
    aes_iv_0: AesIv0,
    aes_iv_1: AesIv1,
    aes_iv_2: AesIv2,
    aes_iv_3: AesIv3,
    aes_key_0: AesKey0,
    aes_key_1: AesKey1,
    aes_key_2: AesKey2,
    aes_key_3: AesKey3,
    aes_key_4: AesKey4,
    aes_key_5: AesKey5,
    aes_key_6: AesKey6,
    aes_key_7: AesKey7,
    aes_cnt_0: AesCnt0,
    aes_cnt_1: AesCnt1,
    aes_cnt_2: AesCnt2,
    aes_cnt_3: AesCnt3,
    _reserved35: [u8; 0x18],
    tdes_ctrl: TdesCtrl,
    tdes_sts: TdesSts,
    tdes_din_0: TdesDin0,
    tdes_din_1: TdesDin1,
    tdes_dout_0: TdesDout0,
    tdes_dout_1: TdesDout1,
    tdes_iv_0: TdesIv0,
    tdes_iv_1: TdesIv1,
    tdes_key1_0: TdesKey1_0,
    tdes_key1_1: TdesKey1_1,
    tdes_key2_0: TdesKey2_0,
    tdes_key2_1: TdesKey2_1,
    tdes_key3_0: TdesKey3_0,
    tdes_key3_1: TdesKey3_1,
    _reserved49: [u8; 0x48],
    hash_ctrl: HashCtrl,
    hash_sts: HashSts,
    hash_msg_len: HashMsgLen,
    hash_dout_0: HashDout0,
    hash_dout_1: HashDout1,
    hash_dout_2: HashDout2,
    hash_dout_3: HashDout3,
    hash_dout_4: HashDout4,
    hash_dout_5: HashDout5,
    hash_dout_6: HashDout6,
    hash_dout_7: HashDout7,
    hash_seed_0: HashSeed0,
    hash_seed_1: HashSeed1,
    hash_seed_2: HashSeed2,
    hash_seed_3: HashSeed3,
    hash_seed_4: HashSeed4,
    _reserved65: [u8; 0x40],
    trng_ctrl: TrngCtrl,
    trng_dout_0: TrngDout0,
    trng_dout_1: TrngDout1,
    trng_dout_2: TrngDout2,
    trng_dout_3: TrngDout3,
    trng_dout_4: TrngDout4,
    trng_dout_5: TrngDout5,
    trng_dout_6: TrngDout6,
    trng_dout_7: TrngDout7,
    _reserved74: [u8; 0x5c],
    pka_ctrl: PkaCtrl,
    _reserved75: [u8; 0x7c],
    aes_tkey_0: AesTkey0,
    aes_tkey_1: AesTkey1,
    aes_tkey_2: AesTkey2,
    aes_tkey_3: AesTkey3,
    aes_tkey_4: AesTkey4,
    aes_tkey_5: AesTkey5,
    aes_tkey_6: AesTkey6,
    aes_tkey_7: AesTkey7,
    aes_twk_0: AesTwk0,
    aes_twk_1: AesTwk1,
    aes_twk_2: AesTwk2,
    aes_twk_3: AesTwk3,
    key_secure: KeySecure,
    clk_gate: ClkGate,
    _reserved89: [u8; 0xb8],
    crypto_ver: CryptoVer,
    _reserved90: [u8; 0x0c],
    pka_m: PkaM,
    _reserved91: [u8; 0xfc],
    pka_c: PkaC,
    _reserved92: [u8; 0xfc],
    pka_n: PkaN,
    _reserved93: [u8; 0xfc],
    pka_e: PkaE,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsts(&self) -> &Intsts {
        &self.intsts
    }
    #[doc = "0x04 - Interrupt Set Register"]
    #[inline(always)]
    pub const fn intena(&self) -> &Intena {
        &self.intena
    }
    #[doc = "0x08 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Configure Register"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
    #[doc = "0x10 - Block Receiving DMA Start Address Register"]
    #[inline(always)]
    pub const fn brdmas(&self) -> &Brdmas {
        &self.brdmas
    }
    #[doc = "0x14 - Block Transmiting DMA Start Address Register"]
    #[inline(always)]
    pub const fn btdmas(&self) -> &Btdmas {
        &self.btdmas
    }
    #[doc = "0x18 - Block Receiving DMA Length Register"]
    #[inline(always)]
    pub const fn brdmal(&self) -> &Brdmal {
        &self.brdmal
    }
    #[doc = "0x1c - Hash Receiving DMA Start Address Register"]
    #[inline(always)]
    pub const fn hrdmas(&self) -> &Hrdmas {
        &self.hrdmas
    }
    #[doc = "0x20 - Hash Receiving DMA Length Register"]
    #[inline(always)]
    pub const fn hrdmal(&self) -> &Hrdmal {
        &self.hrdmal
    }
    #[doc = "0x80 - AES Control Register"]
    #[inline(always)]
    pub const fn aes_ctrl(&self) -> &AesCtrl {
        &self.aes_ctrl
    }
    #[doc = "0x84 - Status Register"]
    #[inline(always)]
    pub const fn aes_sts(&self) -> &AesSts {
        &self.aes_sts
    }
    #[doc = "0x88 - AES Input Data 0 Register"]
    #[inline(always)]
    pub const fn aes_din_0(&self) -> &AesDin0 {
        &self.aes_din_0
    }
    #[doc = "0x8c - AES Input Data 1 Register"]
    #[inline(always)]
    pub const fn aes_din_1(&self) -> &AesDin1 {
        &self.aes_din_1
    }
    #[doc = "0x90 - AES Input Data 2 Register"]
    #[inline(always)]
    pub const fn aes_din_2(&self) -> &AesDin2 {
        &self.aes_din_2
    }
    #[doc = "0x94 - AES Input Data 3 Register"]
    #[inline(always)]
    pub const fn aes_din_3(&self) -> &AesDin3 {
        &self.aes_din_3
    }
    #[doc = "0x98 - AES Output Data 0 Register"]
    #[inline(always)]
    pub const fn aes_dout_0(&self) -> &AesDout0 {
        &self.aes_dout_0
    }
    #[doc = "0x9c - AES Output Data 1 Register"]
    #[inline(always)]
    pub const fn aes_dout_1(&self) -> &AesDout1 {
        &self.aes_dout_1
    }
    #[doc = "0xa0 - AES Output Data 2 Register"]
    #[inline(always)]
    pub const fn aes_dout_2(&self) -> &AesDout2 {
        &self.aes_dout_2
    }
    #[doc = "0xa4 - AES Output Data 3 Register"]
    #[inline(always)]
    pub const fn aes_dout_3(&self) -> &AesDout3 {
        &self.aes_dout_3
    }
    #[doc = "0xa8 - AES IV data 0 Register"]
    #[inline(always)]
    pub const fn aes_iv_0(&self) -> &AesIv0 {
        &self.aes_iv_0
    }
    #[doc = "0xac - AES IV data 1 Register"]
    #[inline(always)]
    pub const fn aes_iv_1(&self) -> &AesIv1 {
        &self.aes_iv_1
    }
    #[doc = "0xb0 - AES IV data 2 Register"]
    #[inline(always)]
    pub const fn aes_iv_2(&self) -> &AesIv2 {
        &self.aes_iv_2
    }
    #[doc = "0xb4 - AES IV data 3 Register"]
    #[inline(always)]
    pub const fn aes_iv_3(&self) -> &AesIv3 {
        &self.aes_iv_3
    }
    #[doc = "0xb8 - AES Key data 0 Register"]
    #[inline(always)]
    pub const fn aes_key_0(&self) -> &AesKey0 {
        &self.aes_key_0
    }
    #[doc = "0xbc - AES Key data 1 Register"]
    #[inline(always)]
    pub const fn aes_key_1(&self) -> &AesKey1 {
        &self.aes_key_1
    }
    #[doc = "0xc0 - AES Key data 2 Register"]
    #[inline(always)]
    pub const fn aes_key_2(&self) -> &AesKey2 {
        &self.aes_key_2
    }
    #[doc = "0xc4 - AES Key data 3 Register"]
    #[inline(always)]
    pub const fn aes_key_3(&self) -> &AesKey3 {
        &self.aes_key_3
    }
    #[doc = "0xc8 - AES Key data 4 Register"]
    #[inline(always)]
    pub const fn aes_key_4(&self) -> &AesKey4 {
        &self.aes_key_4
    }
    #[doc = "0xcc - AES Key data 5 Register"]
    #[inline(always)]
    pub const fn aes_key_5(&self) -> &AesKey5 {
        &self.aes_key_5
    }
    #[doc = "0xd0 - AES Key data 6 Register"]
    #[inline(always)]
    pub const fn aes_key_6(&self) -> &AesKey6 {
        &self.aes_key_6
    }
    #[doc = "0xd4 - AES Key data 7 Register"]
    #[inline(always)]
    pub const fn aes_key_7(&self) -> &AesKey7 {
        &self.aes_key_7
    }
    #[doc = "0xd8 - AES Input Counter 0 Register"]
    #[inline(always)]
    pub const fn aes_cnt_0(&self) -> &AesCnt0 {
        &self.aes_cnt_0
    }
    #[doc = "0xdc - AES Input Counter 1 Register"]
    #[inline(always)]
    pub const fn aes_cnt_1(&self) -> &AesCnt1 {
        &self.aes_cnt_1
    }
    #[doc = "0xe0 - AES Input Counter 2 Register"]
    #[inline(always)]
    pub const fn aes_cnt_2(&self) -> &AesCnt2 {
        &self.aes_cnt_2
    }
    #[doc = "0xe4 - AES Input Counter 3 Register"]
    #[inline(always)]
    pub const fn aes_cnt_3(&self) -> &AesCnt3 {
        &self.aes_cnt_3
    }
    #[doc = "0x100 - TDES Control Register"]
    #[inline(always)]
    pub const fn tdes_ctrl(&self) -> &TdesCtrl {
        &self.tdes_ctrl
    }
    #[doc = "0x104 - Status Register"]
    #[inline(always)]
    pub const fn tdes_sts(&self) -> &TdesSts {
        &self.tdes_sts
    }
    #[doc = "0x108 - TDES Input Data 0 Register"]
    #[inline(always)]
    pub const fn tdes_din_0(&self) -> &TdesDin0 {
        &self.tdes_din_0
    }
    #[doc = "0x10c - TDES Input Data 1 Register"]
    #[inline(always)]
    pub const fn tdes_din_1(&self) -> &TdesDin1 {
        &self.tdes_din_1
    }
    #[doc = "0x110 - TDES Output Data 0 Register"]
    #[inline(always)]
    pub const fn tdes_dout_0(&self) -> &TdesDout0 {
        &self.tdes_dout_0
    }
    #[doc = "0x114 - TDES Output Data 1 Register"]
    #[inline(always)]
    pub const fn tdes_dout_1(&self) -> &TdesDout1 {
        &self.tdes_dout_1
    }
    #[doc = "0x118 - TDES IV data 0 Register"]
    #[inline(always)]
    pub const fn tdes_iv_0(&self) -> &TdesIv0 {
        &self.tdes_iv_0
    }
    #[doc = "0x11c - TDES IV data 1 Register"]
    #[inline(always)]
    pub const fn tdes_iv_1(&self) -> &TdesIv1 {
        &self.tdes_iv_1
    }
    #[doc = "0x120 - TDES Key1 data 1 Register"]
    #[inline(always)]
    pub const fn tdes_key1_0(&self) -> &TdesKey1_0 {
        &self.tdes_key1_0
    }
    #[doc = "0x124 - TDES Key1 data 1 Register"]
    #[inline(always)]
    pub const fn tdes_key1_1(&self) -> &TdesKey1_1 {
        &self.tdes_key1_1
    }
    #[doc = "0x128 - TDES Key2 data 0 Register"]
    #[inline(always)]
    pub const fn tdes_key2_0(&self) -> &TdesKey2_0 {
        &self.tdes_key2_0
    }
    #[doc = "0x12c - TDES Key2 data 1 Register"]
    #[inline(always)]
    pub const fn tdes_key2_1(&self) -> &TdesKey2_1 {
        &self.tdes_key2_1
    }
    #[doc = "0x130 - TDES Key3 data 0 Register"]
    #[inline(always)]
    pub const fn tdes_key3_0(&self) -> &TdesKey3_0 {
        &self.tdes_key3_0
    }
    #[doc = "0x134 - TDES Key3 data 1 Register"]
    #[inline(always)]
    pub const fn tdes_key3_1(&self) -> &TdesKey3_1 {
        &self.tdes_key3_1
    }
    #[doc = "0x180 - Hash Control Register"]
    #[inline(always)]
    pub const fn hash_ctrl(&self) -> &HashCtrl {
        &self.hash_ctrl
    }
    #[doc = "0x184 - Hash Status Register"]
    #[inline(always)]
    pub const fn hash_sts(&self) -> &HashSts {
        &self.hash_sts
    }
    #[doc = "0x188 - Hash Message Len"]
    #[inline(always)]
    pub const fn hash_msg_len(&self) -> &HashMsgLen {
        &self.hash_msg_len
    }
    #[doc = "0x18c - Hash Result Register 0"]
    #[inline(always)]
    pub const fn hash_dout_0(&self) -> &HashDout0 {
        &self.hash_dout_0
    }
    #[doc = "0x190 - Hash Result Register 1"]
    #[inline(always)]
    pub const fn hash_dout_1(&self) -> &HashDout1 {
        &self.hash_dout_1
    }
    #[doc = "0x194 - Hash Result Register 2"]
    #[inline(always)]
    pub const fn hash_dout_2(&self) -> &HashDout2 {
        &self.hash_dout_2
    }
    #[doc = "0x198 - Hash Result Register 3"]
    #[inline(always)]
    pub const fn hash_dout_3(&self) -> &HashDout3 {
        &self.hash_dout_3
    }
    #[doc = "0x19c - Hash Result Register 4"]
    #[inline(always)]
    pub const fn hash_dout_4(&self) -> &HashDout4 {
        &self.hash_dout_4
    }
    #[doc = "0x1a0 - Hash Result Register 5"]
    #[inline(always)]
    pub const fn hash_dout_5(&self) -> &HashDout5 {
        &self.hash_dout_5
    }
    #[doc = "0x1a4 - Hash Result Register 6"]
    #[inline(always)]
    pub const fn hash_dout_6(&self) -> &HashDout6 {
        &self.hash_dout_6
    }
    #[doc = "0x1a8 - Hash Result Register 7"]
    #[inline(always)]
    pub const fn hash_dout_7(&self) -> &HashDout7 {
        &self.hash_dout_7
    }
    #[doc = "0x1ac - PRNG Seed/HMAC Key Register 0"]
    #[inline(always)]
    pub const fn hash_seed_0(&self) -> &HashSeed0 {
        &self.hash_seed_0
    }
    #[doc = "0x1b0 - PRNG Seed/HMAC Key Register 1"]
    #[inline(always)]
    pub const fn hash_seed_1(&self) -> &HashSeed1 {
        &self.hash_seed_1
    }
    #[doc = "0x1b4 - PRNG Seed/HMAC Key Register 2"]
    #[inline(always)]
    pub const fn hash_seed_2(&self) -> &HashSeed2 {
        &self.hash_seed_2
    }
    #[doc = "0x1b8 - PRNG Seed/HMAC Key Register 3"]
    #[inline(always)]
    pub const fn hash_seed_3(&self) -> &HashSeed3 {
        &self.hash_seed_3
    }
    #[doc = "0x1bc - PRNG Seed/HMAC Key Register 4"]
    #[inline(always)]
    pub const fn hash_seed_4(&self) -> &HashSeed4 {
        &self.hash_seed_4
    }
    #[doc = "0x200 - TRNG Control register"]
    #[inline(always)]
    pub const fn trng_ctrl(&self) -> &TrngCtrl {
        &self.trng_ctrl
    }
    #[doc = "0x204 - TRNG output register 0"]
    #[inline(always)]
    pub const fn trng_dout_0(&self) -> &TrngDout0 {
        &self.trng_dout_0
    }
    #[doc = "0x208 - TRNG output register 1"]
    #[inline(always)]
    pub const fn trng_dout_1(&self) -> &TrngDout1 {
        &self.trng_dout_1
    }
    #[doc = "0x20c - TRNG output register 2"]
    #[inline(always)]
    pub const fn trng_dout_2(&self) -> &TrngDout2 {
        &self.trng_dout_2
    }
    #[doc = "0x210 - TRNG output register 3"]
    #[inline(always)]
    pub const fn trng_dout_3(&self) -> &TrngDout3 {
        &self.trng_dout_3
    }
    #[doc = "0x214 - TRNG output register 4"]
    #[inline(always)]
    pub const fn trng_dout_4(&self) -> &TrngDout4 {
        &self.trng_dout_4
    }
    #[doc = "0x218 - TRNG output register 5"]
    #[inline(always)]
    pub const fn trng_dout_5(&self) -> &TrngDout5 {
        &self.trng_dout_5
    }
    #[doc = "0x21c - TRNG output register 6"]
    #[inline(always)]
    pub const fn trng_dout_6(&self) -> &TrngDout6 {
        &self.trng_dout_6
    }
    #[doc = "0x220 - TRNG output register 7"]
    #[inline(always)]
    pub const fn trng_dout_7(&self) -> &TrngDout7 {
        &self.trng_dout_7
    }
    #[doc = "0x280 - PKA Control Register"]
    #[inline(always)]
    pub const fn pka_ctrl(&self) -> &PkaCtrl {
        &self.pka_ctrl
    }
    #[doc = "0x300 - AES Tweak Key data 0 Register"]
    #[inline(always)]
    pub const fn aes_tkey_0(&self) -> &AesTkey0 {
        &self.aes_tkey_0
    }
    #[doc = "0x304 - AES Tweak Key data 1 Register"]
    #[inline(always)]
    pub const fn aes_tkey_1(&self) -> &AesTkey1 {
        &self.aes_tkey_1
    }
    #[doc = "0x308 - AES Tweak Key data 2 Register"]
    #[inline(always)]
    pub const fn aes_tkey_2(&self) -> &AesTkey2 {
        &self.aes_tkey_2
    }
    #[doc = "0x30c - AES Tweak Key data 3 Register"]
    #[inline(always)]
    pub const fn aes_tkey_3(&self) -> &AesTkey3 {
        &self.aes_tkey_3
    }
    #[doc = "0x310 - AES Tweak Key data 4 Register"]
    #[inline(always)]
    pub const fn aes_tkey_4(&self) -> &AesTkey4 {
        &self.aes_tkey_4
    }
    #[doc = "0x314 - AES Tweak Key data 5 Register"]
    #[inline(always)]
    pub const fn aes_tkey_5(&self) -> &AesTkey5 {
        &self.aes_tkey_5
    }
    #[doc = "0x318 - AES Tweak Key data 6 Register"]
    #[inline(always)]
    pub const fn aes_tkey_6(&self) -> &AesTkey6 {
        &self.aes_tkey_6
    }
    #[doc = "0x31c - AES Tweak Key data 7 Register"]
    #[inline(always)]
    pub const fn aes_tkey_7(&self) -> &AesTkey7 {
        &self.aes_tkey_7
    }
    #[doc = "0x320 - AES Tweak data 0 Register"]
    #[inline(always)]
    pub const fn aes_twk_0(&self) -> &AesTwk0 {
        &self.aes_twk_0
    }
    #[doc = "0x324 - AES Tweak data 1 Register"]
    #[inline(always)]
    pub const fn aes_twk_1(&self) -> &AesTwk1 {
        &self.aes_twk_1
    }
    #[doc = "0x328 - AES Tweak data 2 Register"]
    #[inline(always)]
    pub const fn aes_twk_2(&self) -> &AesTwk2 {
        &self.aes_twk_2
    }
    #[doc = "0x32c - AES Tweak data 3 Register"]
    #[inline(always)]
    pub const fn aes_twk_3(&self) -> &AesTwk3 {
        &self.aes_twk_3
    }
    #[doc = "0x330 - Key Secure Control Register"]
    #[inline(always)]
    pub const fn key_secure(&self) -> &KeySecure {
        &self.key_secure
    }
    #[doc = "0x334 - Clock Gate Control Register"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &ClkGate {
        &self.clk_gate
    }
    #[doc = "0x3f0 - Crypto Version register"]
    #[inline(always)]
    pub const fn crypto_ver(&self) -> &CryptoVer {
        &self.crypto_ver
    }
    #[doc = "0x400 - PKA input/output data"]
    #[inline(always)]
    pub const fn pka_m(&self) -> &PkaM {
        &self.pka_m
    }
    #[doc = "0x500 - PKA C value"]
    #[inline(always)]
    pub const fn pka_c(&self) -> &PkaC {
        &self.pka_c
    }
    #[doc = "0x600 - PKA modular"]
    #[inline(always)]
    pub const fn pka_n(&self) -> &PkaN {
        &self.pka_n
    }
    #[doc = "0x700 - PKA exponent"]
    #[inline(always)]
    pub const fn pka_e(&self) -> &PkaE {
        &self.pka_e
    }
}
#[doc = "INTSTS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`]
module"]
#[doc(alias = "INTSTS")]
pub type Intsts = crate::Reg<intsts::IntstsSpec>;
#[doc = "Interrupt Status Register"]
pub mod intsts;
#[doc = "INTENA (rw) register accessor: Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intena`]
module"]
#[doc(alias = "INTENA")]
pub type Intena = crate::Reg<intena::IntenaSpec>;
#[doc = "Interrupt Set Register"]
pub mod intena;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CONF (rw) register accessor: Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`]
module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "Configure Register"]
pub mod conf;
#[doc = "BRDMAS (rw) register accessor: Block Receiving DMA Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdmas::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdmas::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdmas`]
module"]
#[doc(alias = "BRDMAS")]
pub type Brdmas = crate::Reg<brdmas::BrdmasSpec>;
#[doc = "Block Receiving DMA Start Address Register"]
pub mod brdmas;
#[doc = "BTDMAS (rw) register accessor: Block Transmiting DMA Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btdmas::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btdmas::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btdmas`]
module"]
#[doc(alias = "BTDMAS")]
pub type Btdmas = crate::Reg<btdmas::BtdmasSpec>;
#[doc = "Block Transmiting DMA Start Address Register"]
pub mod btdmas;
#[doc = "BRDMAL (rw) register accessor: Block Receiving DMA Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdmal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdmal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdmal`]
module"]
#[doc(alias = "BRDMAL")]
pub type Brdmal = crate::Reg<brdmal::BrdmalSpec>;
#[doc = "Block Receiving DMA Length Register"]
pub mod brdmal;
#[doc = "HRDMAS (rw) register accessor: Hash Receiving DMA Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrdmas::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrdmas::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrdmas`]
module"]
#[doc(alias = "HRDMAS")]
pub type Hrdmas = crate::Reg<hrdmas::HrdmasSpec>;
#[doc = "Hash Receiving DMA Start Address Register"]
pub mod hrdmas;
#[doc = "HRDMAL (rw) register accessor: Hash Receiving DMA Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrdmal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrdmal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrdmal`]
module"]
#[doc(alias = "HRDMAL")]
pub type Hrdmal = crate::Reg<hrdmal::HrdmalSpec>;
#[doc = "Hash Receiving DMA Length Register"]
pub mod hrdmal;
#[doc = "AES_CTRL (rw) register accessor: AES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctrl`]
module"]
#[doc(alias = "AES_CTRL")]
pub type AesCtrl = crate::Reg<aes_ctrl::AesCtrlSpec>;
#[doc = "AES Control Register"]
pub mod aes_ctrl;
#[doc = "AES_STS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sts`]
module"]
#[doc(alias = "AES_STS")]
pub type AesSts = crate::Reg<aes_sts::AesStsSpec>;
#[doc = "Status Register"]
pub mod aes_sts;
#[doc = "AES_DIN_0 (rw) register accessor: AES Input Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_din_0`]
module"]
#[doc(alias = "AES_DIN_0")]
pub type AesDin0 = crate::Reg<aes_din_0::AesDin0Spec>;
#[doc = "AES Input Data 0 Register"]
pub mod aes_din_0;
#[doc = "AES_DIN_1 (rw) register accessor: AES Input Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_din_1`]
module"]
#[doc(alias = "AES_DIN_1")]
pub type AesDin1 = crate::Reg<aes_din_1::AesDin1Spec>;
#[doc = "AES Input Data 1 Register"]
pub mod aes_din_1;
#[doc = "AES_DIN_2 (rw) register accessor: AES Input Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_din_2`]
module"]
#[doc(alias = "AES_DIN_2")]
pub type AesDin2 = crate::Reg<aes_din_2::AesDin2Spec>;
#[doc = "AES Input Data 2 Register"]
pub mod aes_din_2;
#[doc = "AES_DIN_3 (rw) register accessor: AES Input Data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_din_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_din_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_din_3`]
module"]
#[doc(alias = "AES_DIN_3")]
pub type AesDin3 = crate::Reg<aes_din_3::AesDin3Spec>;
#[doc = "AES Input Data 3 Register"]
pub mod aes_din_3;
#[doc = "AES_DOUT_0 (r) register accessor: AES Output Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dout_0`]
module"]
#[doc(alias = "AES_DOUT_0")]
pub type AesDout0 = crate::Reg<aes_dout_0::AesDout0Spec>;
#[doc = "AES Output Data 0 Register"]
pub mod aes_dout_0;
#[doc = "AES_DOUT_1 (r) register accessor: AES Output Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dout_1`]
module"]
#[doc(alias = "AES_DOUT_1")]
pub type AesDout1 = crate::Reg<aes_dout_1::AesDout1Spec>;
#[doc = "AES Output Data 1 Register"]
pub mod aes_dout_1;
#[doc = "AES_DOUT_2 (r) register accessor: AES Output Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dout_2`]
module"]
#[doc(alias = "AES_DOUT_2")]
pub type AesDout2 = crate::Reg<aes_dout_2::AesDout2Spec>;
#[doc = "AES Output Data 2 Register"]
pub mod aes_dout_2;
#[doc = "AES_DOUT_3 (r) register accessor: AES Output Data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_dout_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_dout_3`]
module"]
#[doc(alias = "AES_DOUT_3")]
pub type AesDout3 = crate::Reg<aes_dout_3::AesDout3Spec>;
#[doc = "AES Output Data 3 Register"]
pub mod aes_dout_3;
#[doc = "AES_IV_0 (rw) register accessor: AES IV data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_0`]
module"]
#[doc(alias = "AES_IV_0")]
pub type AesIv0 = crate::Reg<aes_iv_0::AesIv0Spec>;
#[doc = "AES IV data 0 Register"]
pub mod aes_iv_0;
#[doc = "AES_IV_1 (rw) register accessor: AES IV data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_1`]
module"]
#[doc(alias = "AES_IV_1")]
pub type AesIv1 = crate::Reg<aes_iv_1::AesIv1Spec>;
#[doc = "AES IV data 1 Register"]
pub mod aes_iv_1;
#[doc = "AES_IV_2 (rw) register accessor: AES IV data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_2`]
module"]
#[doc(alias = "AES_IV_2")]
pub type AesIv2 = crate::Reg<aes_iv_2::AesIv2Spec>;
#[doc = "AES IV data 2 Register"]
pub mod aes_iv_2;
#[doc = "AES_IV_3 (rw) register accessor: AES IV data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_3`]
module"]
#[doc(alias = "AES_IV_3")]
pub type AesIv3 = crate::Reg<aes_iv_3::AesIv3Spec>;
#[doc = "AES IV data 3 Register"]
pub mod aes_iv_3;
#[doc = "AES_KEY_0 (rw) register accessor: AES Key data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_0`]
module"]
#[doc(alias = "AES_KEY_0")]
pub type AesKey0 = crate::Reg<aes_key_0::AesKey0Spec>;
#[doc = "AES Key data 0 Register"]
pub mod aes_key_0;
#[doc = "AES_KEY_1 (rw) register accessor: AES Key data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_1`]
module"]
#[doc(alias = "AES_KEY_1")]
pub type AesKey1 = crate::Reg<aes_key_1::AesKey1Spec>;
#[doc = "AES Key data 1 Register"]
pub mod aes_key_1;
#[doc = "AES_KEY_2 (rw) register accessor: AES Key data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_2`]
module"]
#[doc(alias = "AES_KEY_2")]
pub type AesKey2 = crate::Reg<aes_key_2::AesKey2Spec>;
#[doc = "AES Key data 2 Register"]
pub mod aes_key_2;
#[doc = "AES_KEY_3 (rw) register accessor: AES Key data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_3`]
module"]
#[doc(alias = "AES_KEY_3")]
pub type AesKey3 = crate::Reg<aes_key_3::AesKey3Spec>;
#[doc = "AES Key data 3 Register"]
pub mod aes_key_3;
#[doc = "AES_KEY_4 (rw) register accessor: AES Key data 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_4`]
module"]
#[doc(alias = "AES_KEY_4")]
pub type AesKey4 = crate::Reg<aes_key_4::AesKey4Spec>;
#[doc = "AES Key data 4 Register"]
pub mod aes_key_4;
#[doc = "AES_KEY_5 (rw) register accessor: AES Key data 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_5`]
module"]
#[doc(alias = "AES_KEY_5")]
pub type AesKey5 = crate::Reg<aes_key_5::AesKey5Spec>;
#[doc = "AES Key data 5 Register"]
pub mod aes_key_5;
#[doc = "AES_KEY_6 (rw) register accessor: AES Key data 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_6`]
module"]
#[doc(alias = "AES_KEY_6")]
pub type AesKey6 = crate::Reg<aes_key_6::AesKey6Spec>;
#[doc = "AES Key data 6 Register"]
pub mod aes_key_6;
#[doc = "AES_KEY_7 (rw) register accessor: AES Key data 7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_7`]
module"]
#[doc(alias = "AES_KEY_7")]
pub type AesKey7 = crate::Reg<aes_key_7::AesKey7Spec>;
#[doc = "AES Key data 7 Register"]
pub mod aes_key_7;
#[doc = "AES_CNT_0 (rw) register accessor: AES Input Counter 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cnt_0`]
module"]
#[doc(alias = "AES_CNT_0")]
pub type AesCnt0 = crate::Reg<aes_cnt_0::AesCnt0Spec>;
#[doc = "AES Input Counter 0 Register"]
pub mod aes_cnt_0;
#[doc = "AES_CNT_1 (rw) register accessor: AES Input Counter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cnt_1`]
module"]
#[doc(alias = "AES_CNT_1")]
pub type AesCnt1 = crate::Reg<aes_cnt_1::AesCnt1Spec>;
#[doc = "AES Input Counter 1 Register"]
pub mod aes_cnt_1;
#[doc = "AES_CNT_2 (rw) register accessor: AES Input Counter 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cnt_2`]
module"]
#[doc(alias = "AES_CNT_2")]
pub type AesCnt2 = crate::Reg<aes_cnt_2::AesCnt2Spec>;
#[doc = "AES Input Counter 2 Register"]
pub mod aes_cnt_2;
#[doc = "AES_CNT_3 (rw) register accessor: AES Input Counter 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_cnt_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_cnt_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cnt_3`]
module"]
#[doc(alias = "AES_CNT_3")]
pub type AesCnt3 = crate::Reg<aes_cnt_3::AesCnt3Spec>;
#[doc = "AES Input Counter 3 Register"]
pub mod aes_cnt_3;
#[doc = "TDES_CTRL (rw) register accessor: TDES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_ctrl`]
module"]
#[doc(alias = "TDES_CTRL")]
pub type TdesCtrl = crate::Reg<tdes_ctrl::TdesCtrlSpec>;
#[doc = "TDES Control Register"]
pub mod tdes_ctrl;
#[doc = "TDES_STS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_sts`]
module"]
#[doc(alias = "TDES_STS")]
pub type TdesSts = crate::Reg<tdes_sts::TdesStsSpec>;
#[doc = "Status Register"]
pub mod tdes_sts;
#[doc = "TDES_DIN_0 (rw) register accessor: TDES Input Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_din_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_din_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_din_0`]
module"]
#[doc(alias = "TDES_DIN_0")]
pub type TdesDin0 = crate::Reg<tdes_din_0::TdesDin0Spec>;
#[doc = "TDES Input Data 0 Register"]
pub mod tdes_din_0;
#[doc = "TDES_DIN_1 (rw) register accessor: TDES Input Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_din_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_din_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_din_1`]
module"]
#[doc(alias = "TDES_DIN_1")]
pub type TdesDin1 = crate::Reg<tdes_din_1::TdesDin1Spec>;
#[doc = "TDES Input Data 1 Register"]
pub mod tdes_din_1;
#[doc = "TDES_DOUT_0 (r) register accessor: TDES Output Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_dout_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_dout_0`]
module"]
#[doc(alias = "TDES_DOUT_0")]
pub type TdesDout0 = crate::Reg<tdes_dout_0::TdesDout0Spec>;
#[doc = "TDES Output Data 0 Register"]
pub mod tdes_dout_0;
#[doc = "TDES_DOUT_1 (r) register accessor: TDES Output Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_dout_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_dout_1`]
module"]
#[doc(alias = "TDES_DOUT_1")]
pub type TdesDout1 = crate::Reg<tdes_dout_1::TdesDout1Spec>;
#[doc = "TDES Output Data 1 Register"]
pub mod tdes_dout_1;
#[doc = "TDES_IV_0 (rw) register accessor: TDES IV data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_iv_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_iv_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_iv_0`]
module"]
#[doc(alias = "TDES_IV_0")]
pub type TdesIv0 = crate::Reg<tdes_iv_0::TdesIv0Spec>;
#[doc = "TDES IV data 0 Register"]
pub mod tdes_iv_0;
#[doc = "TDES_IV_1 (rw) register accessor: TDES IV data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_iv_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_iv_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_iv_1`]
module"]
#[doc(alias = "TDES_IV_1")]
pub type TdesIv1 = crate::Reg<tdes_iv_1::TdesIv1Spec>;
#[doc = "TDES IV data 1 Register"]
pub mod tdes_iv_1;
#[doc = "TDES_KEY1_0 (rw) register accessor: TDES Key1 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key1_0`]
module"]
#[doc(alias = "TDES_KEY1_0")]
pub type TdesKey1_0 = crate::Reg<tdes_key1_0::TdesKey1_0Spec>;
#[doc = "TDES Key1 data 1 Register"]
pub mod tdes_key1_0;
#[doc = "TDES_KEY1_1 (rw) register accessor: TDES Key1 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key1_1`]
module"]
#[doc(alias = "TDES_KEY1_1")]
pub type TdesKey1_1 = crate::Reg<tdes_key1_1::TdesKey1_1Spec>;
#[doc = "TDES Key1 data 1 Register"]
pub mod tdes_key1_1;
#[doc = "TDES_KEY2_0 (rw) register accessor: TDES Key2 data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key2_0`]
module"]
#[doc(alias = "TDES_KEY2_0")]
pub type TdesKey2_0 = crate::Reg<tdes_key2_0::TdesKey2_0Spec>;
#[doc = "TDES Key2 data 0 Register"]
pub mod tdes_key2_0;
#[doc = "TDES_KEY2_1 (rw) register accessor: TDES Key2 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key2_1`]
module"]
#[doc(alias = "TDES_KEY2_1")]
pub type TdesKey2_1 = crate::Reg<tdes_key2_1::TdesKey2_1Spec>;
#[doc = "TDES Key2 data 1 Register"]
pub mod tdes_key2_1;
#[doc = "TDES_KEY3_0 (rw) register accessor: TDES Key3 data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key3_0`]
module"]
#[doc(alias = "TDES_KEY3_0")]
pub type TdesKey3_0 = crate::Reg<tdes_key3_0::TdesKey3_0Spec>;
#[doc = "TDES Key3 data 0 Register"]
pub mod tdes_key3_0;
#[doc = "TDES_KEY3_1 (rw) register accessor: TDES Key3 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key3_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key3_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdes_key3_1`]
module"]
#[doc(alias = "TDES_KEY3_1")]
pub type TdesKey3_1 = crate::Reg<tdes_key3_1::TdesKey3_1Spec>;
#[doc = "TDES Key3 data 1 Register"]
pub mod tdes_key3_1;
#[doc = "HASH_CTRL (rw) register accessor: Hash Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_ctrl`]
module"]
#[doc(alias = "HASH_CTRL")]
pub type HashCtrl = crate::Reg<hash_ctrl::HashCtrlSpec>;
#[doc = "Hash Control Register"]
pub mod hash_ctrl;
#[doc = "HASH_STS (rw) register accessor: Hash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_sts`]
module"]
#[doc(alias = "HASH_STS")]
pub type HashSts = crate::Reg<hash_sts::HashStsSpec>;
#[doc = "Hash Status Register"]
pub mod hash_sts;
#[doc = "HASH_MSG_LEN (rw) register accessor: Hash Message Len\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_msg_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_msg_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_msg_len`]
module"]
#[doc(alias = "HASH_MSG_LEN")]
pub type HashMsgLen = crate::Reg<hash_msg_len::HashMsgLenSpec>;
#[doc = "Hash Message Len"]
pub mod hash_msg_len;
#[doc = "HASH_DOUT_0 (r) register accessor: Hash Result Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_0`]
module"]
#[doc(alias = "HASH_DOUT_0")]
pub type HashDout0 = crate::Reg<hash_dout_0::HashDout0Spec>;
#[doc = "Hash Result Register 0"]
pub mod hash_dout_0;
#[doc = "HASH_DOUT_1 (r) register accessor: Hash Result Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_1`]
module"]
#[doc(alias = "HASH_DOUT_1")]
pub type HashDout1 = crate::Reg<hash_dout_1::HashDout1Spec>;
#[doc = "Hash Result Register 1"]
pub mod hash_dout_1;
#[doc = "HASH_DOUT_2 (r) register accessor: Hash Result Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_2`]
module"]
#[doc(alias = "HASH_DOUT_2")]
pub type HashDout2 = crate::Reg<hash_dout_2::HashDout2Spec>;
#[doc = "Hash Result Register 2"]
pub mod hash_dout_2;
#[doc = "HASH_DOUT_3 (r) register accessor: Hash Result Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_3`]
module"]
#[doc(alias = "HASH_DOUT_3")]
pub type HashDout3 = crate::Reg<hash_dout_3::HashDout3Spec>;
#[doc = "Hash Result Register 3"]
pub mod hash_dout_3;
#[doc = "HASH_DOUT_4 (r) register accessor: Hash Result Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_4`]
module"]
#[doc(alias = "HASH_DOUT_4")]
pub type HashDout4 = crate::Reg<hash_dout_4::HashDout4Spec>;
#[doc = "Hash Result Register 4"]
pub mod hash_dout_4;
#[doc = "HASH_DOUT_5 (r) register accessor: Hash Result Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_5`]
module"]
#[doc(alias = "HASH_DOUT_5")]
pub type HashDout5 = crate::Reg<hash_dout_5::HashDout5Spec>;
#[doc = "Hash Result Register 5"]
pub mod hash_dout_5;
#[doc = "HASH_DOUT_6 (r) register accessor: Hash Result Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_6`]
module"]
#[doc(alias = "HASH_DOUT_6")]
pub type HashDout6 = crate::Reg<hash_dout_6::HashDout6Spec>;
#[doc = "Hash Result Register 6"]
pub mod hash_dout_6;
#[doc = "HASH_DOUT_7 (r) register accessor: Hash Result Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_dout_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_dout_7`]
module"]
#[doc(alias = "HASH_DOUT_7")]
pub type HashDout7 = crate::Reg<hash_dout_7::HashDout7Spec>;
#[doc = "Hash Result Register 7"]
pub mod hash_dout_7;
#[doc = "HASH_SEED_0 (rw) register accessor: PRNG Seed/HMAC Key Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_seed_0`]
module"]
#[doc(alias = "HASH_SEED_0")]
pub type HashSeed0 = crate::Reg<hash_seed_0::HashSeed0Spec>;
#[doc = "PRNG Seed/HMAC Key Register 0"]
pub mod hash_seed_0;
#[doc = "HASH_SEED_1 (rw) register accessor: PRNG Seed/HMAC Key Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_seed_1`]
module"]
#[doc(alias = "HASH_SEED_1")]
pub type HashSeed1 = crate::Reg<hash_seed_1::HashSeed1Spec>;
#[doc = "PRNG Seed/HMAC Key Register 1"]
pub mod hash_seed_1;
#[doc = "HASH_SEED_2 (rw) register accessor: PRNG Seed/HMAC Key Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_seed_2`]
module"]
#[doc(alias = "HASH_SEED_2")]
pub type HashSeed2 = crate::Reg<hash_seed_2::HashSeed2Spec>;
#[doc = "PRNG Seed/HMAC Key Register 2"]
pub mod hash_seed_2;
#[doc = "HASH_SEED_3 (rw) register accessor: PRNG Seed/HMAC Key Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_seed_3`]
module"]
#[doc(alias = "HASH_SEED_3")]
pub type HashSeed3 = crate::Reg<hash_seed_3::HashSeed3Spec>;
#[doc = "PRNG Seed/HMAC Key Register 3"]
pub mod hash_seed_3;
#[doc = "HASH_SEED_4 (rw) register accessor: PRNG Seed/HMAC Key Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_seed_4`]
module"]
#[doc(alias = "HASH_SEED_4")]
pub type HashSeed4 = crate::Reg<hash_seed_4::HashSeed4Spec>;
#[doc = "PRNG Seed/HMAC Key Register 4"]
pub mod hash_seed_4;
#[doc = "TRNG_CTRL (rw) register accessor: TRNG Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_ctrl`]
module"]
#[doc(alias = "TRNG_CTRL")]
pub type TrngCtrl = crate::Reg<trng_ctrl::TrngCtrlSpec>;
#[doc = "TRNG Control register"]
pub mod trng_ctrl;
#[doc = "TRNG_DOUT_0 (rw) register accessor: TRNG output register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_0`]
module"]
#[doc(alias = "TRNG_DOUT_0")]
pub type TrngDout0 = crate::Reg<trng_dout_0::TrngDout0Spec>;
#[doc = "TRNG output register 0"]
pub mod trng_dout_0;
#[doc = "TRNG_DOUT_1 (rw) register accessor: TRNG output register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_1`]
module"]
#[doc(alias = "TRNG_DOUT_1")]
pub type TrngDout1 = crate::Reg<trng_dout_1::TrngDout1Spec>;
#[doc = "TRNG output register 1"]
pub mod trng_dout_1;
#[doc = "TRNG_DOUT_2 (rw) register accessor: TRNG output register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_2`]
module"]
#[doc(alias = "TRNG_DOUT_2")]
pub type TrngDout2 = crate::Reg<trng_dout_2::TrngDout2Spec>;
#[doc = "TRNG output register 2"]
pub mod trng_dout_2;
#[doc = "TRNG_DOUT_3 (rw) register accessor: TRNG output register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_3`]
module"]
#[doc(alias = "TRNG_DOUT_3")]
pub type TrngDout3 = crate::Reg<trng_dout_3::TrngDout3Spec>;
#[doc = "TRNG output register 3"]
pub mod trng_dout_3;
#[doc = "TRNG_DOUT_4 (rw) register accessor: TRNG output register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_4`]
module"]
#[doc(alias = "TRNG_DOUT_4")]
pub type TrngDout4 = crate::Reg<trng_dout_4::TrngDout4Spec>;
#[doc = "TRNG output register 4"]
pub mod trng_dout_4;
#[doc = "TRNG_DOUT_5 (rw) register accessor: TRNG output register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_5`]
module"]
#[doc(alias = "TRNG_DOUT_5")]
pub type TrngDout5 = crate::Reg<trng_dout_5::TrngDout5Spec>;
#[doc = "TRNG output register 5"]
pub mod trng_dout_5;
#[doc = "TRNG_DOUT_6 (rw) register accessor: TRNG output register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_6`]
module"]
#[doc(alias = "TRNG_DOUT_6")]
pub type TrngDout6 = crate::Reg<trng_dout_6::TrngDout6Spec>;
#[doc = "TRNG output register 6"]
pub mod trng_dout_6;
#[doc = "TRNG_DOUT_7 (rw) register accessor: TRNG output register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_dout_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_dout_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_dout_7`]
module"]
#[doc(alias = "TRNG_DOUT_7")]
pub type TrngDout7 = crate::Reg<trng_dout_7::TrngDout7Spec>;
#[doc = "TRNG output register 7"]
pub mod trng_dout_7;
#[doc = "PKA_CTRL (rw) register accessor: PKA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_ctrl`]
module"]
#[doc(alias = "PKA_CTRL")]
pub type PkaCtrl = crate::Reg<pka_ctrl::PkaCtrlSpec>;
#[doc = "PKA Control Register"]
pub mod pka_ctrl;
#[doc = "AES_TKEY_0 (rw) register accessor: AES Tweak Key data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_0`]
module"]
#[doc(alias = "AES_TKEY_0")]
pub type AesTkey0 = crate::Reg<aes_tkey_0::AesTkey0Spec>;
#[doc = "AES Tweak Key data 0 Register"]
pub mod aes_tkey_0;
#[doc = "AES_TKEY_1 (rw) register accessor: AES Tweak Key data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_1`]
module"]
#[doc(alias = "AES_TKEY_1")]
pub type AesTkey1 = crate::Reg<aes_tkey_1::AesTkey1Spec>;
#[doc = "AES Tweak Key data 1 Register"]
pub mod aes_tkey_1;
#[doc = "AES_TKEY_2 (rw) register accessor: AES Tweak Key data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_2`]
module"]
#[doc(alias = "AES_TKEY_2")]
pub type AesTkey2 = crate::Reg<aes_tkey_2::AesTkey2Spec>;
#[doc = "AES Tweak Key data 2 Register"]
pub mod aes_tkey_2;
#[doc = "AES_TKEY_3 (rw) register accessor: AES Tweak Key data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_3`]
module"]
#[doc(alias = "AES_TKEY_3")]
pub type AesTkey3 = crate::Reg<aes_tkey_3::AesTkey3Spec>;
#[doc = "AES Tweak Key data 3 Register"]
pub mod aes_tkey_3;
#[doc = "AES_TKEY_4 (rw) register accessor: AES Tweak Key data 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_4`]
module"]
#[doc(alias = "AES_TKEY_4")]
pub type AesTkey4 = crate::Reg<aes_tkey_4::AesTkey4Spec>;
#[doc = "AES Tweak Key data 4 Register"]
pub mod aes_tkey_4;
#[doc = "AES_TKEY_5 (rw) register accessor: AES Tweak Key data 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_5`]
module"]
#[doc(alias = "AES_TKEY_5")]
pub type AesTkey5 = crate::Reg<aes_tkey_5::AesTkey5Spec>;
#[doc = "AES Tweak Key data 5 Register"]
pub mod aes_tkey_5;
#[doc = "AES_TKEY_6 (rw) register accessor: AES Tweak Key data 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_6`]
module"]
#[doc(alias = "AES_TKEY_6")]
pub type AesTkey6 = crate::Reg<aes_tkey_6::AesTkey6Spec>;
#[doc = "AES Tweak Key data 6 Register"]
pub mod aes_tkey_6;
#[doc = "AES_TKEY_7 (rw) register accessor: AES Tweak Key data 7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tkey_7`]
module"]
#[doc(alias = "AES_TKEY_7")]
pub type AesTkey7 = crate::Reg<aes_tkey_7::AesTkey7Spec>;
#[doc = "AES Tweak Key data 7 Register"]
pub mod aes_tkey_7;
#[doc = "AES_TWK_0 (rw) register accessor: AES Tweak data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_twk_0`]
module"]
#[doc(alias = "AES_TWK_0")]
pub type AesTwk0 = crate::Reg<aes_twk_0::AesTwk0Spec>;
#[doc = "AES Tweak data 0 Register"]
pub mod aes_twk_0;
#[doc = "AES_TWK_1 (rw) register accessor: AES Tweak data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_twk_1`]
module"]
#[doc(alias = "AES_TWK_1")]
pub type AesTwk1 = crate::Reg<aes_twk_1::AesTwk1Spec>;
#[doc = "AES Tweak data 1 Register"]
pub mod aes_twk_1;
#[doc = "AES_TWK_2 (rw) register accessor: AES Tweak data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_twk_2`]
module"]
#[doc(alias = "AES_TWK_2")]
pub type AesTwk2 = crate::Reg<aes_twk_2::AesTwk2Spec>;
#[doc = "AES Tweak data 2 Register"]
pub mod aes_twk_2;
#[doc = "AES_TWK_3 (rw) register accessor: AES Tweak data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_twk_3`]
module"]
#[doc(alias = "AES_TWK_3")]
pub type AesTwk3 = crate::Reg<aes_twk_3::AesTwk3Spec>;
#[doc = "AES Tweak data 3 Register"]
pub mod aes_twk_3;
#[doc = "KEY_SECURE (rw) register accessor: Key Secure Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_secure::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_secure::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_secure`]
module"]
#[doc(alias = "KEY_SECURE")]
pub type KeySecure = crate::Reg<key_secure::KeySecureSpec>;
#[doc = "Key Secure Control Register"]
pub mod key_secure;
#[doc = "CLK_GATE (rw) register accessor: Clock Gate Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`]
module"]
#[doc(alias = "CLK_GATE")]
pub type ClkGate = crate::Reg<clk_gate::ClkGateSpec>;
#[doc = "Clock Gate Control Register"]
pub mod clk_gate;
#[doc = "CRYPTO_VER (rw) register accessor: Crypto Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_ver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_ver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_ver`]
module"]
#[doc(alias = "CRYPTO_VER")]
pub type CryptoVer = crate::Reg<crypto_ver::CryptoVerSpec>;
#[doc = "Crypto Version register"]
pub mod crypto_ver;
#[doc = "PKA_M (rw) register accessor: PKA input/output data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_m`]
module"]
#[doc(alias = "PKA_M")]
pub type PkaM = crate::Reg<pka_m::PkaMSpec>;
#[doc = "PKA input/output data"]
pub mod pka_m;
#[doc = "PKA_C (rw) register accessor: PKA C value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_c`]
module"]
#[doc(alias = "PKA_C")]
pub type PkaC = crate::Reg<pka_c::PkaCSpec>;
#[doc = "PKA C value"]
pub mod pka_c;
#[doc = "PKA_N (rw) register accessor: PKA modular\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_n`]
module"]
#[doc(alias = "PKA_N")]
pub type PkaN = crate::Reg<pka_n::PkaNSpec>;
#[doc = "PKA modular"]
pub mod pka_n;
#[doc = "PKA_E (rw) register accessor: PKA exponent\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_e`]
module"]
#[doc(alias = "PKA_E")]
pub type PkaE = crate::Reg<pka_e::PkaESpec>;
#[doc = "PKA exponent"]
pub mod pka_e;
