#[doc = "Register `MC_LOCKONCLOCK_2` reader"]
pub type R = crate::R<McLockonclock2Spec>;
#[doc = "Register `MC_LOCKONCLOCK_2` writer"]
pub type W = crate::W<McLockonclock2Spec>;
#[doc = "Field `AHBDMACLK` reader - AHB audio DMA clock status. Indicates that the clock is present in the system. Cleared by WR 1 to this position."]
pub type AhbdmaclkR = crate::BitReader;
#[doc = "Field `AHBDMACLK` writer - AHB audio DMA clock status. Indicates that the clock is present in the system. Cleared by WR 1 to this position."]
pub type AhbdmaclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB audio DMA clock status. Indicates that the clock is present in the system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn ahbdmaclk(&self) -> AhbdmaclkR {
        AhbdmaclkR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB audio DMA clock status. Indicates that the clock is present in the system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaclk(&mut self) -> AhbdmaclkW<McLockonclock2Spec> {
        AhbdmaclkW::new(self, 0)
    }
}
#[doc = "AHB audio DMA clock status. Indicates that the clock is present in the system. Cleared by WR 1 to this position.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_lockonclock_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_lockonclock_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McLockonclock2Spec;
impl crate::RegisterSpec for McLockonclock2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_lockonclock_2::R`](R) reader structure"]
impl crate::Readable for McLockonclock2Spec {}
#[doc = "`write(|w| ..)` method takes [`mc_lockonclock_2::W`](W) writer structure"]
impl crate::Writable for McLockonclock2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets MC_LOCKONCLOCK_2 to value 0"]
impl crate::Resettable for McLockonclock2Spec {
    const RESET_VALUE: u8 = 0;
}
