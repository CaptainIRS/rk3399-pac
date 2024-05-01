#[doc = "Register `SWREG16_H264_REFER6_BASE` reader"]
pub type R = crate::R<Swreg16H264Refer6BaseSpec>;
#[doc = "Register `SWREG16_H264_REFER6_BASE` writer"]
pub type W = crate::W<Swreg16H264Refer6BaseSpec>;
#[doc = "reference 6 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef6Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef6Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef6Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF6_FIELD` reader - reference 6 picture field flag"]
pub type SwRef6FieldR = crate::BitReader<SwRef6Field>;
impl SwRef6FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef6Field {
        match self.bits {
            false => SwRef6Field::B0,
            true => SwRef6Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef6Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef6Field::B1
    }
}
#[doc = "Field `SW_REF6_FIELD` writer - reference 6 picture field flag"]
pub type SwRef6FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef6Field>;
impl<'a, REG> SwRef6FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef6Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef6Field::B1)
    }
}
#[doc = "Field `SW_REF6_TOPFIELD_USED` reader - ref6 topfield is used\n\nref6 topfield is used"]
pub type SwRef6TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF6_TOPFIELD_USED` writer - ref6 topfield is used\n\nref6 topfield is used"]
pub type SwRef6TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF6_BOTFIELD_USED` reader - ref6 botfield is used\n\nref6 botfield is used"]
pub type SwRef6BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF6_BOTFIELD_USED` writer - ref6 botfield is used\n\nref6 botfield is used"]
pub type SwRef6BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF6_COLMV_USE_FLAG` reader - sw_ref6_colmv_use_flag\n\nsw_ref6_colmv_use_flag"]
pub type SwRef6ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF6_COLMV_USE_FLAG` writer - sw_ref6_colmv_use_flag\n\nsw_ref6_colmv_use_flag"]
pub type SwRef6ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER6_BASE` reader - base address for reference picture index6\n\nbase address for reference picture index6\n\n(the address should be 128bit align)"]
pub type SwRefer6BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER6_BASE` writer - base address for reference picture index6\n\nbase address for reference picture index6\n\n(the address should be 128bit align)"]
pub type SwRefer6BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 6 picture field flag"]
    #[inline(always)]
    pub fn sw_ref6_field(&self) -> SwRef6FieldR {
        SwRef6FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref6 topfield is used\n\nref6 topfield is used"]
    #[inline(always)]
    pub fn sw_ref6_topfield_used(&self) -> SwRef6TopfieldUsedR {
        SwRef6TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref6 botfield is used\n\nref6 botfield is used"]
    #[inline(always)]
    pub fn sw_ref6_botfield_used(&self) -> SwRef6BotfieldUsedR {
        SwRef6BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref6_colmv_use_flag\n\nsw_ref6_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref6_colmv_use_flag(&self) -> SwRef6ColmvUseFlagR {
        SwRef6ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index6\n\nbase address for reference picture index6\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer6_base(&self) -> SwRefer6BaseR {
        SwRefer6BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 6 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref6_field(&mut self) -> SwRef6FieldW<Swreg16H264Refer6BaseSpec> {
        SwRef6FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref6 topfield is used\n\nref6 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref6_topfield_used(&mut self) -> SwRef6TopfieldUsedW<Swreg16H264Refer6BaseSpec> {
        SwRef6TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref6 botfield is used\n\nref6 botfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref6_botfield_used(&mut self) -> SwRef6BotfieldUsedW<Swreg16H264Refer6BaseSpec> {
        SwRef6BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref6_colmv_use_flag\n\nsw_ref6_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref6_colmv_use_flag(&mut self) -> SwRef6ColmvUseFlagW<Swreg16H264Refer6BaseSpec> {
        SwRef6ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index6\n\nbase address for reference picture index6\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer6_base(&mut self) -> SwRefer6BaseW<Swreg16H264Refer6BaseSpec> {
        SwRefer6BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_h264_refer6_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_h264_refer6_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg16H264Refer6BaseSpec;
impl crate::RegisterSpec for Swreg16H264Refer6BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg16_h264_refer6_base::R`](R) reader structure"]
impl crate::Readable for Swreg16H264Refer6BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg16_h264_refer6_base::W`](W) writer structure"]
impl crate::Writable for Swreg16H264Refer6BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG16_H264_REFER6_BASE to value 0"]
impl crate::Resettable for Swreg16H264Refer6BaseSpec {
    const RESET_VALUE: u32 = 0;
}
