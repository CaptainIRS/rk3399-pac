#[doc = "Register `SWREG24_H264_REFER14_BASE` reader"]
pub type R = crate::R<Swreg24H264Refer14BaseSpec>;
#[doc = "Register `SWREG24_H264_REFER14_BASE` writer"]
pub type W = crate::W<Swreg24H264Refer14BaseSpec>;
#[doc = "reference 14 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef14Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef14Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef14Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF14_FIELD` reader - reference 14 picture field flag"]
pub type SwRef14FieldR = crate::BitReader<SwRef14Field>;
impl SwRef14FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef14Field {
        match self.bits {
            false => SwRef14Field::B0,
            true => SwRef14Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef14Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef14Field::B1
    }
}
#[doc = "Field `SW_REF14_FIELD` writer - reference 14 picture field flag"]
pub type SwRef14FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef14Field>;
impl<'a, REG> SwRef14FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef14Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef14Field::B1)
    }
}
#[doc = "Field `SW_REF14_TOPFIELD_USED` reader - ref14 topfield is used\n\nref14 topfield is used"]
pub type SwRef14TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF14_TOPFIELD_USED` writer - ref14 topfield is used\n\nref14 topfield is used"]
pub type SwRef14TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF14_BOTFIELD_USED` reader - ref14 bottom field is used\n\nref14 bottom field is used"]
pub type SwRef14BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF14_BOTFIELD_USED` writer - ref14 bottom field is used\n\nref14 bottom field is used"]
pub type SwRef14BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF14_COLMV_USE_FLAG` reader - sw_ref14_colmv_use_flag\n\nsw_ref14_colmv_use_flag"]
pub type SwRef14ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF14_COLMV_USE_FLAG` writer - sw_ref14_colmv_use_flag\n\nsw_ref14_colmv_use_flag"]
pub type SwRef14ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER14_BASE` reader - base address for reference picture index14\n\nbase address for reference picture index14\n\n(the address should be 128bit align)"]
pub type SwRefer14BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER14_BASE` writer - base address for reference picture index14\n\nbase address for reference picture index14\n\n(the address should be 128bit align)"]
pub type SwRefer14BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 14 picture field flag"]
    #[inline(always)]
    pub fn sw_ref14_field(&self) -> SwRef14FieldR {
        SwRef14FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref14 topfield is used\n\nref14 topfield is used"]
    #[inline(always)]
    pub fn sw_ref14_topfield_used(&self) -> SwRef14TopfieldUsedR {
        SwRef14TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref14 bottom field is used\n\nref14 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref14_botfield_used(&self) -> SwRef14BotfieldUsedR {
        SwRef14BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref14_colmv_use_flag\n\nsw_ref14_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref14_colmv_use_flag(&self) -> SwRef14ColmvUseFlagR {
        SwRef14ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index14\n\nbase address for reference picture index14\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer14_base(&self) -> SwRefer14BaseR {
        SwRefer14BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 14 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref14_field(&mut self) -> SwRef14FieldW<Swreg24H264Refer14BaseSpec> {
        SwRef14FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref14 topfield is used\n\nref14 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref14_topfield_used(&mut self) -> SwRef14TopfieldUsedW<Swreg24H264Refer14BaseSpec> {
        SwRef14TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref14 bottom field is used\n\nref14 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref14_botfield_used(&mut self) -> SwRef14BotfieldUsedW<Swreg24H264Refer14BaseSpec> {
        SwRef14BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref14_colmv_use_flag\n\nsw_ref14_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref14_colmv_use_flag(&mut self) -> SwRef14ColmvUseFlagW<Swreg24H264Refer14BaseSpec> {
        SwRef14ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index14\n\nbase address for reference picture index14\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer14_base(&mut self) -> SwRefer14BaseW<Swreg24H264Refer14BaseSpec> {
        SwRefer14BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_h264_refer14_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_h264_refer14_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg24H264Refer14BaseSpec;
impl crate::RegisterSpec for Swreg24H264Refer14BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg24_h264_refer14_base::R`](R) reader structure"]
impl crate::Readable for Swreg24H264Refer14BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg24_h264_refer14_base::W`](W) writer structure"]
impl crate::Writable for Swreg24H264Refer14BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG24_H264_REFER14_BASE to value 0"]
impl crate::Resettable for Swreg24H264Refer14BaseSpec {
    const RESET_VALUE: u32 = 0;
}
