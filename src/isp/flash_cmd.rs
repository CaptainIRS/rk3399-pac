#[doc = "Register `FLASH_CMD` writer"]
pub type W = crate::W<FlashCmdSpec>;
#[doc = "Field `prelight_on` writer - prelight on\n\n0: prelight is switched off at next trigger event 1:\n\nprelight is switched on at next trigger event"]
pub type PrelightOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "flash on\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashOn {
    #[doc = "0: no effect"]
    B0 = 0,
    #[doc = "1: flash delay counter is started at next trigger event A capture event is signaled to the sensor interface block."]
    B1 = 1,
}
impl From<FlashOn> for bool {
    #[inline(always)]
    fn from(variant: FlashOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `flash_on` writer - flash on"]
pub type FlashOnW<'a, REG> = crate::BitWriter<'a, REG, FlashOn>;
impl<'a, REG> FlashOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FlashOn::B0)
    }
    #[doc = "flash delay counter is started at next trigger event A capture event is signaled to the sensor interface block."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FlashOn::B1)
    }
}
#[doc = "preflash on\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreflashOn {
    #[doc = "0: no effect"]
    B0 = 0,
    #[doc = "1: flash delay counter is started at next trigger event No capture event is signaled to the sensor interface block."]
    B1 = 1,
}
impl From<PreflashOn> for bool {
    #[inline(always)]
    fn from(variant: PreflashOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `preflash_on` writer - preflash on"]
pub type PreflashOnW<'a, REG> = crate::BitWriter<'a, REG, PreflashOn>;
impl<'a, REG> PreflashOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PreflashOn::B0)
    }
    #[doc = "flash delay counter is started at next trigger event No capture event is signaled to the sensor interface block."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PreflashOn::B1)
    }
}
impl W {
    #[doc = "Bit 0 - prelight on\n\n0: prelight is switched off at next trigger event 1:\n\nprelight is switched on at next trigger event"]
    #[inline(always)]
    #[must_use]
    pub fn prelight_on(&mut self) -> PrelightOnW<FlashCmdSpec> {
        PrelightOnW::new(self, 0)
    }
    #[doc = "Bit 1 - flash on"]
    #[inline(always)]
    #[must_use]
    pub fn flash_on(&mut self) -> FlashOnW<FlashCmdSpec> {
        FlashOnW::new(self, 1)
    }
    #[doc = "Bit 2 - preflash on"]
    #[inline(always)]
    #[must_use]
    pub fn preflash_on(&mut self) -> PreflashOnW<FlashCmdSpec> {
        PreflashOnW::new(self, 2)
    }
}
#[doc = "Flash command\n\nNote: This is the command register for flash light and prelight activation. If the 'rw' bits \n\n(e.g. 'fl_cap_del') are re-programmed during operation, the following scheme shall be \n\napplied: \n\n\n\nprelight is active (prelight_on = 1 has been set before): Every write access to this register \n\nshall use prelight_on = 1 (to prevent undesired switch off of the prelight). \n\n\n\nprelight is off: Every write access to this register shall use prelight_on = 0 (to prevent \n\nundesired switch on of the prelight). \n\n\n\n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCmdSpec;
impl crate::RegisterSpec for FlashCmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_cmd::W`](W) writer structure"]
impl crate::Writable for FlashCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_CMD to value 0"]
impl crate::Resettable for FlashCmdSpec {
    const RESET_VALUE: u32 = 0;
}
