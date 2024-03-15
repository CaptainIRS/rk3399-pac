#[doc = "Register `IH_MUTE` reader"]
pub type R = crate::R<IhMuteSpec>;
#[doc = "Register `IH_MUTE` writer"]
pub type W = crate::W<IhMuteSpec>;
#[doc = "Field `MUTE_ALL_INTERRUPT` reader - When set to 1, mutes the main interrupt line (where all interrupts are ORed). The sticky bit interrupts continue with their state; only the main interrupt line is muted."]
pub type MuteAllInterruptR = crate::BitReader;
#[doc = "Field `MUTE_ALL_INTERRUPT` writer - When set to 1, mutes the main interrupt line (where all interrupts are ORed). The sticky bit interrupts continue with their state; only the main interrupt line is muted."]
pub type MuteAllInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE_WAKEUP_INTERRUPT` reader - When set to 1, mutes the main interrupt output port. The sticky bit interrupts continue with their state accessible through the configuration bus, only the main interrupt line is muted."]
pub type MuteWakeupInterruptR = crate::BitReader;
#[doc = "Field `MUTE_WAKEUP_INTERRUPT` writer - When set to 1, mutes the main interrupt output port. The sticky bit interrupts continue with their state accessible through the configuration bus, only the main interrupt line is muted."]
pub type MuteWakeupInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes the main interrupt line (where all interrupts are ORed). The sticky bit interrupts continue with their state; only the main interrupt line is muted."]
    #[inline(always)]
    pub fn mute_all_interrupt(&self) -> MuteAllInterruptR {
        MuteAllInterruptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes the main interrupt output port. The sticky bit interrupts continue with their state accessible through the configuration bus, only the main interrupt line is muted."]
    #[inline(always)]
    pub fn mute_wakeup_interrupt(&self) -> MuteWakeupInterruptR {
        MuteWakeupInterruptR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes the main interrupt line (where all interrupts are ORed). The sticky bit interrupts continue with their state; only the main interrupt line is muted."]
    #[inline(always)]
    #[must_use]
    pub fn mute_all_interrupt(&mut self) -> MuteAllInterruptW<IhMuteSpec> {
        MuteAllInterruptW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes the main interrupt output port. The sticky bit interrupts continue with their state accessible through the configuration bus, only the main interrupt line is muted."]
    #[inline(always)]
    #[must_use]
    pub fn mute_wakeup_interrupt(&mut self) -> MuteWakeupInterruptW<IhMuteSpec> {
        MuteWakeupInterruptW::new(self, 1)
    }
}
#[doc = "When set to 1, mutes the main interrupt line (where all interrupts are ORed). The sticky bit interrupts continue with their state; only the main interrupt line is muted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteSpec;
impl crate::RegisterSpec for IhMuteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute::R`](R) reader structure"]
impl crate::Readable for IhMuteSpec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute::W`](W) writer structure"]
impl crate::Writable for IhMuteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE to value 0x03"]
impl crate::Resettable for IhMuteSpec {
    const RESET_VALUE: u8 = 0x03;
}
