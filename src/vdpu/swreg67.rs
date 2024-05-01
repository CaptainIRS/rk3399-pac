#[doc = "Register `SWREG67` reader"]
pub type R = crate::R<Swreg67Spec>;
#[doc = "tile mode support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TileModeSel {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: 8x4 support 2,3: no used"]
    B1 = 1,
}
impl From<TileModeSel> for u8 {
    #[inline(always)]
    fn from(variant: TileModeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TileModeSel {
    type Ux = u8;
}
#[doc = "Field `TILE_MODE_SEL` reader - tile mode support"]
pub type TileModeSelR = crate::FieldReader<TileModeSel>;
impl TileModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TileModeSel> {
        match self.bits {
            0 => Some(TileModeSel::B0),
            1 => Some(TileModeSel::B1),
            _ => None,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TileModeSel::B0
    }
    #[doc = "8x4 support 2,3: no used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TileModeSel::B1
    }
}
#[doc = "Field `MVC_ALLOW_FLAG` reader - mvc support"]
pub type MvcAllowFlagR = crate::FieldReader;
#[doc = "Field `VP7_ALLOW_FLAG` reader - vp7 support"]
pub type Vp7AllowFlagR = crate::BitReader;
#[doc = "rom implementation type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RomImpType {
    #[doc = "0: from actual ROM units"]
    B0 = 0,
    #[doc = "1: from RTL"]
    B1 = 1,
}
impl From<RomImpType> for bool {
    #[inline(always)]
    fn from(variant: RomImpType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_IMP_TYPE` reader - rom implementation type"]
pub type RomImpTypeR = crate::BitReader<RomImpType>;
impl RomImpTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RomImpType {
        match self.bits {
            false => RomImpType::B0,
            true => RomImpType::B1,
        }
    }
    #[doc = "from actual ROM units"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RomImpType::B0
    }
    #[doc = "from RTL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RomImpType::B1
    }
}
#[doc = "Field `RV_ALLOW_FLAG` reader - rv_allow_flag"]
pub type RvAllowFlagR = crate::FieldReader;
#[doc = "Field `REFBUF2_ALLOW_FLAG` reader - refbuffer2 support"]
pub type Refbuf2AllowFlagR = crate::BitReader;
#[doc = "ref buffer support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefbufAllowFlag {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<RefbufAllowFlag> for bool {
    #[inline(always)]
    fn from(variant: RefbufAllowFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBUF_ALLOW_FLAG` reader - ref buffer support"]
pub type RefbufAllowFlagR = crate::BitReader<RefbufAllowFlag>;
impl RefbufAllowFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefbufAllowFlag {
        match self.bits {
            false => RefbufAllowFlag::B0,
            true => RefbufAllowFlag::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RefbufAllowFlag::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RefbufAllowFlag::B1
    }
}
#[doc = "Field `JPEG_ALLOW_FLAG` reader - JPEG sampling support\n\n16Mpixel~67Mpixel be sampled and supported by 411 and 444"]
pub type JpegAllowFlagR = crate::BitReader;
impl R {
    #[doc = "Bits 17:18 - tile mode support"]
    #[inline(always)]
    pub fn tile_mode_sel(&self) -> TileModeSelR {
        TileModeSelR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:21 - mvc support"]
    #[inline(always)]
    pub fn mvc_allow_flag(&self) -> MvcAllowFlagR {
        MvcAllowFlagR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - vp7 support"]
    #[inline(always)]
    pub fn vp7_allow_flag(&self) -> Vp7AllowFlagR {
        Vp7AllowFlagR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - rom implementation type"]
    #[inline(always)]
    pub fn rom_imp_type(&self) -> RomImpTypeR {
        RomImpTypeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - rv_allow_flag"]
    #[inline(always)]
    pub fn rv_allow_flag(&self) -> RvAllowFlagR {
        RvAllowFlagR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - refbuffer2 support"]
    #[inline(always)]
    pub fn refbuf2_allow_flag(&self) -> Refbuf2AllowFlagR {
        Refbuf2AllowFlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - ref buffer support"]
    #[inline(always)]
    pub fn refbuf_allow_flag(&self) -> RefbufAllowFlagR {
        RefbufAllowFlagR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - JPEG sampling support\n\n16Mpixel~67Mpixel be sampled and supported by 411 and 444"]
    #[inline(always)]
    pub fn jpeg_allow_flag(&self) -> JpegAllowFlagR {
        JpegAllowFlagR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Synthesis configuration register decoder 1(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg67::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg67Spec;
impl crate::RegisterSpec for Swreg67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg67::R`](R) reader structure"]
impl crate::Readable for Swreg67Spec {}
#[doc = "`reset()` method sets SWREG67 to value 0xe5da_0000"]
impl crate::Resettable for Swreg67Spec {
    const RESET_VALUE: u32 = 0xe5da_0000;
}
