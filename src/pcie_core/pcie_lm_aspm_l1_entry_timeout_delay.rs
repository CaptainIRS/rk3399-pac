#[doc = "Register `PCIE_LM_ASPM_L1_ENTRY_TIMEOUT_DELAY` reader"]
pub type R = crate::R<PcieLmAspmL1EntryTimeoutDelaySpec>;
#[doc = "Register `PCIE_LM_ASPM_L1_ENTRY_TIMEOUT_DELAY` writer"]
pub type W = crate::W<PcieLmAspmL1EntryTimeoutDelaySpec>;
#[doc = "Field `L1T` reader - L1 Timeout \\[L1T\\]\n\nContains the timeout value (in units\n\nof 4 ns) for transitioning to the L1\n\npower state. Setting it to 0\n\npermanently disables the transition\n\nto the L1 power state."]
pub type L1tR = crate::FieldReader<u32>;
#[doc = "Field `L1T` writer - L1 Timeout \\[L1T\\]\n\nContains the timeout value (in units\n\nof 4 ns) for transitioning to the L1\n\npower state. Setting it to 0\n\npermanently disables the transition\n\nto the L1 power state."]
pub type L1tW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `R7` reader - Reserved \\[R7\\]\n\nReserved"]
pub type R7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - L1 Timeout \\[L1T\\]\n\nContains the timeout value (in units\n\nof 4 ns) for transitioning to the L1\n\npower state. Setting it to 0\n\npermanently disables the transition\n\nto the L1 power state."]
    #[inline(always)]
    pub fn l1t(&self) -> L1tR {
        L1tR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Reserved \\[R7\\]\n\nReserved"]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - L1 Timeout \\[L1T\\]\n\nContains the timeout value (in units\n\nof 4 ns) for transitioning to the L1\n\npower state. Setting it to 0\n\npermanently disables the transition\n\nto the L1 power state."]
    #[inline(always)]
    #[must_use]
    pub fn l1t(&mut self) -> L1tW<PcieLmAspmL1EntryTimeoutDelaySpec> {
        L1tW::new(self, 0)
    }
}
#[doc = "ASPM L1 Entry Timeout Delay Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_aspm_l1_entry_timeout_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_aspm_l1_entry_timeout_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmAspmL1EntryTimeoutDelaySpec;
impl crate::RegisterSpec for PcieLmAspmL1EntryTimeoutDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_aspm_l1_entry_timeout_delay::R`](R) reader structure"]
impl crate::Readable for PcieLmAspmL1EntryTimeoutDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_aspm_l1_entry_timeout_delay::W`](W) writer structure"]
impl crate::Writable for PcieLmAspmL1EntryTimeoutDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_ASPM_L1_ENTRY_TIMEOUT_DELAY to value 0x05dc"]
impl crate::Resettable for PcieLmAspmL1EntryTimeoutDelaySpec {
    const RESET_VALUE: u32 = 0x05dc;
}
