#[doc = "Register `PCIE_LM_L1_STATE_RE_ENTRY_DELAY` reader"]
pub type R = crate::R<PcieLmL1StateReEntryDelaySpec>;
#[doc = "Register `PCIE_LM_L1_STATE_RE_ENTRY_DELAY` writer"]
pub type W = crate::W<PcieLmL1StateReEntryDelaySpec>;
#[doc = "Field `L1RD` reader - L1 Re- Entry Delay \\[L1RD\\]
Delay to re-enter L1 after no activity (in units of 4 ns)."]
pub type L1rdR = crate::FieldReader<u32>;
#[doc = "Field `L1RD` writer - L1 Re- Entry Delay \\[L1RD\\]
Delay to re-enter L1 after no activity (in units of 4 ns)."]
pub type L1rdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L1 Re- Entry Delay \\[L1RD\\]
Delay to re-enter L1 after no activity (in units of 4 ns)."]
    #[inline(always)]
    pub fn l1rd(&self) -> L1rdR {
        L1rdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L1 Re- Entry Delay \\[L1RD\\]
Delay to re-enter L1 after no activity (in units of 4 ns)."]
    #[inline(always)]
    #[must_use]
    pub fn l1rd(&mut self) -> L1rdW<PcieLmL1StateReEntryDelaySpec> {
        L1rdW::new(self, 0)
    }
}
#[doc = "L1 State Re-Entry Delay Register Delay to re-enter L1 after no activity (in units of 4 ns).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_l1_state_re_entry_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_l1_state_re_entry_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmL1StateReEntryDelaySpec;
impl crate::RegisterSpec for PcieLmL1StateReEntryDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_l1_state_re_entry_delay::R`](R) reader structure"]
impl crate::Readable for PcieLmL1StateReEntryDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_l1_state_re_entry_delay::W`](W) writer structure"]
impl crate::Writable for PcieLmL1StateReEntryDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_L1_STATE_RE_ENTRY_DELAY to value 0"]
impl crate::Resettable for PcieLmL1StateReEntryDelaySpec {
    const RESET_VALUE: u32 = 0;
}
