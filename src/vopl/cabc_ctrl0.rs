#[doc = "Register `CABC_CTRL0` reader"]
pub type R = crate::R<CabcCtrl0Spec>;
#[doc = "Register `CABC_CTRL0` writer"]
pub type W = crate::W<CabcCtrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CabcEn {
    #[doc = "0: cabc disable"]
    B0 = 0,
    #[doc = "1: cabc enable"]
    B1 = 1,
}
impl From<CabcEn> for bool {
    #[inline(always)]
    fn from(variant: CabcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CABC_EN` reader - "]
pub type CabcEnR = crate::BitReader<CabcEn>;
impl CabcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CabcEn {
        match self.bits {
            false => CabcEn::B0,
            true => CabcEn::B1,
        }
    }
    #[doc = "cabc disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CabcEn::B0
    }
    #[doc = "cabc enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CabcEn::B1
    }
}
#[doc = "Field `CABC_EN` writer - "]
pub type CabcEnW<'a, REG> = crate::BitWriter<'a, REG, CabcEn>;
impl<'a, REG> CabcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cabc disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CabcEn::B0)
    }
    #[doc = "cabc enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CabcEn::B1)
    }
}
#[doc = "Field `CABC_HANDLE_EN` reader - cabc control pwm"]
pub type CabcHandleEnR = crate::BitReader;
#[doc = "Field `CABC_HANDLE_EN` writer - cabc control pwm"]
pub type CabcHandleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwmConfigMode {
    #[doc = "0: last frame pwm value"]
    B00 = 0,
    #[doc = "1: cur frame pwm value"]
    B01 = 1,
    #[doc = "2: stage by stage"]
    B1x = 2,
}
impl From<PwmConfigMode> for u8 {
    #[inline(always)]
    fn from(variant: PwmConfigMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwmConfigMode {
    type Ux = u8;
}
#[doc = "Field `PWM_CONFIG_MODE` reader - "]
pub type PwmConfigModeR = crate::FieldReader<PwmConfigMode>;
impl PwmConfigModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwmConfigMode> {
        match self.bits {
            0 => Some(PwmConfigMode::B00),
            1 => Some(PwmConfigMode::B01),
            2 => Some(PwmConfigMode::B1x),
            _ => None,
        }
    }
    #[doc = "last frame pwm value"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PwmConfigMode::B00
    }
    #[doc = "cur frame pwm value"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PwmConfigMode::B01
    }
    #[doc = "stage by stage"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == PwmConfigMode::B1x
    }
}
#[doc = "Field `PWM_CONFIG_MODE` writer - "]
pub type PwmConfigModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwmConfigMode>;
impl<'a, REG> PwmConfigModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "last frame pwm value"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PwmConfigMode::B00)
    }
    #[doc = "cur frame pwm value"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PwmConfigMode::B01)
    }
    #[doc = "stage by stage"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(PwmConfigMode::B1x)
    }
}
#[doc = "Field `CABC_CALC_PIXEL_NUM` reader - cabc calc pixel numbers = x % * cabc_total_num"]
pub type CabcCalcPixelNumR = crate::FieldReader<u32>;
#[doc = "Field `CABC_CALC_PIXEL_NUM` writer - cabc calc pixel numbers = x % * cabc_total_num"]
pub type CabcCalcPixelNumW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cabc_en(&self) -> CabcEnR {
        CabcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cabc control pwm"]
    #[inline(always)]
    pub fn cabc_handle_en(&self) -> CabcHandleEnR {
        CabcHandleEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pwm_config_mode(&self) -> PwmConfigModeR {
        PwmConfigModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:26 - cabc calc pixel numbers = x % * cabc_total_num"]
    #[inline(always)]
    pub fn cabc_calc_pixel_num(&self) -> CabcCalcPixelNumR {
        CabcCalcPixelNumR::new((self.bits >> 4) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_en(&mut self) -> CabcEnW<CabcCtrl0Spec> {
        CabcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - cabc control pwm"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_handle_en(&mut self) -> CabcHandleEnW<CabcCtrl0Spec> {
        CabcHandleEnW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_config_mode(&mut self) -> PwmConfigModeW<CabcCtrl0Spec> {
        PwmConfigModeW::new(self, 2)
    }
    #[doc = "Bits 4:26 - cabc calc pixel numbers = x % * cabc_total_num"]
    #[inline(always)]
    #[must_use]
    pub fn cabc_calc_pixel_num(&mut self) -> CabcCalcPixelNumW<CabcCtrl0Spec> {
        CabcCalcPixelNumW::new(self, 4)
    }
}
#[doc = "Content Adaptive Backlight Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcCtrl0Spec;
impl crate::RegisterSpec for CabcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_ctrl0::R`](R) reader structure"]
impl crate::Readable for CabcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cabc_ctrl0::W`](W) writer structure"]
impl crate::Writable for CabcCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_CTRL0 to value 0x00ed_8000"]
impl crate::Resettable for CabcCtrl0Spec {
    const RESET_VALUE: u32 = 0x00ed_8000;
}
