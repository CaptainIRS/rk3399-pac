#[doc = "Register `SWREG48_H264_REFER15_BASE` reader"]
pub type R = crate::R<Swreg48H264Refer15BaseSpec>;
#[doc = "Register `SWREG48_H264_REFER15_BASE` writer"]
pub type W = crate::W<Swreg48H264Refer15BaseSpec>;
#[doc = "reference 15 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef15Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef15Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef15Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF15_FIELD` reader - reference 15 picture field flag"]
pub type SwRef15FieldR = crate::BitReader<SwRef15Field>;
impl SwRef15FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef15Field {
        match self.bits {
            false => SwRef15Field::B0,
            true => SwRef15Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef15Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef15Field::B1
    }
}
#[doc = "Field `SW_REF15_FIELD` writer - reference 15 picture field flag"]
pub type SwRef15FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef15Field>;
impl<'a, REG> SwRef15FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef15Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef15Field::B1)
    }
}
#[doc = "Field `SW_REF15_TOPFIELD_USED` reader - ref15 topfield is used\n\nref15 topfield is used"]
pub type SwRef15TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF15_TOPFIELD_USED` writer - ref15 topfield is used\n\nref15 topfield is used"]
pub type SwRef15TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF15_BOTFIELD_USED` reader - ref15 bottom field is used\n\nref15 bottom field is used"]
pub type SwRef15BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF15_BOTFIELD_USED` writer - ref15 bottom field is used\n\nref15 bottom field is used"]
pub type SwRef15BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF15_COLMV_USE_FLAG` reader - sw_ref15_colmv_use_flag\n\nsw_ref15_colmv_use_flag"]
pub type SwRef15ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF15_COLMV_USE_FLAG` writer - sw_ref15_colmv_use_flag\n\nsw_ref15_colmv_use_flag"]
pub type SwRef15ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER15_BASE` reader - base address for reference picture index15\n\nbase address for reference picture index15\n\n(the address should be 128bit align)"]
pub type SwRefer15BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER15_BASE` writer - base address for reference picture index15\n\nbase address for reference picture index15\n\n(the address should be 128bit align)"]
pub type SwRefer15BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 15 picture field flag"]
    #[inline(always)]
    pub fn sw_ref15_field(&self) -> SwRef15FieldR {
        SwRef15FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref15 topfield is used\n\nref15 topfield is used"]
    #[inline(always)]
    pub fn sw_ref15_topfield_used(&self) -> SwRef15TopfieldUsedR {
        SwRef15TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref15 bottom field is used\n\nref15 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref15_botfield_used(&self) -> SwRef15BotfieldUsedR {
        SwRef15BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref15_colmv_use_flag\n\nsw_ref15_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref15_colmv_use_flag(&self) -> SwRef15ColmvUseFlagR {
        SwRef15ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index15\n\nbase address for reference picture index15\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer15_base(&self) -> SwRefer15BaseR {
        SwRefer15BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 15 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref15_field(&mut self) -> SwRef15FieldW<Swreg48H264Refer15BaseSpec> {
        SwRef15FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref15 topfield is used\n\nref15 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref15_topfield_used(&mut self) -> SwRef15TopfieldUsedW<Swreg48H264Refer15BaseSpec> {
        SwRef15TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref15 bottom field is used\n\nref15 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref15_botfield_used(&mut self) -> SwRef15BotfieldUsedW<Swreg48H264Refer15BaseSpec> {
        SwRef15BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref15_colmv_use_flag\n\nsw_ref15_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref15_colmv_use_flag(&mut self) -> SwRef15ColmvUseFlagW<Swreg48H264Refer15BaseSpec> {
        SwRef15ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index15\n\nbase address for reference picture index15\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer15_base(&mut self) -> SwRefer15BaseW<Swreg48H264Refer15BaseSpec> {
        SwRefer15BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg48_h264_refer15_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg48_h264_refer15_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg48H264Refer15BaseSpec;
impl crate::RegisterSpec for Swreg48H264Refer15BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg48_h264_refer15_base::R`](R) reader structure"]
impl crate::Readable for Swreg48H264Refer15BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg48_h264_refer15_base::W`](W) writer structure"]
impl crate::Writable for Swreg48H264Refer15BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG48_H264_REFER15_BASE to value 0"]
impl crate::Resettable for Swreg48H264Refer15BaseSpec {
    const RESET_VALUE: u32 = 0;
}
