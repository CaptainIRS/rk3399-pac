#[doc = "Register `DPCC_OUTPUT_MODE` reader"]
pub type R = crate::R<DpccOutputModeSpec>;
#[doc = "Register `DPCC_OUTPUT_MODE` writer"]
pub type W = crate::W<DpccOutputModeSpec>;
#[doc = "Field `STAGE1_INCL_RB_CENTER` reader - 1: stage1 include center pixel for red/blue output\n\nmedian 2x2+1\n\n*Default* 0: stage1 do not include center pixel\n\nfor red/blue output median 2x2"]
pub type Stage1InclRbCenterR = crate::BitReader;
#[doc = "Field `STAGE1_INCL_RB_CENTER` writer - 1: stage1 include center pixel for red/blue output\n\nmedian 2x2+1\n\n*Default* 0: stage1 do not include center pixel\n\nfor red/blue output median 2x2"]
pub type Stage1InclRbCenterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1G3x3 {
    #[doc = "1: stage1 green 9 pixel (3x3) output median"]
    B1 = 1,
    #[doc = "0: stage1 green 4 or 5 pixel output median *Default*"]
    B0 = 0,
}
impl From<Stage1G3x3> for bool {
    #[inline(always)]
    fn from(variant: Stage1G3x3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_G_3x3` reader - "]
pub type Stage1G3x3R = crate::BitReader<Stage1G3x3>;
impl Stage1G3x3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1G3x3 {
        match self.bits {
            true => Stage1G3x3::B1,
            false => Stage1G3x3::B0,
        }
    }
    #[doc = "stage1 green 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1G3x3::B1
    }
    #[doc = "stage1 green 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1G3x3::B0
    }
}
#[doc = "Field `STAGE1_G_3x3` writer - "]
pub type Stage1G3x3W<'a, REG> = crate::BitWriter<'a, REG, Stage1G3x3>;
impl<'a, REG> Stage1G3x3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stage1 green 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1G3x3::B1)
    }
    #[doc = "stage1 green 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1G3x3::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1Rb3x3 {
    #[doc = "1: stage1 red/blue 9 pixel (3x3) output median"]
    B1 = 1,
    #[doc = "0: stage1 red/blue 4 or 5 pixel output median *Default*"]
    B0 = 0,
}
impl From<Stage1Rb3x3> for bool {
    #[inline(always)]
    fn from(variant: Stage1Rb3x3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_RB_3x3` reader - "]
pub type Stage1Rb3x3R = crate::BitReader<Stage1Rb3x3>;
impl Stage1Rb3x3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1Rb3x3 {
        match self.bits {
            true => Stage1Rb3x3::B1,
            false => Stage1Rb3x3::B0,
        }
    }
    #[doc = "stage1 red/blue 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1Rb3x3::B1
    }
    #[doc = "stage1 red/blue 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1Rb3x3::B0
    }
}
#[doc = "Field `STAGE1_RB_3x3` writer - "]
pub type Stage1Rb3x3W<'a, REG> = crate::BitWriter<'a, REG, Stage1Rb3x3>;
impl<'a, REG> Stage1Rb3x3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stage1 red/blue 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1Rb3x3::B1)
    }
    #[doc = "stage1 red/blue 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1Rb3x3::B0)
    }
}
impl R {
    #[doc = "Bit 1 - 1: stage1 include center pixel for red/blue output\n\nmedian 2x2+1\n\n*Default* 0: stage1 do not include center pixel\n\nfor red/blue output median 2x2"]
    #[inline(always)]
    pub fn stage1_incl_rb_center(&self) -> Stage1InclRbCenterR {
        Stage1InclRbCenterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stage1_g_3x3(&self) -> Stage1G3x3R {
        Stage1G3x3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stage1_rb_3x3(&self) -> Stage1Rb3x3R {
        Stage1Rb3x3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1: stage1 include center pixel for red/blue output\n\nmedian 2x2+1\n\n*Default* 0: stage1 do not include center pixel\n\nfor red/blue output median 2x2"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_incl_rb_center(&mut self) -> Stage1InclRbCenterW<DpccOutputModeSpec> {
        Stage1InclRbCenterW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_g_3x3(&mut self) -> Stage1G3x3W<DpccOutputModeSpec> {
        Stage1G3x3W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_rb_3x3(&mut self) -> Stage1Rb3x3W<DpccOutputModeSpec> {
        Stage1Rb3x3W::new(self, 3)
    }
}
#[doc = "Interpolation mode for correction unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_output_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_output_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccOutputModeSpec;
impl crate::RegisterSpec for DpccOutputModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_output_mode::R`](R) reader structure"]
impl crate::Readable for DpccOutputModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_output_mode::W`](W) writer structure"]
impl crate::Writable for DpccOutputModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_OUTPUT_MODE to value 0"]
impl crate::Resettable for DpccOutputModeSpec {
    const RESET_VALUE: u32 = 0;
}
