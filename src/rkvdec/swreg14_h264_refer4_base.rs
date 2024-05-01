#[doc = "Register `SWREG14_H264_REFER4_BASE` reader"]
pub type R = crate::R<Swreg14H264Refer4BaseSpec>;
#[doc = "Register `SWREG14_H264_REFER4_BASE` writer"]
pub type W = crate::W<Swreg14H264Refer4BaseSpec>;
#[doc = "reference 4 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef4Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef4Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef4Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF4_FIELD` reader - reference 4 picture field flag"]
pub type SwRef4FieldR = crate::BitReader<SwRef4Field>;
impl SwRef4FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef4Field {
        match self.bits {
            false => SwRef4Field::B0,
            true => SwRef4Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef4Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef4Field::B1
    }
}
#[doc = "Field `SW_REF4_FIELD` writer - reference 4 picture field flag"]
pub type SwRef4FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef4Field>;
impl<'a, REG> SwRef4FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef4Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef4Field::B1)
    }
}
#[doc = "Field `SW_REF4_TOPFIELD_USED` reader - ref4 topfield is used\n\nref4 topfield is used"]
pub type SwRef4TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF4_TOPFIELD_USED` writer - ref4 topfield is used\n\nref4 topfield is used"]
pub type SwRef4TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF4_BOTFIELD_USED` reader - ref4 bottom field is used\n\nref4 bottom field is used"]
pub type SwRef4BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF4_BOTFIELD_USED` writer - ref4 bottom field is used\n\nref4 bottom field is used"]
pub type SwRef4BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF4_COLMV_USE_FLAG` reader - sw_ref4_colmv_use_flag\n\nsw_ref4_colmv_use_flag"]
pub type SwRef4ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF4_COLMV_USE_FLAG` writer - sw_ref4_colmv_use_flag\n\nsw_ref4_colmv_use_flag"]
pub type SwRef4ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER4_BASE` reader - base address for reference picture index4\n\nbase address for reference picture index4\n\n(the address should be 128bit align)"]
pub type SwRefer4BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER4_BASE` writer - base address for reference picture index4\n\nbase address for reference picture index4\n\n(the address should be 128bit align)"]
pub type SwRefer4BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 4 picture field flag"]
    #[inline(always)]
    pub fn sw_ref4_field(&self) -> SwRef4FieldR {
        SwRef4FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref4 topfield is used\n\nref4 topfield is used"]
    #[inline(always)]
    pub fn sw_ref4_topfield_used(&self) -> SwRef4TopfieldUsedR {
        SwRef4TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref4 bottom field is used\n\nref4 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref4_botfield_used(&self) -> SwRef4BotfieldUsedR {
        SwRef4BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref4_colmv_use_flag\n\nsw_ref4_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref4_colmv_use_flag(&self) -> SwRef4ColmvUseFlagR {
        SwRef4ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index4\n\nbase address for reference picture index4\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer4_base(&self) -> SwRefer4BaseR {
        SwRefer4BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 4 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref4_field(&mut self) -> SwRef4FieldW<Swreg14H264Refer4BaseSpec> {
        SwRef4FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref4 topfield is used\n\nref4 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref4_topfield_used(&mut self) -> SwRef4TopfieldUsedW<Swreg14H264Refer4BaseSpec> {
        SwRef4TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref4 bottom field is used\n\nref4 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref4_botfield_used(&mut self) -> SwRef4BotfieldUsedW<Swreg14H264Refer4BaseSpec> {
        SwRef4BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref4_colmv_use_flag\n\nsw_ref4_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref4_colmv_use_flag(&mut self) -> SwRef4ColmvUseFlagW<Swreg14H264Refer4BaseSpec> {
        SwRef4ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index4\n\nbase address for reference picture index4\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer4_base(&mut self) -> SwRefer4BaseW<Swreg14H264Refer4BaseSpec> {
        SwRefer4BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_h264_refer4_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_h264_refer4_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg14H264Refer4BaseSpec;
impl crate::RegisterSpec for Swreg14H264Refer4BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg14_h264_refer4_base::R`](R) reader structure"]
impl crate::Readable for Swreg14H264Refer4BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg14_h264_refer4_base::W`](W) writer structure"]
impl crate::Writable for Swreg14H264Refer4BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG14_H264_REFER4_BASE to value 0"]
impl crate::Resettable for Swreg14H264Refer4BaseSpec {
    const RESET_VALUE: u32 = 0;
}
