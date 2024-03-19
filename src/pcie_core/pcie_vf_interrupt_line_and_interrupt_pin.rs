#[doc = "Register `PCIE_VF_INTERRUPT_LINE_AND_INTERRUPT_PIN` reader"]
pub type R = crate::R<PcieVfInterruptLineAndInterruptPinSpec>;
#[doc = "Field `NI` reader - Not Implemented \\[NI\\]
(no description)"]
pub type NiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not Implemented \\[NI\\]
(no description)"]
    #[inline(always)]
    pub fn ni(&self) -> NiR {
        NiR::new(self.bits)
    }
}
#[doc = "Interrupt Line and Interrupt Pin Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_interrupt_line_and_interrupt_pin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfInterruptLineAndInterruptPinSpec;
impl crate::RegisterSpec for PcieVfInterruptLineAndInterruptPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_interrupt_line_and_interrupt_pin::R`](R) reader structure"]
impl crate::Readable for PcieVfInterruptLineAndInterruptPinSpec {}
#[doc = "`reset()` method sets PCIE_VF_INTERRUPT_LINE_AND_INTERRUPT_PIN to value 0"]
impl crate::Resettable for PcieVfInterruptLineAndInterruptPinSpec {
    const RESET_VALUE: u32 = 0;
}
