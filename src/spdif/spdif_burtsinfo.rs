#[doc = "Register `SPDIF_BURTSINFO` reader"]
pub type R = crate::R<SpdifBurtsinfoSpec>;
#[doc = "Register `SPDIF_BURTSINFO` writer"]
pub type W = crate::W<SpdifBurtsinfoSpec>;
#[doc = "Data type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    #[doc = "0: MAT others: reserved"]
    B0000000 = 0,
    #[doc = "1: MAT others: reserved"]
    B0000001 = 1,
    #[doc = "3: MAT others: reserved"]
    B0000011 = 3,
    #[doc = "4: MAT others: reserved"]
    B0000100 = 4,
    #[doc = "5: MAT others: reserved"]
    B0000101 = 5,
    #[doc = "6: MAT others: reserved"]
    B0000110 = 6,
    #[doc = "7: MAT others: reserved"]
    B0000111 = 7,
    #[doc = "8: MAT others: reserved"]
    B0001000 = 8,
    #[doc = "9: MAT others: reserved"]
    B0001001 = 9,
    #[doc = "10: MAT others: reserved"]
    B0001010 = 10,
    #[doc = "11: MAT others: reserved"]
    B0001011 = 11,
    #[doc = "12: MAT others: reserved"]
    B0001100 = 12,
    #[doc = "13: MAT others: reserved"]
    B0001101 = 13,
    #[doc = "14: MAT others: reserved"]
    B0001110 = 14,
    #[doc = "15: MAT others: reserved"]
    B0001111 = 15,
    #[doc = "16: MAT others: reserved"]
    B0010000 = 16,
    #[doc = "17: MAT others: reserved"]
    B0010001 = 17,
    #[doc = "18: MAT others: reserved"]
    B0010010 = 18,
    #[doc = "50: MAT others: reserved"]
    B0110010 = 50,
    #[doc = "82: MAT others: reserved"]
    B1010010 = 82,
    #[doc = "114: MAT others: reserved"]
    B1110010 = 114,
    #[doc = "19: MAT others: reserved"]
    B0010011 = 19,
    #[doc = "51: MAT others: reserved"]
    B0110011 = 51,
    #[doc = "83: MAT others: reserved"]
    B1010011 = 83,
    #[doc = "115: MAT others: reserved"]
    B1110011 = 115,
    #[doc = "20: MAT others: reserved"]
    B0010100 = 20,
    #[doc = "52: MAT others: reserved"]
    B0110100 = 52,
    #[doc = "84: MAT others: reserved"]
    B1010100 = 84,
    #[doc = "116: MAT others: reserved"]
    B1110100 = 116,
    #[doc = "21: MAT others: reserved"]
    B0010101 = 21,
    #[doc = "22: MAT others: reserved"]
    B0010110 = 22,
}
impl From<Datatype> for u8 {
    #[inline(always)]
    fn from(variant: Datatype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datatype {
    type Ux = u8;
}
#[doc = "Field `DATATYPE` reader - Data type"]
pub type DatatypeR = crate::FieldReader<Datatype>;
impl DatatypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datatype> {
        match self.bits {
            0 => Some(Datatype::B0000000),
            1 => Some(Datatype::B0000001),
            3 => Some(Datatype::B0000011),
            4 => Some(Datatype::B0000100),
            5 => Some(Datatype::B0000101),
            6 => Some(Datatype::B0000110),
            7 => Some(Datatype::B0000111),
            8 => Some(Datatype::B0001000),
            9 => Some(Datatype::B0001001),
            10 => Some(Datatype::B0001010),
            11 => Some(Datatype::B0001011),
            12 => Some(Datatype::B0001100),
            13 => Some(Datatype::B0001101),
            14 => Some(Datatype::B0001110),
            15 => Some(Datatype::B0001111),
            16 => Some(Datatype::B0010000),
            17 => Some(Datatype::B0010001),
            18 => Some(Datatype::B0010010),
            50 => Some(Datatype::B0110010),
            82 => Some(Datatype::B1010010),
            114 => Some(Datatype::B1110010),
            19 => Some(Datatype::B0010011),
            51 => Some(Datatype::B0110011),
            83 => Some(Datatype::B1010011),
            115 => Some(Datatype::B1110011),
            20 => Some(Datatype::B0010100),
            52 => Some(Datatype::B0110100),
            84 => Some(Datatype::B1010100),
            116 => Some(Datatype::B1110100),
            21 => Some(Datatype::B0010101),
            22 => Some(Datatype::B0010110),
            _ => None,
        }
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000000(&self) -> bool {
        *self == Datatype::B0000000
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000001(&self) -> bool {
        *self == Datatype::B0000001
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000011(&self) -> bool {
        *self == Datatype::B0000011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000100(&self) -> bool {
        *self == Datatype::B0000100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000101(&self) -> bool {
        *self == Datatype::B0000101
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000110(&self) -> bool {
        *self == Datatype::B0000110
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0000111(&self) -> bool {
        *self == Datatype::B0000111
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001000(&self) -> bool {
        *self == Datatype::B0001000
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001001(&self) -> bool {
        *self == Datatype::B0001001
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001010(&self) -> bool {
        *self == Datatype::B0001010
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001011(&self) -> bool {
        *self == Datatype::B0001011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001100(&self) -> bool {
        *self == Datatype::B0001100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001101(&self) -> bool {
        *self == Datatype::B0001101
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001110(&self) -> bool {
        *self == Datatype::B0001110
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0001111(&self) -> bool {
        *self == Datatype::B0001111
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010000(&self) -> bool {
        *self == Datatype::B0010000
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010001(&self) -> bool {
        *self == Datatype::B0010001
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010010(&self) -> bool {
        *self == Datatype::B0010010
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0110010(&self) -> bool {
        *self == Datatype::B0110010
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1010010(&self) -> bool {
        *self == Datatype::B1010010
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1110010(&self) -> bool {
        *self == Datatype::B1110010
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010011(&self) -> bool {
        *self == Datatype::B0010011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0110011(&self) -> bool {
        *self == Datatype::B0110011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1010011(&self) -> bool {
        *self == Datatype::B1010011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1110011(&self) -> bool {
        *self == Datatype::B1110011
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010100(&self) -> bool {
        *self == Datatype::B0010100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0110100(&self) -> bool {
        *self == Datatype::B0110100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1010100(&self) -> bool {
        *self == Datatype::B1010100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b1110100(&self) -> bool {
        *self == Datatype::B1110100
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010101(&self) -> bool {
        *self == Datatype::B0010101
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn is_b0010110(&self) -> bool {
        *self == Datatype::B0010110
    }
}
#[doc = "Field `DATATYPE` writer - Data type"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 7, Datatype>;
impl<'a, REG> DatatypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000000(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000000)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000001(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000001)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000101(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000101)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000110(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000110)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0000111(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0000111)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001000(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001000)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001001(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001001)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001010(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001010)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001101(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001101)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001110(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001110)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0001111(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0001111)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010000(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010000)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010001(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010001)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010010(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010010)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0110010(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0110010)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1010010(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1010010)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1110010(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1110010)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0110011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0110011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1010011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1010011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1110011(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1110011)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0110100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0110100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1010100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1010100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b1110100(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B1110100)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010101(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010101)
    }
    #[doc = "MAT others: reserved"]
    #[inline(always)]
    pub fn b0010110(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::B0010110)
    }
}
#[doc = "Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errflag {
    #[doc = "0: indicates that the burst-payload may contain errors"]
    B0 = 0,
    #[doc = "1: indicates that the burst-payload may contain errors"]
    B1 = 1,
}
impl From<Errflag> for bool {
    #[inline(always)]
    fn from(variant: Errflag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRFLAG` reader - Error Flag"]
pub type ErrflagR = crate::BitReader<Errflag>;
impl ErrflagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errflag {
        match self.bits {
            false => Errflag::B0,
            true => Errflag::B1,
        }
    }
    #[doc = "indicates that the burst-payload may contain errors"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Errflag::B0
    }
    #[doc = "indicates that the burst-payload may contain errors"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Errflag::B1
    }
}
#[doc = "Field `ERRFLAG` writer - Error Flag"]
pub type ErrflagW<'a, REG> = crate::BitWriter<'a, REG, Errflag>;
impl<'a, REG> ErrflagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "indicates that the burst-payload may contain errors"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Errflag::B0)
    }
    #[doc = "indicates that the burst-payload may contain errors"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Errflag::B1)
    }
}
#[doc = "Field `DATAINFO` reader - Data-type-dependent info This field gives the data-type-dependent info"]
pub type DatainfoR = crate::FieldReader;
#[doc = "Field `DATAINFO` writer - Data-type-dependent info This field gives the data-type-dependent info"]
pub type DatainfoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BSNUM` reader - Bitstream Number This field indicates the bitstream number. Usually the bitstream number is 0."]
pub type BsnumR = crate::FieldReader;
#[doc = "Field `BSNUM` writer - Bitstream Number This field indicates the bitstream number. Usually the bitstream number is 0."]
pub type BsnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD` reader - pd Preamble Pd for non-linear pcm, indicating the length of burst payload in unit of bytes or bits."]
pub type PdR = crate::FieldReader<u16>;
#[doc = "Field `PD` writer - pd Preamble Pd for non-linear pcm, indicating the length of burst payload in unit of bytes or bits."]
pub type PdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - Data type"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Error Flag"]
    #[inline(always)]
    pub fn errflag(&self) -> ErrflagR {
        ErrflagR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Data-type-dependent info This field gives the data-type-dependent info"]
    #[inline(always)]
    pub fn datainfo(&self) -> DatainfoR {
        DatainfoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Bitstream Number This field indicates the bitstream number. Usually the bitstream number is 0."]
    #[inline(always)]
    pub fn bsnum(&self) -> BsnumR {
        BsnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - pd Preamble Pd for non-linear pcm, indicating the length of burst payload in unit of bytes or bits."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Data type"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DatatypeW<SpdifBurtsinfoSpec> {
        DatatypeW::new(self, 0)
    }
    #[doc = "Bit 7 - Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn errflag(&mut self) -> ErrflagW<SpdifBurtsinfoSpec> {
        ErrflagW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Data-type-dependent info This field gives the data-type-dependent info"]
    #[inline(always)]
    #[must_use]
    pub fn datainfo(&mut self) -> DatainfoW<SpdifBurtsinfoSpec> {
        DatainfoW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Bitstream Number This field indicates the bitstream number. Usually the bitstream number is 0."]
    #[inline(always)]
    #[must_use]
    pub fn bsnum(&mut self) -> BsnumW<SpdifBurtsinfoSpec> {
        BsnumW::new(self, 13)
    }
    #[doc = "Bits 16:31 - pd Preamble Pd for non-linear pcm, indicating the length of burst payload in unit of bytes or bits."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<SpdifBurtsinfoSpec> {
        PdW::new(self, 16)
    }
}
#[doc = "Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_burtsinfo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_burtsinfo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifBurtsinfoSpec;
impl crate::RegisterSpec for SpdifBurtsinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_burtsinfo::R`](R) reader structure"]
impl crate::Readable for SpdifBurtsinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_burtsinfo::W`](W) writer structure"]
impl crate::Writable for SpdifBurtsinfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_BURTSINFO to value 0"]
impl crate::Resettable for SpdifBurtsinfoSpec {
    const RESET_VALUE: u32 = 0;
}
