#[doc = "Register `PCIE_VF_BASE_ADDRESS_3` reader"]
pub type R = crate::R<PcieVfBaseAddress3Spec>;
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
#[doc = "Base Address Register 3 (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfBaseAddress3Spec;
impl crate::RegisterSpec for PcieVfBaseAddress3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_base_address_3::R`](R) reader structure"]
impl crate::Readable for PcieVfBaseAddress3Spec {}
#[doc = "`reset()` method sets PCIE_VF_BASE_ADDRESS_3 to value 0"]
impl crate::Resettable for PcieVfBaseAddress3Spec {
    const RESET_VALUE: u32 = 0;
}
