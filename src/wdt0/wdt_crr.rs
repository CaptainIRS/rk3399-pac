#[doc = "Register `WDT_CRR` reader"]
pub type R = crate::R<WdtCrrSpec>;
#[doc = "Register `WDT_CRR` writer"]
pub type W = crate::W<WdtCrrSpec>;
#[doc = "Field `CNT_RESTART` reader - Counter restart This register is used to restart the WDT counter. As a safety feature to prevent accidental restarts, the value 0x76 must be written. A restart also clears the WDT interrupt. Reading this register returns zero."]
pub type CntRestartR = crate::FieldReader;
#[doc = "Field `CNT_RESTART` writer - Counter restart This register is used to restart the WDT counter. As a safety feature to prevent accidental restarts, the value 0x76 must be written. A restart also clears the WDT interrupt. Reading this register returns zero."]
pub type CntRestartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter restart This register is used to restart the WDT counter. As a safety feature to prevent accidental restarts, the value 0x76 must be written. A restart also clears the WDT interrupt. Reading this register returns zero."]
    #[inline(always)]
    pub fn cnt_restart(&self) -> CntRestartR {
        CntRestartR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter restart This register is used to restart the WDT counter. As a safety feature to prevent accidental restarts, the value 0x76 must be written. A restart also clears the WDT interrupt. Reading this register returns zero."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_restart(&mut self) -> CntRestartW<WdtCrrSpec> {
        CntRestartW::new(self, 0)
    }
}
#[doc = "Counter restart Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_crr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_crr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCrrSpec;
impl crate::RegisterSpec for WdtCrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_crr::R`](R) reader structure"]
impl crate::Readable for WdtCrrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_crr::W`](W) writer structure"]
impl crate::Writable for WdtCrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets WDT_CRR to value 0"]
impl crate::Resettable for WdtCrrSpec {
    const RESET_VALUE: u32 = 0;
}
