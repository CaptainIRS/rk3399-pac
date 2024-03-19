#[doc = "Register `PCIE_LM_L0S_TIMEOUT_LIMIT` reader"]
pub type R = crate::R<PcieLmL0sTimeoutLimitSpec>;
#[doc = "Register `PCIE_LM_L0S_TIMEOUT_LIMIT` writer"]
pub type W = crate::W<PcieLmL0sTimeoutLimitSpec>;
#[doc = "Field `LT` reader - L0S Timeout \\[LT\\]
Contains the timeout value (in units of 4ns) for transitioning to the L0S power state. Setting this parameter to 0 permanently disables the transition to the L0S power state."]
pub type LtR = crate::FieldReader<u16>;
#[doc = "Field `LT` writer - L0S Timeout \\[LT\\]
Contains the timeout value (in units of 4ns) for transitioning to the L0S power state. Setting this parameter to 0 permanently disables the transition to the L0S power state."]
pub type LtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - L0S Timeout \\[LT\\]
Contains the timeout value (in units of 4ns) for transitioning to the L0S power state. Setting this parameter to 0 permanently disables the transition to the L0S power state."]
    #[inline(always)]
    pub fn lt(&self) -> LtR {
        LtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L0S Timeout \\[LT\\]
Contains the timeout value (in units of 4ns) for transitioning to the L0S power state. Setting this parameter to 0 permanently disables the transition to the L0S power state."]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LtW<PcieLmL0sTimeoutLimitSpec> {
        LtW::new(self, 0)
    }
}
#[doc = "L0S Timeout Limit Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_l0s_timeout_limit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_l0s_timeout_limit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmL0sTimeoutLimitSpec;
impl crate::RegisterSpec for PcieLmL0sTimeoutLimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_l0s_timeout_limit::R`](R) reader structure"]
impl crate::Readable for PcieLmL0sTimeoutLimitSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_l0s_timeout_limit::W`](W) writer structure"]
impl crate::Writable for PcieLmL0sTimeoutLimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_L0S_TIMEOUT_LIMIT to value 0x02ee"]
impl crate::Resettable for PcieLmL0sTimeoutLimitSpec {
    const RESET_VALUE: u32 = 0x02ee;
}
