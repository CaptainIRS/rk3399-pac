#[doc = "Register `SWREG21_H264_REFER11_BASE` reader"]
pub type R = crate::R<Swreg21H264Refer11BaseSpec>;
#[doc = "Register `SWREG21_H264_REFER11_BASE` writer"]
pub type W = crate::W<Swreg21H264Refer11BaseSpec>;
#[doc = "reference 11 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef11Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef11Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef11Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF11_FIELD` reader - reference 11 picture field flag"]
pub type SwRef11FieldR = crate::BitReader<SwRef11Field>;
impl SwRef11FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef11Field {
        match self.bits {
            false => SwRef11Field::B0,
            true => SwRef11Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef11Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef11Field::B1
    }
}
#[doc = "Field `SW_REF11_FIELD` writer - reference 11 picture field flag"]
pub type SwRef11FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef11Field>;
impl<'a, REG> SwRef11FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef11Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef11Field::B1)
    }
}
#[doc = "Field `SW_REF11_TOPFIELD_USED` reader - ref11 topfield is used\n\nref11 topfield is used"]
pub type SwRef11TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF11_TOPFIELD_USED` writer - ref11 topfield is used\n\nref11 topfield is used"]
pub type SwRef11TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF11_BOTFIELD_USED` reader - ref11 bottom field is used\n\nref11 bottom field is used"]
pub type SwRef11BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF11_BOTFIELD_USED` writer - ref11 bottom field is used\n\nref11 bottom field is used"]
pub type SwRef11BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF11_COLMV_USE_FLAG` reader - sw_ref11_colmv_use_flag\n\nsw_ref11_colmv_use_flag"]
pub type SwRef11ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF11_COLMV_USE_FLAG` writer - sw_ref11_colmv_use_flag\n\nsw_ref11_colmv_use_flag"]
pub type SwRef11ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER11_BASE` reader - base address for reference picture index11\n\nbase address for reference picture index11\n\n(the address should be 128bit align)"]
pub type SwRefer11BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER11_BASE` writer - base address for reference picture index11\n\nbase address for reference picture index11\n\n(the address should be 128bit align)"]
pub type SwRefer11BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 11 picture field flag"]
    #[inline(always)]
    pub fn sw_ref11_field(&self) -> SwRef11FieldR {
        SwRef11FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref11 topfield is used\n\nref11 topfield is used"]
    #[inline(always)]
    pub fn sw_ref11_topfield_used(&self) -> SwRef11TopfieldUsedR {
        SwRef11TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref11 bottom field is used\n\nref11 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref11_botfield_used(&self) -> SwRef11BotfieldUsedR {
        SwRef11BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref11_colmv_use_flag\n\nsw_ref11_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref11_colmv_use_flag(&self) -> SwRef11ColmvUseFlagR {
        SwRef11ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index11\n\nbase address for reference picture index11\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer11_base(&self) -> SwRefer11BaseR {
        SwRefer11BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 11 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref11_field(&mut self) -> SwRef11FieldW<Swreg21H264Refer11BaseSpec> {
        SwRef11FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref11 topfield is used\n\nref11 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref11_topfield_used(&mut self) -> SwRef11TopfieldUsedW<Swreg21H264Refer11BaseSpec> {
        SwRef11TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref11 bottom field is used\n\nref11 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref11_botfield_used(&mut self) -> SwRef11BotfieldUsedW<Swreg21H264Refer11BaseSpec> {
        SwRef11BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref11_colmv_use_flag\n\nsw_ref11_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref11_colmv_use_flag(&mut self) -> SwRef11ColmvUseFlagW<Swreg21H264Refer11BaseSpec> {
        SwRef11ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index11\n\nbase address for reference picture index11\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer11_base(&mut self) -> SwRefer11BaseW<Swreg21H264Refer11BaseSpec> {
        SwRefer11BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21_h264_refer11_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21_h264_refer11_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg21H264Refer11BaseSpec;
impl crate::RegisterSpec for Swreg21H264Refer11BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg21_h264_refer11_base::R`](R) reader structure"]
impl crate::Readable for Swreg21H264Refer11BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg21_h264_refer11_base::W`](W) writer structure"]
impl crate::Writable for Swreg21H264Refer11BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG21_H264_REFER11_BASE to value 0"]
impl crate::Resettable for Swreg21H264Refer11BaseSpec {
    const RESET_VALUE: u32 = 0;
}
