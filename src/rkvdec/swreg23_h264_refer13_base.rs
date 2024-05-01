#[doc = "Register `SWREG23_H264_REFER13_BASE` reader"]
pub type R = crate::R<Swreg23H264Refer13BaseSpec>;
#[doc = "Register `SWREG23_H264_REFER13_BASE` writer"]
pub type W = crate::W<Swreg23H264Refer13BaseSpec>;
#[doc = "reference 13 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef13Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef13Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef13Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF13_FIELD` reader - reference 13 picture field flag"]
pub type SwRef13FieldR = crate::BitReader<SwRef13Field>;
impl SwRef13FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef13Field {
        match self.bits {
            false => SwRef13Field::B0,
            true => SwRef13Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef13Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef13Field::B1
    }
}
#[doc = "Field `SW_REF13_FIELD` writer - reference 13 picture field flag"]
pub type SwRef13FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef13Field>;
impl<'a, REG> SwRef13FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef13Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef13Field::B1)
    }
}
#[doc = "Field `SW_REF13_TOPFIELD_USED` reader - ref13 topfield is used\n\nref13 topfield is used"]
pub type SwRef13TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF13_TOPFIELD_USED` writer - ref13 topfield is used\n\nref13 topfield is used"]
pub type SwRef13TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF13_BOTFIELD_USED` reader - ref13 bottom field is used\n\nref13 bottom field is used"]
pub type SwRef13BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF13_BOTFIELD_USED` writer - ref13 bottom field is used\n\nref13 bottom field is used"]
pub type SwRef13BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF13_COLMV_USE_FLAG` reader - sw_ref13_colmv_use_flag\n\nsw_ref13_colmv_use_flag"]
pub type SwRef13ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF13_COLMV_USE_FLAG` writer - sw_ref13_colmv_use_flag\n\nsw_ref13_colmv_use_flag"]
pub type SwRef13ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER13_BASE` reader - base address for reference picture index13\n\nbase address for reference picture index13\n\n(the address should be 128bit align)"]
pub type SwRefer13BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER13_BASE` writer - base address for reference picture index13\n\nbase address for reference picture index13\n\n(the address should be 128bit align)"]
pub type SwRefer13BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 13 picture field flag"]
    #[inline(always)]
    pub fn sw_ref13_field(&self) -> SwRef13FieldR {
        SwRef13FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref13 topfield is used\n\nref13 topfield is used"]
    #[inline(always)]
    pub fn sw_ref13_topfield_used(&self) -> SwRef13TopfieldUsedR {
        SwRef13TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref13 bottom field is used\n\nref13 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref13_botfield_used(&self) -> SwRef13BotfieldUsedR {
        SwRef13BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref13_colmv_use_flag\n\nsw_ref13_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref13_colmv_use_flag(&self) -> SwRef13ColmvUseFlagR {
        SwRef13ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index13\n\nbase address for reference picture index13\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer13_base(&self) -> SwRefer13BaseR {
        SwRefer13BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 13 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref13_field(&mut self) -> SwRef13FieldW<Swreg23H264Refer13BaseSpec> {
        SwRef13FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref13 topfield is used\n\nref13 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref13_topfield_used(&mut self) -> SwRef13TopfieldUsedW<Swreg23H264Refer13BaseSpec> {
        SwRef13TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref13 bottom field is used\n\nref13 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref13_botfield_used(&mut self) -> SwRef13BotfieldUsedW<Swreg23H264Refer13BaseSpec> {
        SwRef13BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref13_colmv_use_flag\n\nsw_ref13_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref13_colmv_use_flag(&mut self) -> SwRef13ColmvUseFlagW<Swreg23H264Refer13BaseSpec> {
        SwRef13ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index13\n\nbase address for reference picture index13\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer13_base(&mut self) -> SwRefer13BaseW<Swreg23H264Refer13BaseSpec> {
        SwRefer13BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23_h264_refer13_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23_h264_refer13_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg23H264Refer13BaseSpec;
impl crate::RegisterSpec for Swreg23H264Refer13BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg23_h264_refer13_base::R`](R) reader structure"]
impl crate::Readable for Swreg23H264Refer13BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg23_h264_refer13_base::W`](W) writer structure"]
impl crate::Writable for Swreg23H264Refer13BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG23_H264_REFER13_BASE to value 0"]
impl crate::Resettable for Swreg23H264Refer13BaseSpec {
    const RESET_VALUE: u32 = 0;
}
