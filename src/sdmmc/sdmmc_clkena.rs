#[doc = "Register `SDMMC_CLKENA` reader"]
pub type R = crate::R<SdmmcClkenaSpec>;
#[doc = "Register `SDMMC_CLKENA` writer"]
pub type W = crate::W<SdmmcClkenaSpec>;
#[doc = "Clock-enable control for SD card clock and MMC card clock\n\nsupported.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CclkEnable {
    #[doc = "0: clock disabled"]
    B0 = 0,
    #[doc = "1: clock enabled"]
    B1 = 1,
}
impl From<CclkEnable> for bool {
    #[inline(always)]
    fn from(variant: CclkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_ENABLE` reader - Clock-enable control for SD card clock and MMC card clock\n\nsupported."]
pub type CclkEnableR = crate::BitReader<CclkEnable>;
impl CclkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkEnable {
        match self.bits {
            false => CclkEnable::B0,
            true => CclkEnable::B1,
        }
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CclkEnable::B0
    }
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CclkEnable::B1
    }
}
#[doc = "Field `CCLK_ENABLE` writer - Clock-enable control for SD card clock and MMC card clock\n\nsupported."]
pub type CclkEnableW<'a, REG> = crate::BitWriter<'a, REG, CclkEnable>;
impl<'a, REG> CclkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CclkEnable::B0)
    }
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CclkEnable::B1)
    }
}
#[doc = "Low-power control for SD card clock and MMC card clock\n\nsupported.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CclkLowPower {
    #[doc = "0: non-low-power mode"]
    B0 = 0,
    #[doc = "1: low-power mode; stop clock when card in IDLE (should be normally set to only MMC and SD memory cards; for SDIO cards, if interrupts must be detected, clock should not be stopped)."]
    B1 = 1,
}
impl From<CclkLowPower> for bool {
    #[inline(always)]
    fn from(variant: CclkLowPower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_LOW_POWER` reader - Low-power control for SD card clock and MMC card clock\n\nsupported."]
pub type CclkLowPowerR = crate::BitReader<CclkLowPower>;
impl CclkLowPowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkLowPower {
        match self.bits {
            false => CclkLowPower::B0,
            true => CclkLowPower::B1,
        }
    }
    #[doc = "non-low-power mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CclkLowPower::B0
    }
    #[doc = "low-power mode; stop clock when card in IDLE (should be normally set to only MMC and SD memory cards; for SDIO cards, if interrupts must be detected, clock should not be stopped)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CclkLowPower::B1
    }
}
#[doc = "Field `CCLK_LOW_POWER` writer - Low-power control for SD card clock and MMC card clock\n\nsupported."]
pub type CclkLowPowerW<'a, REG> = crate::BitWriter<'a, REG, CclkLowPower>;
impl<'a, REG> CclkLowPowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-low-power mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CclkLowPower::B0)
    }
    #[doc = "low-power mode; stop clock when card in IDLE (should be normally set to only MMC and SD memory cards; for SDIO cards, if interrupts must be detected, clock should not be stopped)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CclkLowPower::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock-enable control for SD card clock and MMC card clock\n\nsupported."]
    #[inline(always)]
    pub fn cclk_enable(&self) -> CclkEnableR {
        CclkEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card clock and MMC card clock\n\nsupported."]
    #[inline(always)]
    pub fn cclk_low_power(&self) -> CclkLowPowerR {
        CclkLowPowerR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card clock and MMC card clock\n\nsupported."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_enable(&mut self) -> CclkEnableW<SdmmcClkenaSpec> {
        CclkEnableW::new(self, 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card clock and MMC card clock\n\nsupported."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_low_power(&mut self) -> CclkLowPowerW<SdmmcClkenaSpec> {
        CclkLowPowerW::new(self, 16)
    }
}
#[doc = "Clock-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcClkenaSpec;
impl crate::RegisterSpec for SdmmcClkenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_clkena::R`](R) reader structure"]
impl crate::Readable for SdmmcClkenaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_clkena::W`](W) writer structure"]
impl crate::Writable for SdmmcClkenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CLKENA to value 0"]
impl crate::Resettable for SdmmcClkenaSpec {
    const RESET_VALUE: u32 = 0;
}
