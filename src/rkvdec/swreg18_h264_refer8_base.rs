#[doc = "Register `SWREG18_H264_REFER8_BASE` reader"]
pub type R = crate::R<Swreg18H264Refer8BaseSpec>;
#[doc = "Register `SWREG18_H264_REFER8_BASE` writer"]
pub type W = crate::W<Swreg18H264Refer8BaseSpec>;
#[doc = "reference 8 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef8Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef8Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef8Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF8_FIELD` reader - reference 8 picture field flag"]
pub type SwRef8FieldR = crate::BitReader<SwRef8Field>;
impl SwRef8FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef8Field {
        match self.bits {
            false => SwRef8Field::B0,
            true => SwRef8Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef8Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef8Field::B1
    }
}
#[doc = "Field `SW_REF8_FIELD` writer - reference 8 picture field flag"]
pub type SwRef8FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef8Field>;
impl<'a, REG> SwRef8FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef8Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef8Field::B1)
    }
}
#[doc = "Field `SW_REF8_TOPFILED_USED` reader - ref8 topfield is used\n\nref8 topfield is used"]
pub type SwRef8TopfiledUsedR = crate::BitReader;
#[doc = "Field `SW_REF8_TOPFILED_USED` writer - ref8 topfield is used\n\nref8 topfield is used"]
pub type SwRef8TopfiledUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF8_BOTFIELD_USED` reader - ref8 bottom field is used\n\nref8 bottom field is used"]
pub type SwRef8BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF8_BOTFIELD_USED` writer - ref8 bottom field is used\n\nref8 bottom field is used"]
pub type SwRef8BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF8_COLMV_USE_FLAG` reader - sw_ref8_colmv_use_flag\n\nsw_ref8_colmv_use_flag"]
pub type SwRef8ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF8_COLMV_USE_FLAG` writer - sw_ref8_colmv_use_flag\n\nsw_ref8_colmv_use_flag"]
pub type SwRef8ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER8_BASE` reader - base address for reference picture index8\n\nbase address for reference picture index8\n\n(the address should be 128bit align)"]
pub type SwRefer8BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER8_BASE` writer - base address for reference picture index8\n\nbase address for reference picture index8\n\n(the address should be 128bit align)"]
pub type SwRefer8BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 8 picture field flag"]
    #[inline(always)]
    pub fn sw_ref8_field(&self) -> SwRef8FieldR {
        SwRef8FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref8 topfield is used\n\nref8 topfield is used"]
    #[inline(always)]
    pub fn sw_ref8_topfiled_used(&self) -> SwRef8TopfiledUsedR {
        SwRef8TopfiledUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref8 bottom field is used\n\nref8 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref8_botfield_used(&self) -> SwRef8BotfieldUsedR {
        SwRef8BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref8_colmv_use_flag\n\nsw_ref8_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref8_colmv_use_flag(&self) -> SwRef8ColmvUseFlagR {
        SwRef8ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index8\n\nbase address for reference picture index8\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer8_base(&self) -> SwRefer8BaseR {
        SwRefer8BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 8 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref8_field(&mut self) -> SwRef8FieldW<Swreg18H264Refer8BaseSpec> {
        SwRef8FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref8 topfield is used\n\nref8 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref8_topfiled_used(&mut self) -> SwRef8TopfiledUsedW<Swreg18H264Refer8BaseSpec> {
        SwRef8TopfiledUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref8 bottom field is used\n\nref8 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref8_botfield_used(&mut self) -> SwRef8BotfieldUsedW<Swreg18H264Refer8BaseSpec> {
        SwRef8BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref8_colmv_use_flag\n\nsw_ref8_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref8_colmv_use_flag(&mut self) -> SwRef8ColmvUseFlagW<Swreg18H264Refer8BaseSpec> {
        SwRef8ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index8\n\nbase address for reference picture index8\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer8_base(&mut self) -> SwRefer8BaseW<Swreg18H264Refer8BaseSpec> {
        SwRefer8BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_h264_refer8_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_h264_refer8_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg18H264Refer8BaseSpec;
impl crate::RegisterSpec for Swreg18H264Refer8BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg18_h264_refer8_base::R`](R) reader structure"]
impl crate::Readable for Swreg18H264Refer8BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg18_h264_refer8_base::W`](W) writer structure"]
impl crate::Writable for Swreg18H264Refer8BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG18_H264_REFER8_BASE to value 0"]
impl crate::Resettable for Swreg18H264Refer8BaseSpec {
    const RESET_VALUE: u32 = 0;
}
