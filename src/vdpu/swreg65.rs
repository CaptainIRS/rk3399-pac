#[doc = "Register `SWREG65` reader"]
pub type R = crate::R<Swreg65Spec>;
#[doc = "Register `SWREG65` writer"]
pub type W = crate::W<Swreg65Spec>;
#[doc = "Field `SW_REFBUF_Y_OFSET` reader - the y offset for refbufferd\n\nif hw should compensate the global motion of the video for better\n\nbuffer hit rate will use this coordinate"]
pub type SwRefbufYOfsetR = crate::FieldReader<u16>;
#[doc = "Field `SW_REFBUF_Y_OFSET` writer - the y offset for refbufferd\n\nif hw should compensate the global motion of the video for better\n\nbuffer hit rate will use this coordinate"]
pub type SwRefbufYOfsetW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "the mode enable for Field parity mode enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRefbufFildparModE {
    #[doc = "0: the result field of the evaluation be used"]
    B0 = 0,
    #[doc = "1: the parity mode field be used"]
    B1 = 1,
}
impl From<SwRefbufFildparModE> for bool {
    #[inline(always)]
    fn from(variant: SwRefbufFildparModE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REFBUF_FILDPAR_MOD_E` reader - the mode enable for Field parity mode enable."]
pub type SwRefbufFildparModER = crate::BitReader<SwRefbufFildparModE>;
impl SwRefbufFildparModER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRefbufFildparModE {
        match self.bits {
            false => SwRefbufFildparModE::B0,
            true => SwRefbufFildparModE::B1,
        }
    }
    #[doc = "the result field of the evaluation be used"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRefbufFildparModE::B0
    }
    #[doc = "the parity mode field be used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRefbufFildparModE::B1
    }
}
#[doc = "Field `SW_REFBUF_FILDPAR_MOD_E` writer - the mode enable for Field parity mode enable."]
pub type SwRefbufFildparModEW<'a, REG> = crate::BitWriter<'a, REG, SwRefbufFildparModE>;
impl<'a, REG> SwRefbufFildparModEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the result field of the evaluation be used"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefbufFildparModE::B0)
    }
    #[doc = "the parity mode field be used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefbufFildparModE::B1)
    }
}
#[doc = "Field `SW_REFBUF_IDCAL_E` reader - Enable for HW internal reference ID calculation\n\nIf given threshold level is reached by any picture_id after first MB\n\nrow,\n\nthat picture_id is used for reference buffer fill for rest of the\n\npicture"]
pub type SwRefbufIdcalER = crate::BitReader;
#[doc = "Field `SW_REFBUF_IDCAL_E` writer - Enable for HW internal reference ID calculation\n\nIf given threshold level is reached by any picture_id after first MB\n\nrow,\n\nthat picture_id is used for reference buffer fill for rest of the\n\npicture"]
pub type SwRefbufIdcalEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFBUF_PICID` reader - the picture id for refference buffer"]
pub type SwRefbufPicidR = crate::FieldReader;
#[doc = "Field `SW_REFBUF_PICID` writer - the picture id for refference buffer"]
pub type SwRefbufPicidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SW_REFBU_THR_LEVEL` reader - Reference buffer disable threshold value (cache miss\n\namount). Used to buffer shut down (if more misses than\n\nallowed)"]
pub type SwRefbuThrLevelR = crate::FieldReader<u16>;
#[doc = "Field `SW_REFBU_THR_LEVEL` writer - Reference buffer disable threshold value (cache miss\n\namount). Used to buffer shut down (if more misses than\n\nallowed)"]
pub type SwRefbuThrLevelW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRefbuE {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwRefbuE> for bool {
    #[inline(always)]
    fn from(variant: SwRefbuE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REFBU_E` reader - "]
pub type SwRefbuER = crate::BitReader<SwRefbuE>;
impl SwRefbuER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRefbuE {
        match self.bits {
            false => SwRefbuE::B0,
            true => SwRefbuE::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRefbuE::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRefbuE::B1
    }
}
#[doc = "Field `SW_REFBU_E` writer - "]
pub type SwRefbuEW<'a, REG> = crate::BitWriter<'a, REG, SwRefbuE>;
impl<'a, REG> SwRefbuEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefbuE::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefbuE::B1)
    }
}
impl R {
    #[doc = "Bits 0:8 - the y offset for refbufferd\n\nif hw should compensate the global motion of the video for better\n\nbuffer hit rate will use this coordinate"]
    #[inline(always)]
    pub fn sw_refbuf_y_ofset(&self) -> SwRefbufYOfsetR {
        SwRefbufYOfsetR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - the mode enable for Field parity mode enable."]
    #[inline(always)]
    pub fn sw_refbuf_fildpar_mod_e(&self) -> SwRefbufFildparModER {
        SwRefbufFildparModER::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable for HW internal reference ID calculation\n\nIf given threshold level is reached by any picture_id after first MB\n\nrow,\n\nthat picture_id is used for reference buffer fill for rest of the\n\npicture"]
    #[inline(always)]
    pub fn sw_refbuf_idcal_e(&self) -> SwRefbufIdcalER {
        SwRefbufIdcalER::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:18 - the picture id for refference buffer"]
    #[inline(always)]
    pub fn sw_refbuf_picid(&self) -> SwRefbufPicidR {
        SwRefbufPicidR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:30 - Reference buffer disable threshold value (cache miss\n\namount). Used to buffer shut down (if more misses than\n\nallowed)"]
    #[inline(always)]
    pub fn sw_refbu_thr_level(&self) -> SwRefbuThrLevelR {
        SwRefbuThrLevelR::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sw_refbu_e(&self) -> SwRefbuER {
        SwRefbuER::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - the y offset for refbufferd\n\nif hw should compensate the global motion of the video for better\n\nbuffer hit rate will use this coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_y_ofset(&mut self) -> SwRefbufYOfsetW<Swreg65Spec> {
        SwRefbufYOfsetW::new(self, 0)
    }
    #[doc = "Bit 12 - the mode enable for Field parity mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_fildpar_mod_e(&mut self) -> SwRefbufFildparModEW<Swreg65Spec> {
        SwRefbufFildparModEW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable for HW internal reference ID calculation\n\nIf given threshold level is reached by any picture_id after first MB\n\nrow,\n\nthat picture_id is used for reference buffer fill for rest of the\n\npicture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_idcal_e(&mut self) -> SwRefbufIdcalEW<Swreg65Spec> {
        SwRefbufIdcalEW::new(self, 13)
    }
    #[doc = "Bits 14:18 - the picture id for refference buffer"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_picid(&mut self) -> SwRefbufPicidW<Swreg65Spec> {
        SwRefbufPicidW::new(self, 14)
    }
    #[doc = "Bits 19:30 - Reference buffer disable threshold value (cache miss\n\namount). Used to buffer shut down (if more misses than\n\nallowed)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbu_thr_level(&mut self) -> SwRefbuThrLevelW<Swreg65Spec> {
        SwRefbuThrLevelW::new(self, 19)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbu_e(&mut self) -> SwRefbuEW<Swreg65Spec> {
        SwRefbuEW::new(self, 31)
    }
}
#[doc = "refbufferd related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg65Spec;
impl crate::RegisterSpec for Swreg65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg65::R`](R) reader structure"]
impl crate::Readable for Swreg65Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg65::W`](W) writer structure"]
impl crate::Writable for Swreg65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG65 to value 0"]
impl crate::Resettable for Swreg65Spec {
    const RESET_VALUE: u32 = 0;
}
