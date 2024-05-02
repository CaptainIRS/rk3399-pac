#[doc = "Register `AWB_PROP` reader"]
pub type R = crate::R<AwbPropSpec>;
#[doc = "Register `AWB_PROP` writer"]
pub type W = crate::W<AwbPropSpec>;
#[doc = "AWB_MODE(1:0):\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AwbMode {
    #[doc = "2: measurement of YCbCr means (AWB_MEAS_MODE = 0) or RGB means (AWB_MEAS_MODE = 1)"]
    B10 = 2,
    #[doc = "0: no measurement"]
    B00 = 0,
}
impl From<AwbMode> for u8 {
    #[inline(always)]
    fn from(variant: AwbMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AwbMode {
    type Ux = u8;
}
#[doc = "Field `AWB_MODE` reader - AWB_MODE(1:0):"]
pub type AwbModeR = crate::FieldReader<AwbMode>;
impl AwbModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AwbMode> {
        match self.bits {
            2 => Some(AwbMode::B10),
            0 => Some(AwbMode::B00),
            _ => None,
        }
    }
    #[doc = "measurement of YCbCr means (AWB_MEAS_MODE = 0) or RGB means (AWB_MEAS_MODE = 1)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AwbMode::B10
    }
    #[doc = "no measurement"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AwbMode::B00
    }
}
#[doc = "Field `AWB_MODE` writer - AWB_MODE(1:0):"]
pub type AwbModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, AwbMode>;
impl<'a, REG> AwbModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "measurement of YCbCr means (AWB_MEAS_MODE = 0) or RGB means (AWB_MEAS_MODE = 1)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AwbMode::B10)
    }
    #[doc = "no measurement"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AwbMode::B00)
    }
}
#[doc = "Field `AWB_MAX_EN` reader - 1: enable Y_MAX\n\ncompare 0: disable Y_MAX compare\n\nNot valid in RGB measurement mode."]
pub type AwbMaxEnR = crate::BitReader;
#[doc = "Field `AWB_MAX_EN` writer - 1: enable Y_MAX\n\ncompare 0: disable Y_MAX compare\n\nNot valid in RGB measurement mode."]
pub type AwbMaxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AwbMeasMode {
    #[doc = "1: RGB based measurement mode"]
    B1 = 1,
    #[doc = "0: near white discrimination mode using YCbCr color space"]
    B0 = 0,
}
impl From<AwbMeasMode> for bool {
    #[inline(always)]
    fn from(variant: AwbMeasMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWB_MEAS_MODE` reader - "]
pub type AwbMeasModeR = crate::BitReader<AwbMeasMode>;
impl AwbMeasModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AwbMeasMode {
        match self.bits {
            true => AwbMeasMode::B1,
            false => AwbMeasMode::B0,
        }
    }
    #[doc = "RGB based measurement mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AwbMeasMode::B1
    }
    #[doc = "near white discrimination mode using YCbCr color space"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AwbMeasMode::B0
    }
}
#[doc = "Field `AWB_MEAS_MODE` writer - "]
pub type AwbMeasModeW<'a, REG> = crate::BitWriter<'a, REG, AwbMeasMode>;
impl<'a, REG> AwbMeasModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB based measurement mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AwbMeasMode::B1)
    }
    #[doc = "near white discrimination mode using YCbCr color space"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AwbMeasMode::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - AWB_MODE(1:0):"]
    #[inline(always)]
    pub fn awb_mode(&self) -> AwbModeR {
        AwbModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1: enable Y_MAX\n\ncompare 0: disable Y_MAX compare\n\nNot valid in RGB measurement mode."]
    #[inline(always)]
    pub fn awb_max_en(&self) -> AwbMaxEnR {
        AwbMaxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn awb_meas_mode(&self) -> AwbMeasModeR {
        AwbMeasModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AWB_MODE(1:0):"]
    #[inline(always)]
    #[must_use]
    pub fn awb_mode(&mut self) -> AwbModeW<AwbPropSpec> {
        AwbModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 1: enable Y_MAX\n\ncompare 0: disable Y_MAX compare\n\nNot valid in RGB measurement mode."]
    #[inline(always)]
    #[must_use]
    pub fn awb_max_en(&mut self) -> AwbMaxEnW<AwbPropSpec> {
        AwbMaxEnW::new(self, 2)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn awb_meas_mode(&mut self) -> AwbMeasModeW<AwbPropSpec> {
        AwbMeasModeW::new(self, 31)
    }
}
#[doc = "Auto white balance properties\n\nNote: The following conversion matrix is used for calculating the YCbCr values: \n\n\n\nY = 16 + 0.2500 R + 0.5000 G + 0.1094 B \n\nCb = 128 - 0.1406 R - 0.2969 G + 0.4375 B \n\nCr = 128 + 0.4375 R - 0.3750 G - 0.0625 B \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_prop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_prop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbPropSpec;
impl crate::RegisterSpec for AwbPropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_prop::R`](R) reader structure"]
impl crate::Readable for AwbPropSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_prop::W`](W) writer structure"]
impl crate::Writable for AwbPropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_PROP to value 0"]
impl crate::Resettable for AwbPropSpec {
    const RESET_VALUE: u32 = 0;
}
