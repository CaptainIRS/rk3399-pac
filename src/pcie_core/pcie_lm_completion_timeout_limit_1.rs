#[doc = "Register `PCIE_LM_COMPLETION_TIMEOUT_LIMIT_1` reader"]
pub type R = crate::R<PcieLmCompletionTimeoutLimit1Spec>;
#[doc = "Register `PCIE_LM_COMPLETION_TIMEOUT_LIMIT_1` writer"]
pub type W = crate::W<PcieLmCompletionTimeoutLimit1Spec>;
#[doc = "Field `CTL` reader - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
pub type CtlR = crate::FieldReader<u32>;
#[doc = "Field `CTL` writer - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
pub type CtlW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `R6` reader - Reserved \\[R6\\]
Reserved"]
pub type R6R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Reserved \\[R6\\]
Reserved"]
    #[inline(always)]
    pub fn r6(&self) -> R6R {
        R6R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Completion Timeout Limit \\[CTL\\]
Timeout limit for completion timers (in 4 ns cycles)."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<PcieLmCompletionTimeoutLimit1Spec> {
        CtlW::new(self, 0)
    }
}
#[doc = "Completion Timeout Limit Register 1 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_completion_timeout_limit_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_completion_timeout_limit_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmCompletionTimeoutLimit1Spec;
impl crate::RegisterSpec for PcieLmCompletionTimeoutLimit1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_completion_timeout_limit_1::R`](R) reader structure"]
impl crate::Readable for PcieLmCompletionTimeoutLimit1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_completion_timeout_limit_1::W`](W) writer structure"]
impl crate::Writable for PcieLmCompletionTimeoutLimit1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_COMPLETION_TIMEOUT_LIMIT_1 to value 0x02fa_f080"]
impl crate::Resettable for PcieLmCompletionTimeoutLimit1Spec {
    const RESET_VALUE: u32 = 0x02fa_f080;
}
