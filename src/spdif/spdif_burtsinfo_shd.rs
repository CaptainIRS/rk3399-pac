#[doc = "Register `SPDIF_BURTSINFO_SHD` reader"]
pub type R = crate::R<SpdifBurtsinfoShdSpec>;
#[doc = "Data type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    #[doc = "0: null data"]
    B0000000 = 0,
    #[doc = "1: AC-3 data"]
    B0000001 = 1,
    #[doc = "3: Pause data"]
    B0000011 = 3,
    #[doc = "4: MPEG-1 layer 1 data"]
    B0000100 = 4,
    #[doc = "5: MPEG-1 layer 2 or 3 data or MPEG-2 without extension"]
    B0000101 = 5,
    #[doc = "6: MPEG-2 data with extension"]
    B0000110 = 6,
    #[doc = "7: MPEG-2 AAC"]
    B0000111 = 7,
    #[doc = "8: MPEG-2, layer-1 low sampling frequency"]
    B0001000 = 8,
    #[doc = "9: MPEG-2, layer-2 low sampling frequency"]
    B0001001 = 9,
    #[doc = "10: MPEG-2, layer-3 low sampling frequency"]
    B0001010 = 10,
    #[doc = "11: DTS type I"]
    B0001011 = 11,
    #[doc = "12: DTS type II"]
    B0001100 = 12,
    #[doc = "13: DTS type III"]
    B0001101 = 13,
    #[doc = "14: ATRAC"]
    B0001110 = 14,
    #[doc = "15: ATRAC 2/3"]
    B0001111 = 15,
    #[doc = "16: ATRAC-X"]
    B0010000 = 16,
    #[doc = "17: DTS type IV"]
    B0010001 = 17,
    #[doc = "18: WMA professional type I"]
    B0010010 = 18,
    #[doc = "50: WMA professional type II"]
    B0110010 = 50,
    #[doc = "82: WMA professional type III"]
    B1010010 = 82,
    #[doc = "114: WMA professional type IV"]
    B1110010 = 114,
    #[doc = "19: MPEG-2 AAC low sampling frequency"]
    B0010011 = 19,
    #[doc = "51: MPEG-2 AAC low sampling frequency"]
    B0110011 = 51,
    #[doc = "83: MPEG-2 AAC low sampling frequency"]
    B1010011 = 83,
    #[doc = "115: MPEG-2 AAC low sampling frequency"]
    B1110011 = 115,
    #[doc = "20: MPEG-4 AAC"]
    B0010100 = 20,
    #[doc = "52: MPEG-4 AAC"]
    B0110100 = 52,
    #[doc = "84: MPEG-4 AAC"]
    B1010100 = 84,
    #[doc = "116: MPEG-4 AAC"]
    B1110100 = 116,
    #[doc = "21: Enhanced AC-3"]
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
    #[doc = "null data"]
    #[inline(always)]
    pub fn is_b0000000(&self) -> bool {
        *self == Datatype::B0000000
    }
    #[doc = "AC-3 data"]
    #[inline(always)]
    pub fn is_b0000001(&self) -> bool {
        *self == Datatype::B0000001
    }
    #[doc = "Pause data"]
    #[inline(always)]
    pub fn is_b0000011(&self) -> bool {
        *self == Datatype::B0000011
    }
    #[doc = "MPEG-1 layer 1 data"]
    #[inline(always)]
    pub fn is_b0000100(&self) -> bool {
        *self == Datatype::B0000100
    }
    #[doc = "MPEG-1 layer 2 or 3 data or MPEG-2 without extension"]
    #[inline(always)]
    pub fn is_b0000101(&self) -> bool {
        *self == Datatype::B0000101
    }
    #[doc = "MPEG-2 data with extension"]
    #[inline(always)]
    pub fn is_b0000110(&self) -> bool {
        *self == Datatype::B0000110
    }
    #[doc = "MPEG-2 AAC"]
    #[inline(always)]
    pub fn is_b0000111(&self) -> bool {
        *self == Datatype::B0000111
    }
    #[doc = "MPEG-2, layer-1 low sampling frequency"]
    #[inline(always)]
    pub fn is_b0001000(&self) -> bool {
        *self == Datatype::B0001000
    }
    #[doc = "MPEG-2, layer-2 low sampling frequency"]
    #[inline(always)]
    pub fn is_b0001001(&self) -> bool {
        *self == Datatype::B0001001
    }
    #[doc = "MPEG-2, layer-3 low sampling frequency"]
    #[inline(always)]
    pub fn is_b0001010(&self) -> bool {
        *self == Datatype::B0001010
    }
    #[doc = "DTS type I"]
    #[inline(always)]
    pub fn is_b0001011(&self) -> bool {
        *self == Datatype::B0001011
    }
    #[doc = "DTS type II"]
    #[inline(always)]
    pub fn is_b0001100(&self) -> bool {
        *self == Datatype::B0001100
    }
    #[doc = "DTS type III"]
    #[inline(always)]
    pub fn is_b0001101(&self) -> bool {
        *self == Datatype::B0001101
    }
    #[doc = "ATRAC"]
    #[inline(always)]
    pub fn is_b0001110(&self) -> bool {
        *self == Datatype::B0001110
    }
    #[doc = "ATRAC 2/3"]
    #[inline(always)]
    pub fn is_b0001111(&self) -> bool {
        *self == Datatype::B0001111
    }
    #[doc = "ATRAC-X"]
    #[inline(always)]
    pub fn is_b0010000(&self) -> bool {
        *self == Datatype::B0010000
    }
    #[doc = "DTS type IV"]
    #[inline(always)]
    pub fn is_b0010001(&self) -> bool {
        *self == Datatype::B0010001
    }
    #[doc = "WMA professional type I"]
    #[inline(always)]
    pub fn is_b0010010(&self) -> bool {
        *self == Datatype::B0010010
    }
    #[doc = "WMA professional type II"]
    #[inline(always)]
    pub fn is_b0110010(&self) -> bool {
        *self == Datatype::B0110010
    }
    #[doc = "WMA professional type III"]
    #[inline(always)]
    pub fn is_b1010010(&self) -> bool {
        *self == Datatype::B1010010
    }
    #[doc = "WMA professional type IV"]
    #[inline(always)]
    pub fn is_b1110010(&self) -> bool {
        *self == Datatype::B1110010
    }
    #[doc = "MPEG-2 AAC low sampling frequency"]
    #[inline(always)]
    pub fn is_b0010011(&self) -> bool {
        *self == Datatype::B0010011
    }
    #[doc = "MPEG-2 AAC low sampling frequency"]
    #[inline(always)]
    pub fn is_b0110011(&self) -> bool {
        *self == Datatype::B0110011
    }
    #[doc = "MPEG-2 AAC low sampling frequency"]
    #[inline(always)]
    pub fn is_b1010011(&self) -> bool {
        *self == Datatype::B1010011
    }
    #[doc = "MPEG-2 AAC low sampling frequency"]
    #[inline(always)]
    pub fn is_b1110011(&self) -> bool {
        *self == Datatype::B1110011
    }
    #[doc = "MPEG-4 AAC"]
    #[inline(always)]
    pub fn is_b0010100(&self) -> bool {
        *self == Datatype::B0010100
    }
    #[doc = "MPEG-4 AAC"]
    #[inline(always)]
    pub fn is_b0110100(&self) -> bool {
        *self == Datatype::B0110100
    }
    #[doc = "MPEG-4 AAC"]
    #[inline(always)]
    pub fn is_b1010100(&self) -> bool {
        *self == Datatype::B1010100
    }
    #[doc = "MPEG-4 AAC"]
    #[inline(always)]
    pub fn is_b1110100(&self) -> bool {
        *self == Datatype::B1110100
    }
    #[doc = "Enhanced AC-3"]
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
#[doc = "Error Flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errflag {
    #[doc = "0: indicates a valid burst-payload"]
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
    #[doc = "indicates a valid burst-payload"]
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
#[doc = "Field `DATAINFO` reader - Data-type-dependent info\n\nThis field gives the data-type-dependent info"]
pub type DatainfoR = crate::FieldReader;
#[doc = "Field `BSNUM` reader - Bitstream Number\n\nThis field indicates the bitstream number. Usually the birstream\n\nnumber is 0."]
pub type BsnumR = crate::FieldReader;
#[doc = "Field `PD` reader - pd\n\nPreamble Pd for non-linear pcm, indicating the length of burst\n\npayload in unit of bytes or bits."]
pub type PdR = crate::FieldReader<u16>;
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
    #[doc = "Bits 8:12 - Data-type-dependent info\n\nThis field gives the data-type-dependent info"]
    #[inline(always)]
    pub fn datainfo(&self) -> DatainfoR {
        DatainfoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Bitstream Number\n\nThis field indicates the bitstream number. Usually the birstream\n\nnumber is 0."]
    #[inline(always)]
    pub fn bsnum(&self) -> BsnumR {
        BsnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - pd\n\nPreamble Pd for non-linear pcm, indicating the length of burst\n\npayload in unit of bytes or bits."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Shadow Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_burtsinfo_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifBurtsinfoShdSpec;
impl crate::RegisterSpec for SpdifBurtsinfoShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_burtsinfo_shd::R`](R) reader structure"]
impl crate::Readable for SpdifBurtsinfoShdSpec {}
#[doc = "`reset()` method sets SPDIF_BURTSINFO_SHD to value 0"]
impl crate::Resettable for SpdifBurtsinfoShdSpec {
    const RESET_VALUE: u32 = 0;
}
