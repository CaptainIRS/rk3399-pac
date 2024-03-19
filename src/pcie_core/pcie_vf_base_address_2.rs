#[doc = "Register `PCIE_VF_BASE_ADDRESS_2` reader"]
pub type R = crate::R<PcieVfBaseAddress2Spec>;
#[doc = "Field `NI` reader - Not Implemented \\[NI\\]\n\n(no description)"]
pub type NiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not Implemented \\[NI\\]\n\n(no description)"]
    #[inline(always)]
    pub fn ni(&self) -> NiR {
        NiR::new(self.bits)
    }
}
#[doc = "Base Address Register 2\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_base_address_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfBaseAddress2Spec;
impl crate::RegisterSpec for PcieVfBaseAddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_base_address_2::R`](R) reader structure"]
impl crate::Readable for PcieVfBaseAddress2Spec {}
#[doc = "`reset()` method sets PCIE_VF_BASE_ADDRESS_2 to value 0"]
impl crate::Resettable for PcieVfBaseAddress2Spec {
    const RESET_VALUE: u32 = 0;
}
