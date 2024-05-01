#[doc = "Register `SWREG17` reader"]
pub type R = crate::R<Swreg17Spec>;
#[doc = "Field `PP_OUTW_352_EN` reader - on-off for pp output width up to 352\n\n4st priority used"]
pub type PpOutw352EnR = crate::BitReader;
#[doc = "Field `PP_OUTW_720_EN` reader - on-off for pp output width up to 720\n\n3st priority used"]
pub type PpOutw720EnR = crate::BitReader;
#[doc = "Field `PP_OUTW_1280_EN` reader - on-off for pp output width up to 1280\n\n2st priority used"]
pub type PpOutw1280EnR = crate::BitReader;
#[doc = "Field `PP_OUTW_1920_EN` reader - on-off for pp output width up to 1920\n\n1st priority used"]
pub type PpOutw1920EnR = crate::BitReader;
#[doc = "pp work allow flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpWorkEn {
    #[doc = "0: off"]
    B0 = 0,
    #[doc = "1: on"]
    B1 = 1,
}
impl From<PpWorkEn> for bool {
    #[inline(always)]
    fn from(variant: PpWorkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_WORK_EN` reader - pp work allow flag"]
pub type PpWorkEnR = crate::BitReader<PpWorkEn>;
impl PpWorkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpWorkEn {
        match self.bits {
            false => PpWorkEn::B0,
            true => PpWorkEn::B1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpWorkEn::B0
    }
    #[doc = "on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpWorkEn::B1
    }
}
#[doc = "on-off for pp Alpha Blending\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpAbledEn {
    #[doc = "0: off"]
    B0 = 0,
    #[doc = "1: on"]
    B1 = 1,
}
impl From<PpAbledEn> for bool {
    #[inline(always)]
    fn from(variant: PpAbledEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_ABLED_EN` reader - on-off for pp Alpha Blending"]
pub type PpAbledEnR = crate::BitReader<PpAbledEn>;
impl PpAbledEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpAbledEn {
        match self.bits {
            false => PpAbledEn::B0,
            true => PpAbledEn::B1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpAbledEn::B0
    }
    #[doc = "on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpAbledEn::B1
    }
}
#[doc = "on-off for pp deinterlance\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpDeinterlEn {
    #[doc = "0: off"]
    B0 = 0,
    #[doc = "1: on"]
    B1 = 1,
}
impl From<PpDeinterlEn> for bool {
    #[inline(always)]
    fn from(variant: PpDeinterlEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_DEINTERL_EN` reader - on-off for pp deinterlance"]
pub type PpDeinterlEnR = crate::BitReader<PpDeinterlEn>;
impl PpDeinterlEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpDeinterlEn {
        match self.bits {
            false => PpDeinterlEn::B0,
            true => PpDeinterlEn::B1,
        }
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PpDeinterlEn::B0
    }
    #[doc = "on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PpDeinterlEn::B1
    }
}
impl R {
    #[doc = "Bit 0 - on-off for pp output width up to 352\n\n4st priority used"]
    #[inline(always)]
    pub fn pp_outw_352_en(&self) -> PpOutw352EnR {
        PpOutw352EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - on-off for pp output width up to 720\n\n3st priority used"]
    #[inline(always)]
    pub fn pp_outw_720_en(&self) -> PpOutw720EnR {
        PpOutw720EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - on-off for pp output width up to 1280\n\n2st priority used"]
    #[inline(always)]
    pub fn pp_outw_1280_en(&self) -> PpOutw1280EnR {
        PpOutw1280EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - on-off for pp output width up to 1920\n\n1st priority used"]
    #[inline(always)]
    pub fn pp_outw_1920_en(&self) -> PpOutw1920EnR {
        PpOutw1920EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 28 - pp work allow flag"]
    #[inline(always)]
    pub fn pp_work_en(&self) -> PpWorkEnR {
        PpWorkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - on-off for pp Alpha Blending"]
    #[inline(always)]
    pub fn pp_abled_en(&self) -> PpAbledEnR {
        PpAbledEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - on-off for pp deinterlance"]
    #[inline(always)]
    pub fn pp_deinterl_en(&self) -> PpDeinterlEnR {
        PpDeinterlEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "hw support informan,read only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg17Spec;
impl crate::RegisterSpec for Swreg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg17::R`](R) reader structure"]
impl crate::Readable for Swreg17Spec {}
#[doc = "`reset()` method sets SWREG17 to value 0"]
impl crate::Resettable for Swreg17Spec {
    const RESET_VALUE: u32 = 0;
}
