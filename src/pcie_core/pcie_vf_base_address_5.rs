#[doc = "Register `PCIE_VF_BASE_ADDRESS_5` reader"]
pub type R = crate::R<PcieVfBaseAddress5Spec>;
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
#[doc = "Base Address Register 5 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfBaseAddress5Spec;
impl crate::RegisterSpec for PcieVfBaseAddress5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_base_address_5::R`](R) reader structure"]
impl crate::Readable for PcieVfBaseAddress5Spec {}
#[doc = "`reset()` method sets PCIE_VF_BASE_ADDRESS_5 to value 0"]
impl crate::Resettable for PcieVfBaseAddress5Spec {
    const RESET_VALUE: u32 = 0;
}
