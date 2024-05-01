#[doc = "Register `SWREG22_H264_REFER12_BASE` reader"]
pub type R = crate::R<Swreg22H264Refer12BaseSpec>;
#[doc = "Register `SWREG22_H264_REFER12_BASE` writer"]
pub type W = crate::W<Swreg22H264Refer12BaseSpec>;
#[doc = "reference 12 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef12Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef12Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef12Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF12_FIELD` reader - reference 12 picture field flag"]
pub type SwRef12FieldR = crate::BitReader<SwRef12Field>;
impl SwRef12FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef12Field {
        match self.bits {
            false => SwRef12Field::B0,
            true => SwRef12Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef12Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef12Field::B1
    }
}
#[doc = "Field `SW_REF12_FIELD` writer - reference 12 picture field flag"]
pub type SwRef12FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef12Field>;
impl<'a, REG> SwRef12FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef12Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef12Field::B1)
    }
}
#[doc = "Field `SW_REF12_TOPFIELD_USED` reader - ref12 topfield is used\n\nref12 topfield is used"]
pub type SwRef12TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF12_TOPFIELD_USED` writer - ref12 topfield is used\n\nref12 topfield is used"]
pub type SwRef12TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF12_BOTFIELD_USED` reader - ref12 bottom field is used\n\nref12 bottom field is used"]
pub type SwRef12BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF12_BOTFIELD_USED` writer - ref12 bottom field is used\n\nref12 bottom field is used"]
pub type SwRef12BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF12_COLMV_USE_FLAG` reader - sw_ref12_colmv_use_flag\n\nsw_ref12_colmv_use_flag"]
pub type SwRef12ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF12_COLMV_USE_FLAG` writer - sw_ref12_colmv_use_flag\n\nsw_ref12_colmv_use_flag"]
pub type SwRef12ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER12_BASE` reader - base address for reference picture index12\n\nbase address for reference picture index12\n\n(the address should be 128bit align)"]
pub type SwRefer12BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER12_BASE` writer - base address for reference picture index12\n\nbase address for reference picture index12\n\n(the address should be 128bit align)"]
pub type SwRefer12BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 12 picture field flag"]
    #[inline(always)]
    pub fn sw_ref12_field(&self) -> SwRef12FieldR {
        SwRef12FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref12 topfield is used\n\nref12 topfield is used"]
    #[inline(always)]
    pub fn sw_ref12_topfield_used(&self) -> SwRef12TopfieldUsedR {
        SwRef12TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref12 bottom field is used\n\nref12 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref12_botfield_used(&self) -> SwRef12BotfieldUsedR {
        SwRef12BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref12_colmv_use_flag\n\nsw_ref12_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref12_colmv_use_flag(&self) -> SwRef12ColmvUseFlagR {
        SwRef12ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index12\n\nbase address for reference picture index12\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer12_base(&self) -> SwRefer12BaseR {
        SwRefer12BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 12 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref12_field(&mut self) -> SwRef12FieldW<Swreg22H264Refer12BaseSpec> {
        SwRef12FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref12 topfield is used\n\nref12 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref12_topfield_used(&mut self) -> SwRef12TopfieldUsedW<Swreg22H264Refer12BaseSpec> {
        SwRef12TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref12 bottom field is used\n\nref12 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref12_botfield_used(&mut self) -> SwRef12BotfieldUsedW<Swreg22H264Refer12BaseSpec> {
        SwRef12BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref12_colmv_use_flag\n\nsw_ref12_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref12_colmv_use_flag(&mut self) -> SwRef12ColmvUseFlagW<Swreg22H264Refer12BaseSpec> {
        SwRef12ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index12\n\nbase address for reference picture index12\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer12_base(&mut self) -> SwRefer12BaseW<Swreg22H264Refer12BaseSpec> {
        SwRefer12BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22_h264_refer12_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22_h264_refer12_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg22H264Refer12BaseSpec;
impl crate::RegisterSpec for Swreg22H264Refer12BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg22_h264_refer12_base::R`](R) reader structure"]
impl crate::Readable for Swreg22H264Refer12BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg22_h264_refer12_base::W`](W) writer structure"]
impl crate::Writable for Swreg22H264Refer12BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG22_H264_REFER12_BASE to value 0"]
impl crate::Resettable for Swreg22H264Refer12BaseSpec {
    const RESET_VALUE: u32 = 0;
}
