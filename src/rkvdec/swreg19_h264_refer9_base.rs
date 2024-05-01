#[doc = "Register `SWREG19_H264_REFER9_BASE` reader"]
pub type R = crate::R<Swreg19H264Refer9BaseSpec>;
#[doc = "Register `SWREG19_H264_REFER9_BASE` writer"]
pub type W = crate::W<Swreg19H264Refer9BaseSpec>;
#[doc = "reference 9 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef9Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef9Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef9Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF9_FIELD` reader - reference 9 picture field flag"]
pub type SwRef9FieldR = crate::BitReader<SwRef9Field>;
impl SwRef9FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef9Field {
        match self.bits {
            false => SwRef9Field::B0,
            true => SwRef9Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef9Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef9Field::B1
    }
}
#[doc = "Field `SW_REF9_FIELD` writer - reference 9 picture field flag"]
pub type SwRef9FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef9Field>;
impl<'a, REG> SwRef9FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef9Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef9Field::B1)
    }
}
#[doc = "Field `SW_REF9_TOPFIELD_USED` reader - ref9 topfield is used\n\nref9 topfield is used"]
pub type SwRef9TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF9_TOPFIELD_USED` writer - ref9 topfield is used\n\nref9 topfield is used"]
pub type SwRef9TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF9_BOTFIELD_USED` reader - ref9 bottom field is used\n\nref9 bottom field is used"]
pub type SwRef9BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF9_BOTFIELD_USED` writer - ref9 bottom field is used\n\nref9 bottom field is used"]
pub type SwRef9BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF9_COLMV_USE_FLAG` reader - sw_ref9_colmv_use_flag\n\nsw_ref9_colmv_use_flag"]
pub type SwRef9ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF9_COLMV_USE_FLAG` writer - sw_ref9_colmv_use_flag\n\nsw_ref9_colmv_use_flag"]
pub type SwRef9ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER9_BASE` reader - base address for reference picture index9\n\nbase address for reference picture index9\n\n(the address should be 128bit align)"]
pub type SwRefer9BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER9_BASE` writer - base address for reference picture index9\n\nbase address for reference picture index9\n\n(the address should be 128bit align)"]
pub type SwRefer9BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 9 picture field flag"]
    #[inline(always)]
    pub fn sw_ref9_field(&self) -> SwRef9FieldR {
        SwRef9FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ref9 topfield is used\n\nref9 topfield is used"]
    #[inline(always)]
    pub fn sw_ref9_topfield_used(&self) -> SwRef9TopfieldUsedR {
        SwRef9TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ref9 bottom field is used\n\nref9 bottom field is used"]
    #[inline(always)]
    pub fn sw_ref9_botfield_used(&self) -> SwRef9BotfieldUsedR {
        SwRef9BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_ref9_colmv_use_flag\n\nsw_ref9_colmv_use_flag"]
    #[inline(always)]
    pub fn sw_ref9_colmv_use_flag(&self) -> SwRef9ColmvUseFlagR {
        SwRef9ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index9\n\nbase address for reference picture index9\n\n(the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_refer9_base(&self) -> SwRefer9BaseR {
        SwRefer9BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 9 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref9_field(&mut self) -> SwRef9FieldW<Swreg19H264Refer9BaseSpec> {
        SwRef9FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - ref9 topfield is used\n\nref9 topfield is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref9_topfield_used(&mut self) -> SwRef9TopfieldUsedW<Swreg19H264Refer9BaseSpec> {
        SwRef9TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - ref9 bottom field is used\n\nref9 bottom field is used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref9_botfield_used(&mut self) -> SwRef9BotfieldUsedW<Swreg19H264Refer9BaseSpec> {
        SwRef9BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_ref9_colmv_use_flag\n\nsw_ref9_colmv_use_flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref9_colmv_use_flag(&mut self) -> SwRef9ColmvUseFlagW<Swreg19H264Refer9BaseSpec> {
        SwRef9ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index9\n\nbase address for reference picture index9\n\n(the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer9_base(&mut self) -> SwRefer9BaseW<Swreg19H264Refer9BaseSpec> {
        SwRefer9BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_h264_refer9_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_h264_refer9_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg19H264Refer9BaseSpec;
impl crate::RegisterSpec for Swreg19H264Refer9BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg19_h264_refer9_base::R`](R) reader structure"]
impl crate::Readable for Swreg19H264Refer9BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg19_h264_refer9_base::W`](W) writer structure"]
impl crate::Writable for Swreg19H264Refer9BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG19_H264_REFER9_BASE to value 0"]
impl crate::Resettable for Swreg19H264Refer9BaseSpec {
    const RESET_VALUE: u32 = 0;
}
