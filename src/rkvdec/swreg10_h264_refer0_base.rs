#[doc = "Register `SWREG10_H264_REFER0_BASE` reader"]
pub type R = crate::R<Swreg10H264Refer0BaseSpec>;
#[doc = "Register `SWREG10_H264_REFER0_BASE` writer"]
pub type W = crate::W<Swreg10H264Refer0BaseSpec>;
#[doc = "reference 0 picture field flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRef0Field {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<SwRef0Field> for bool {
    #[inline(always)]
    fn from(variant: SwRef0Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REF0_FIELD` reader - reference 0 picture field flag"]
pub type SwRef0FieldR = crate::BitReader<SwRef0Field>;
impl SwRef0FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRef0Field {
        match self.bits {
            false => SwRef0Field::B0,
            true => SwRef0Field::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRef0Field::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRef0Field::B1
    }
}
#[doc = "Field `SW_REF0_FIELD` writer - reference 0 picture field flag"]
pub type SwRef0FieldW<'a, REG> = crate::BitWriter<'a, REG, SwRef0Field>;
impl<'a, REG> SwRef0FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef0Field::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRef0Field::B1)
    }
}
#[doc = "Field `SW_REF0_TOPFIELD_USED` reader - top field is used\n\ntop field is used\n\nthe same meaning with ref_valid"]
pub type SwRef0TopfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF0_TOPFIELD_USED` writer - top field is used\n\ntop field is used\n\nthe same meaning with ref_valid"]
pub type SwRef0TopfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF0_BOTFIELD_USED` reader - bottom field is used\n\nbottom field is used\n\nthe same meaning with ref_valid"]
pub type SwRef0BotfieldUsedR = crate::BitReader;
#[doc = "Field `SW_REF0_BOTFIELD_USED` writer - bottom field is used\n\nbottom field is used\n\nthe same meaning with ref_valid"]
pub type SwRef0BotfieldUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REF0_COLMV_USE_FLAG` reader - ref0 colmv use flag\n\nref0 colmv use flag"]
pub type SwRef0ColmvUseFlagR = crate::BitReader;
#[doc = "Field `SW_REF0_COLMV_USE_FLAG` writer - ref0 colmv use flag\n\nref0 colmv use flag"]
pub type SwRef0ColmvUseFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_REFER0_BASE` reader - base address for reference picture index0\n\nbase address for reference picture index0 (the address should be\n\n128bit align)"]
pub type SwRefer0BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER0_BASE` writer - base address for reference picture index0\n\nbase address for reference picture index0 (the address should be\n\n128bit align)"]
pub type SwRefer0BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - reference 0 picture field flag"]
    #[inline(always)]
    pub fn sw_ref0_field(&self) -> SwRef0FieldR {
        SwRef0FieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - top field is used\n\ntop field is used\n\nthe same meaning with ref_valid"]
    #[inline(always)]
    pub fn sw_ref0_topfield_used(&self) -> SwRef0TopfieldUsedR {
        SwRef0TopfieldUsedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - bottom field is used\n\nbottom field is used\n\nthe same meaning with ref_valid"]
    #[inline(always)]
    pub fn sw_ref0_botfield_used(&self) -> SwRef0BotfieldUsedR {
        SwRef0BotfieldUsedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ref0 colmv use flag\n\nref0 colmv use flag"]
    #[inline(always)]
    pub fn sw_ref0_colmv_use_flag(&self) -> SwRef0ColmvUseFlagR {
        SwRef0ColmvUseFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index0\n\nbase address for reference picture index0 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer0_base(&self) -> SwRefer0BaseR {
        SwRefer0BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - reference 0 picture field flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref0_field(&mut self) -> SwRef0FieldW<Swreg10H264Refer0BaseSpec> {
        SwRef0FieldW::new(self, 0)
    }
    #[doc = "Bit 1 - top field is used\n\ntop field is used\n\nthe same meaning with ref_valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref0_topfield_used(&mut self) -> SwRef0TopfieldUsedW<Swreg10H264Refer0BaseSpec> {
        SwRef0TopfieldUsedW::new(self, 1)
    }
    #[doc = "Bit 2 - bottom field is used\n\nbottom field is used\n\nthe same meaning with ref_valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref0_botfield_used(&mut self) -> SwRef0BotfieldUsedW<Swreg10H264Refer0BaseSpec> {
        SwRef0BotfieldUsedW::new(self, 2)
    }
    #[doc = "Bit 3 - ref0 colmv use flag\n\nref0 colmv use flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref0_colmv_use_flag(&mut self) -> SwRef0ColmvUseFlagW<Swreg10H264Refer0BaseSpec> {
        SwRef0ColmvUseFlagW::new(self, 3)
    }
    #[doc = "Bits 4:31 - base address for reference picture index0\n\nbase address for reference picture index0 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer0_base(&mut self) -> SwRefer0BaseW<Swreg10H264Refer0BaseSpec> {
        SwRefer0BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_h264_refer0_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_h264_refer0_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg10H264Refer0BaseSpec;
impl crate::RegisterSpec for Swreg10H264Refer0BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg10_h264_refer0_base::R`](R) reader structure"]
impl crate::Readable for Swreg10H264Refer0BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg10_h264_refer0_base::W`](W) writer structure"]
impl crate::Writable for Swreg10H264Refer0BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG10_H264_REFER0_BASE to value 0"]
impl crate::Resettable for Swreg10H264Refer0BaseSpec {
    const RESET_VALUE: u32 = 0;
}
