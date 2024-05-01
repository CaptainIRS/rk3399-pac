#[doc = "Register `SWREG13_H264_REFER3_BASE` reader"]
pub type R = crate::R<Swreg13H264Refer3BaseSpec>;
#[doc = "Register `SWREG13_H264_REFER3_BASE` writer"]
pub type W = crate::W<Swreg13H264Refer3BaseSpec>;
#[doc = "reference 3 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef3Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef3Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef3Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF3_FIELD` reader - reference 3 picture field flag"]
pub type SwRef3FieldR = crate::BitReader<SwRef3Field>;
impl SwRef3FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef3Field {
        match self.bits {
            false => SwRef3Field::B0,
            true => SwRef3Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef3Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef3Field::B1
    }
}
#[doc = "Field `SW_REF3_FIELD` writer - reference 3 picture field flag"]
pub type SwRef3FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef3Field>;
impl<'a, REG> SwRef3FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef3Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef3Field::B1)
    }
}
#[doc = "Field `SW_REF3_TOPFIELD_USED` reader - ref3 topfield is used\n\nref3 topfield is used"]
pub type SwRef3TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF3_TOPFIELD_USED` writer - ref3 topfield is used\n\nref3 topfield is used"]
pub type SwRef3TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF3_BOTFIELD_USED` reader - ref3 bottom field is used\n\nref3 bottom field is used"]
pub type SwRef3BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF3_BOTFIELD_USED` writer - ref3 bottom field is used\n\nref3 bottom field is used"]
pub type SwRef3BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF3_COLMV_USE_FLAG` reader - sw_ref3_colmv_use_flag\n\nsw_ref3_colmv_use_flag"]
pub type SwRef3ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF3_COLMV_USE_FLAG` writer - sw_ref3_colmv_use_flag\n\nsw_ref3_colmv_use_flag"]
pub type SwRef3ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER3_BASE` reader - base address for reference picture index3\n\nbase address for reference picture index3\n\n(the address should be 128bit align)"]
pub type SwRefer3BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER3_BASE` writer - base address for reference picture index3\n\nbase address for reference picture index3\n\n(the address should be 128bit align)"]
pub type SwRefer3BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 3 picture field flag"]
    #[inline(always)]
    pub fn sw_ref3_field(&self) -> SwRef3FieldR {
        SwRef3FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref3 topfield is used\n\nref3 topfield is used"]
    #[inline(always)]
    pub fn sw_ref3_topfield_used(&self) -> SwRef3TopfieldUsedR {
        SwRef3TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref3 bottom field is used\n\nref3 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref3_botfield_used(&self) -> SwRef3BotfieldUsedR {
        SwRef3BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref3_colmv_use_flag\n\nsw_ref3_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref3_colmv_use_flag(&self) -> SwRef3ColmvUseFlagR {
        SwRef3ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index3\n\nbase address for reference picture index3\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer3_base(&self) -> SwRefer3BaseR {
        SwRefer3BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 3 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref3_field(&mut self) -> SwRef3FieldW<Swreg13H264Refer3BaseSpec> {
        SwRef3FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref3 topfield is used\n\nref3 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref3_topfield_used(&mut self) -> SwRef3TopfieldUsedW<Swreg13H264Refer3BaseSpec> {
        SwRef3TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref3 bottom field is used\n\nref3 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref3_botfield_used(&mut self) -> SwRef3BotfieldUsedW<Swreg13H264Refer3BaseSpec> {
        SwRef3BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref3_colmv_use_flag\n\nsw_ref3_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref3_colmv_use_flag(&mut self) -> SwRef3ColmvUseFlagW<Swreg13H264Refer3BaseSpec> {
        SwRef3ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index3\n\nbase address for reference picture index3\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer3_base(&mut self) -> SwRefer3BaseW<Swreg13H264Refer3BaseSpec> {
        SwRefer3BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_h264_refer3_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_h264_refer3_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg13H264Refer3BaseSpec;
impl crate::RegisterSpec for Swreg13H264Refer3BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg13_h264_refer3_base::R`](R) reader structure"]
impl crate::Readable for Swreg13H264Refer3BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg13_h264_refer3_base::W`](W) writer structure"]
impl crate::Writable for Swreg13H264Refer3BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG13_H264_REFER3_BASE to value 0"]
impl crate::Resettable for Swreg13H264Refer3BaseSpec {
    const RESET_VALUE: u32 = 0;
}
