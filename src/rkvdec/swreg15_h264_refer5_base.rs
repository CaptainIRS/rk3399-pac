#[doc = "Register `SWREG15_H264_REFER5_BASE` reader"]
pub type R = crate::R<Swreg15H264Refer5BaseSpec>;
#[doc = "Register `SWREG15_H264_REFER5_BASE` writer"]
pub type W = crate::W<Swreg15H264Refer5BaseSpec>;
#[doc = "reference 5 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef5Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef5Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef5Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF5_FIELD` reader - reference 5 picture field flag"]
pub type SwRef5FieldR = crate::BitReader<SwRef5Field>;
impl SwRef5FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef5Field {
        match self.bits {
            false => SwRef5Field::B0,
            true => SwRef5Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef5Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef5Field::B1
    }
}
#[doc = "Field `SW_REF5_FIELD` writer - reference 5 picture field flag"]
pub type SwRef5FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef5Field>;
impl<'a, REG> SwRef5FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef5Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef5Field::B1)
    }
}
#[doc = "Field `SW_REF5_TOPFIELD_USED` reader - ref5 topfield is used\n\nref5 topfield is used"]
pub type SwRef5TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF5_TOPFIELD_USED` writer - ref5 topfield is used\n\nref5 topfield is used"]
pub type SwRef5TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF5_BOTFIELD_USED` reader - ref5 bottom field is used\n\nref5 bottom field is used"]
pub type SwRef5BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF5_BOTFIELD_USED` writer - ref5 bottom field is used\n\nref5 bottom field is used"]
pub type SwRef5BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF5_COLMV_USE_FLAG` reader - sw_ref5_colmv_use_flag\n\nsw_ref5_colmv_use_flag"]
pub type SwRef5ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF5_COLMV_USE_FLAG` writer - sw_ref5_colmv_use_flag\n\nsw_ref5_colmv_use_flag"]
pub type SwRef5ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER5_BASE` reader - base address for reference picture index5\n\nbase address for reference picture index5\n\n(the address should be 128bit align)"]
pub type SwRefer5BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER5_BASE` writer - base address for reference picture index5\n\nbase address for reference picture index5\n\n(the address should be 128bit align)"]
pub type SwRefer5BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 5 picture field flag"]
    #[inline(always)]
    pub fn sw_ref5_field(&self) -> SwRef5FieldR {
        SwRef5FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref5 topfield is used\n\nref5 topfield is used"]
    #[inline(always)]
    pub fn sw_ref5_topfield_used(&self) -> SwRef5TopfieldUsedR {
        SwRef5TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref5 bottom field is used\n\nref5 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref5_botfield_used(&self) -> SwRef5BotfieldUsedR {
        SwRef5BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref5_colmv_use_flag\n\nsw_ref5_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref5_colmv_use_flag(&self) -> SwRef5ColmvUseFlagR {
        SwRef5ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index5\n\nbase address for reference picture index5\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer5_base(&self) -> SwRefer5BaseR {
        SwRefer5BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 5 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref5_field(&mut self) -> SwRef5FieldW<Swreg15H264Refer5BaseSpec> {
        SwRef5FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref5 topfield is used\n\nref5 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref5_topfield_used(&mut self) -> SwRef5TopfieldUsedW<Swreg15H264Refer5BaseSpec> {
        SwRef5TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref5 bottom field is used\n\nref5 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref5_botfield_used(&mut self) -> SwRef5BotfieldUsedW<Swreg15H264Refer5BaseSpec> {
        SwRef5BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref5_colmv_use_flag\n\nsw_ref5_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref5_colmv_use_flag(&mut self) -> SwRef5ColmvUseFlagW<Swreg15H264Refer5BaseSpec> {
        SwRef5ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index5\n\nbase address for reference picture index5\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer5_base(&mut self) -> SwRefer5BaseW<Swreg15H264Refer5BaseSpec> {
        SwRefer5BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_h264_refer5_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_h264_refer5_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg15H264Refer5BaseSpec;
impl crate::RegisterSpec for Swreg15H264Refer5BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg15_h264_refer5_base::R`](R) reader structure"]
impl crate::Readable for Swreg15H264Refer5BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg15_h264_refer5_base::W`](W) writer structure"]
impl crate::Writable for Swreg15H264Refer5BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG15_H264_REFER5_BASE to value 0"]
impl crate::Resettable for Swreg15H264Refer5BaseSpec {
    const RESET_VALUE: u32 = 0;
}
