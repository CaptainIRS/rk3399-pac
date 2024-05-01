#[doc = "Register `PWR_UP` reader"]
pub type R = crate::R<PwrUpSpec>;
#[doc = "Register `PWR_UP` writer"]
pub type W = crate::W<PwrUpSpec>;
#[doc = "Field `SHUTDOWNZ` reader - Field0000 Abstract\n\nThis bit configures the core either to power up or to reset.\n\nshutdownz is\n\nthe soft reset register. Its default value is 0. After the core\n\nconfiguration,\n\nto enable the DSI HOST, set this register to 1."]
pub type ShutdownzR = crate::BitReader;
#[doc = "Field `SHUTDOWNZ` writer - Field0000 Abstract\n\nThis bit configures the core either to power up or to reset.\n\nshutdownz is\n\nthe soft reset register. Its default value is 0. After the core\n\nconfiguration,\n\nto enable the DSI HOST, set this register to 1."]
pub type ShutdownzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Field0000 Abstract\n\nThis bit configures the core either to power up or to reset.\n\nshutdownz is\n\nthe soft reset register. Its default value is 0. After the core\n\nconfiguration,\n\nto enable the DSI HOST, set this register to 1."]
    #[inline(always)]
    pub fn shutdownz(&self) -> ShutdownzR {
        ShutdownzR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field0000 Abstract\n\nThis bit configures the core either to power up or to reset.\n\nshutdownz is\n\nthe soft reset register. Its default value is 0. After the core\n\nconfiguration,\n\nto enable the DSI HOST, set this register to 1."]
    #[inline(always)]
    #[must_use]
    pub fn shutdownz(&mut self) -> ShutdownzW<PwrUpSpec> {
        ShutdownzW::new(self, 0)
    }
}
#[doc = "PWR_UP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_up::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_up::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrUpSpec;
impl crate::RegisterSpec for PwrUpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_up::R`](R) reader structure"]
impl crate::Readable for PwrUpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_up::W`](W) writer structure"]
impl crate::Writable for PwrUpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_UP to value 0"]
impl crate::Resettable for PwrUpSpec {
    const RESET_VALUE: u32 = 0;
}
