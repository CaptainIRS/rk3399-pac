#[doc = "Register `PCIE_VF_EXPANSION_ROM_BASE_ADDRESS` reader"]
pub type R = crate::R<PcieVfExpansionRomBaseAddressSpec>;
#[doc = "Field `NI` reader - Not Implemented \\[NI\\]\n\n(no description)"]
pub type NiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not Implemented \\[NI\\]\n\n(no description)"]
    #[inline(always)]
    pub fn ni(&self) -> NiR {
        NiR::new(self.bits)
    }
}
#[doc = "Expansion ROM Base Address Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_expansion_rom_base_address::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfExpansionRomBaseAddressSpec;
impl crate::RegisterSpec for PcieVfExpansionRomBaseAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_expansion_rom_base_address::R`](R) reader structure"]
impl crate::Readable for PcieVfExpansionRomBaseAddressSpec {}
#[doc = "`reset()` method sets PCIE_VF_EXPANSION_ROM_BASE_ADDRESS to value 0"]
impl crate::Resettable for PcieVfExpansionRomBaseAddressSpec {
    const RESET_VALUE: u32 = 0;
}
