#[doc = "Register `SWREG17_H264_REFER7_BASE` reader"]
pub type R = crate::R<Swreg17H264Refer7BaseSpec>;
#[doc = "Register `SWREG17_H264_REFER7_BASE` writer"]
pub type W = crate::W<Swreg17H264Refer7BaseSpec>;
#[doc = "reference 7 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef7Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef7Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef7Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF7_FIELD` reader - reference 7 picture field flag"]
pub type SwRef7FieldR = crate::BitReader<SwRef7Field>;
impl SwRef7FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef7Field {
        match self.bits {
            false => SwRef7Field::B0,
            true => SwRef7Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef7Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef7Field::B1
    }
}
#[doc = "Field `SW_REF7_FIELD` writer - reference 7 picture field flag"]
pub type SwRef7FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef7Field>;
impl<'a, REG> SwRef7FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef7Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef7Field::B1)
    }
}
#[doc = "Field `SW_REF7_TOPFIELD_USED` reader - ref7 topfield is used\n\nref7 topfield is used"]
pub type SwRef7TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF7_TOPFIELD_USED` writer - ref7 topfield is used\n\nref7 topfield is used"]
pub type SwRef7TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF7_BOTFIELD_USED` reader - ref7 bottom field is used\n\nref7 bottom field is used"]
pub type SwRef7BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF7_BOTFIELD_USED` writer - ref7 bottom field is used\n\nref7 bottom field is used"]
pub type SwRef7BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF7_COLMV_USE_FLAG` reader - sw_ref7_colmv_use_flag\n\nsw_ref7_colmv_use_flag"]
pub type SwRef7ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF7_COLMV_USE_FLAG` writer - sw_ref7_colmv_use_flag\n\nsw_ref7_colmv_use_flag"]
pub type SwRef7ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER7_BASE` reader - base address for reference picture index7\n\nbase address for reference picture index7\n\n(the address should be 128bit align)"]
pub type SwRefer7BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER7_BASE` writer - base address for reference picture index7\n\nbase address for reference picture index7\n\n(the address should be 128bit align)"]
pub type SwRefer7BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 7 picture field flag"]
    #[inline(always)]
    pub fn sw_ref7_field(&self) -> SwRef7FieldR {
        SwRef7FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref7 topfield is used\n\nref7 topfield is used"]
    #[inline(always)]
    pub fn sw_ref7_topfield_used(&self) -> SwRef7TopfieldUsedR {
        SwRef7TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref7 bottom field is used\n\nref7 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref7_botfield_used(&self) -> SwRef7BotfieldUsedR {
        SwRef7BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref7_colmv_use_flag\n\nsw_ref7_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref7_colmv_use_flag(&self) -> SwRef7ColmvUseFlagR {
        SwRef7ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index7\n\nbase address for reference picture index7\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer7_base(&self) -> SwRefer7BaseR {
        SwRefer7BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 7 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref7_field(&mut self) -> SwRef7FieldW<Swreg17H264Refer7BaseSpec> {
        SwRef7FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref7 topfield is used\n\nref7 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref7_topfield_used(&mut self) -> SwRef7TopfieldUsedW<Swreg17H264Refer7BaseSpec> {
        SwRef7TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref7 bottom field is used\n\nref7 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref7_botfield_used(&mut self) -> SwRef7BotfieldUsedW<Swreg17H264Refer7BaseSpec> {
        SwRef7BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref7_colmv_use_flag\n\nsw_ref7_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref7_colmv_use_flag(&mut self) -> SwRef7ColmvUseFlagW<Swreg17H264Refer7BaseSpec> {
        SwRef7ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index7\n\nbase address for reference picture index7\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer7_base(&mut self) -> SwRefer7BaseW<Swreg17H264Refer7BaseSpec> {
        SwRefer7BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17_h264_refer7_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg17_h264_refer7_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg17H264Refer7BaseSpec;
impl crate::RegisterSpec for Swreg17H264Refer7BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg17_h264_refer7_base::R`](R) reader structure"]
impl crate::Readable for Swreg17H264Refer7BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg17_h264_refer7_base::W`](W) writer structure"]
impl crate::Writable for Swreg17H264Refer7BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG17_H264_REFER7_BASE to value 0"]
impl crate::Resettable for Swreg17H264Refer7BaseSpec {
    const RESET_VALUE: u32 = 0;
}
