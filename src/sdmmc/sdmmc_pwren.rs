#[doc = "Register `SDMMC_PWREN` reader"]
pub type R = crate::R<SdmmcPwrenSpec>;
#[doc = "Register `SDMMC_PWREN` writer"]
pub type W = crate::W<SdmmcPwrenSpec>;
#[doc = "Power on/off switch for the card. Once power is turned on, firmware should wait for regulator/switch ramp-up time before trying to initialize card.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerEnable {
    #[doc = "0: power on Bit values output to card_power_en port."]
    B0 = 0,
    #[doc = "1: power on Bit values output to card_power_en port."]
    B1 = 1,
}
impl From<PowerEnable> for bool {
    #[inline(always)]
    fn from(variant: PowerEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER_ENABLE` reader - Power on/off switch for the card. Once power is turned on, firmware should wait for regulator/switch ramp-up time before trying to initialize card."]
pub type PowerEnableR = crate::BitReader<PowerEnable>;
impl PowerEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerEnable {
        match self.bits {
            false => PowerEnable::B0,
            true => PowerEnable::B1,
        }
    }
    #[doc = "power on Bit values output to card_power_en port."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PowerEnable::B0
    }
    #[doc = "power on Bit values output to card_power_en port."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PowerEnable::B1
    }
}
#[doc = "Field `POWER_ENABLE` writer - Power on/off switch for the card. Once power is turned on, firmware should wait for regulator/switch ramp-up time before trying to initialize card."]
pub type PowerEnableW<'a, REG> = crate::BitWriter<'a, REG, PowerEnable>;
impl<'a, REG> PowerEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "power on Bit values output to card_power_en port."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEnable::B0)
    }
    #[doc = "power on Bit values output to card_power_en port."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEnable::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Power on/off switch for the card. Once power is turned on, firmware should wait for regulator/switch ramp-up time before trying to initialize card."]
    #[inline(always)]
    pub fn power_enable(&self) -> PowerEnableR {
        PowerEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on/off switch for the card. Once power is turned on, firmware should wait for regulator/switch ramp-up time before trying to initialize card."]
    #[inline(always)]
    #[must_use]
    pub fn power_enable(&mut self) -> PowerEnableW<SdmmcPwrenSpec> {
        PowerEnableW::new(self, 0)
    }
}
#[doc = "Power-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_pwren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_pwren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcPwrenSpec;
impl crate::RegisterSpec for SdmmcPwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_pwren::R`](R) reader structure"]
impl crate::Readable for SdmmcPwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_pwren::W`](W) writer structure"]
impl crate::Writable for SdmmcPwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_PWREN to value 0"]
impl crate::Resettable for SdmmcPwrenSpec {
    const RESET_VALUE: u32 = 0;
}
