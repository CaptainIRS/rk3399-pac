#[doc = "Register `SWREG11_H264_REFER1_BASE` reader"]
pub type R = crate::R<Swreg11H264Refer1BaseSpec>;
#[doc = "Register `SWREG11_H264_REFER1_BASE` writer"]
pub type W = crate::W<Swreg11H264Refer1BaseSpec>;
#[doc = "reference 1 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef1Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef1Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef1Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF1_FIELD` reader - reference 1 picture field flag"]
pub type SwRef1FieldR = crate::BitReader<SwRef1Field>;
impl SwRef1FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef1Field {
        match self.bits {
            false => SwRef1Field::B0,
            true => SwRef1Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef1Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef1Field::B1
    }
}
#[doc = "Field `SW_REF1_FIELD` writer - reference 1 picture field flag"]
pub type SwRef1FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef1Field>;
impl<'a, REG> SwRef1FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef1Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef1Field::B1)
    }
}
#[doc = "Field `SW_REF1_TOPFIELD_USED` reader - ref1 topfield is used\n\nref1 topfield is used"]
pub type SwRef1TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF1_TOPFIELD_USED` writer - ref1 topfield is used\n\nref1 topfield is used"]
pub type SwRef1TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF1_BOTFIELD_USED` reader - ref1 bottom field is used\n\nref1 bottom field is used"]
pub type SwRef1BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF1_BOTFIELD_USED` writer - ref1 bottom field is used\n\nref1 bottom field is used"]
pub type SwRef1BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF1_COLMV_USE_FLAG` reader - sw_ref1_colmv_use_flag\n\nsw_ref1_colmv_use_flag"]
pub type SwRef1ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF1_COLMV_USE_FLAG` writer - sw_ref1_colmv_use_flag\n\nsw_ref1_colmv_use_flag"]
pub type SwRef1ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER1_BASE` reader - base address for reference picture index1\n\nbase address for reference picture index1 (the address should be\n\n128bit align)"]
pub type SwRefer1BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER1_BASE` writer - base address for reference picture index1\n\nbase address for reference picture index1 (the address should be\n\n128bit align)"]
pub type SwRefer1BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 1 picture field flag"]
    #[inline(always)]
    pub fn sw_ref1_field(&self) -> SwRef1FieldR {
        SwRef1FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref1 topfield is used\n\nref1 topfield is used"]
    #[inline(always)]
    pub fn sw_ref1_topfield_used(&self) -> SwRef1TopfieldUsedR {
        SwRef1TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref1 bottom field is used\n\nref1 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref1_botfield_used(&self) -> SwRef1BotfieldUsedR {
        SwRef1BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref1_colmv_use_flag\n\nsw_ref1_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref1_colmv_use_flag(&self) -> SwRef1ColmvUseFlagR {
        SwRef1ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index1\n\nbase address for reference picture index1 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer1_base(&self) -> SwRefer1BaseR {
        SwRefer1BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 1 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref1_field(&mut self) -> SwRef1FieldW<Swreg11H264Refer1BaseSpec> {
        SwRef1FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref1 topfield is used\n\nref1 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref1_topfield_used(&mut self) -> SwRef1TopfieldUsedW<Swreg11H264Refer1BaseSpec> {
        SwRef1TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref1 bottom field is used\n\nref1 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref1_botfield_used(&mut self) -> SwRef1BotfieldUsedW<Swreg11H264Refer1BaseSpec> {
        SwRef1BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref1_colmv_use_flag\n\nsw_ref1_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref1_colmv_use_flag(&mut self) -> SwRef1ColmvUseFlagW<Swreg11H264Refer1BaseSpec> {
        SwRef1ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index1\n\nbase address for reference picture index1 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer1_base(&mut self) -> SwRefer1BaseW<Swreg11H264Refer1BaseSpec> {
        SwRefer1BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_h264_refer1_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_h264_refer1_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg11H264Refer1BaseSpec;
impl crate::RegisterSpec for Swreg11H264Refer1BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg11_h264_refer1_base::R`](R) reader structure"]
impl crate::Readable for Swreg11H264Refer1BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg11_h264_refer1_base::W`](W) writer structure"]
impl crate::Writable for Swreg11H264Refer1BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG11_H264_REFER1_BASE to value 0"]
impl crate::Resettable for Swreg11H264Refer1BaseSpec {
    const RESET_VALUE: u32 = 0;
}
