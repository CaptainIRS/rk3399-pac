#[doc = "Register `SWREG20_H264_REFER10_BASE` reader"]
pub type R = crate::R<Swreg20H264Refer10BaseSpec>;
#[doc = "Register `SWREG20_H264_REFER10_BASE` writer"]
pub type W = crate::W<Swreg20H264Refer10BaseSpec>;
#[doc = "reference 10 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef10Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef10Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef10Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF10_FIELD` reader - reference 10 picture field flag"]
pub type SwRef10FieldR = crate::BitReader<SwRef10Field>;
impl SwRef10FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef10Field {
        match self.bits {
            false => SwRef10Field::B0,
            true => SwRef10Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef10Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef10Field::B1
    }
}
#[doc = "Field `SW_REF10_FIELD` writer - reference 10 picture field flag"]
pub type SwRef10FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef10Field>;
impl<'a, REG> SwRef10FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef10Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef10Field::B1)
    }
}
#[doc = "Field `SW_REF10_TOPFIELD_USED` reader - ref10 topfield is used\n\nref10 topfield is used"]
pub type SwRef10TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF10_TOPFIELD_USED` writer - ref10 topfield is used\n\nref10 topfield is used"]
pub type SwRef10TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF10_BOTFIELD_USED` reader - ref10 bottom field is used\n\nref10 bottom field is used"]
pub type SwRef10BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF10_BOTFIELD_USED` writer - ref10 bottom field is used\n\nref10 bottom field is used"]
pub type SwRef10BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF10_COLMV_USE_FLAG` reader - sw_ref10_colmv_use_flag\n\nsw_ref10_colmv_use_flag"]
pub type SwRef10ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF10_COLMV_USE_FLAG` writer - sw_ref10_colmv_use_flag\n\nsw_ref10_colmv_use_flag"]
pub type SwRef10ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER10_BASE` reader - base address for reference picture index10\n\nbase address for reference picture index10\n\n(the address should be 128bit align)"]
pub type SwRefer10BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER10_BASE` writer - base address for reference picture index10\n\nbase address for reference picture index10\n\n(the address should be 128bit align)"]
pub type SwRefer10BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 10 picture field flag"]
    #[inline(always)]
    pub fn sw_ref10_field(&self) -> SwRef10FieldR {
        SwRef10FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref10 topfield is used\n\nref10 topfield is used"]
    #[inline(always)]
    pub fn sw_ref10_topfield_used(&self) -> SwRef10TopfieldUsedR {
        SwRef10TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref10 bottom field is used\n\nref10 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref10_botfield_used(&self) -> SwRef10BotfieldUsedR {
        SwRef10BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref10_colmv_use_flag\n\nsw_ref10_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref10_colmv_use_flag(&self) -> SwRef10ColmvUseFlagR {
        SwRef10ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index10\n\nbase address for reference picture index10\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer10_base(&self) -> SwRefer10BaseR {
        SwRefer10BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 10 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref10_field(&mut self) -> SwRef10FieldW<Swreg20H264Refer10BaseSpec> {
        SwRef10FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref10 topfield is used\n\nref10 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref10_topfield_used(&mut self) -> SwRef10TopfieldUsedW<Swreg20H264Refer10BaseSpec> {
        SwRef10TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref10 bottom field is used\n\nref10 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref10_botfield_used(&mut self) -> SwRef10BotfieldUsedW<Swreg20H264Refer10BaseSpec> {
        SwRef10BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref10_colmv_use_flag\n\nsw_ref10_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref10_colmv_use_flag(&mut self) -> SwRef10ColmvUseFlagW<Swreg20H264Refer10BaseSpec> {
        SwRef10ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index10\n\nbase address for reference picture index10\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer10_base(&mut self) -> SwRefer10BaseW<Swreg20H264Refer10BaseSpec> {
        SwRefer10BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20_h264_refer10_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20_h264_refer10_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg20H264Refer10BaseSpec;
impl crate::RegisterSpec for Swreg20H264Refer10BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg20_h264_refer10_base::R`](R) reader structure"]
impl crate::Readable for Swreg20H264Refer10BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg20_h264_refer10_base::W`](W) writer structure"]
impl crate::Writable for Swreg20H264Refer10BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG20_H264_REFER10_BASE to value 0"]
impl crate::Resettable for Swreg20H264Refer10BaseSpec {
    const RESET_VALUE: u32 = 0;
}
