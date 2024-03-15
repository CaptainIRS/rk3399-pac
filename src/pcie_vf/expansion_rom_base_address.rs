#[doc = "Register `EXPANSION_ROM_BASE_ADDRESS` reader"]
pub type R = crate::R<ExpansionRomBaseAddressSpec>;
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
#[doc = "Expansion ROM Base Address Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`expansion_rom_base_address::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpansionRomBaseAddressSpec;
impl crate::RegisterSpec for ExpansionRomBaseAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`expansion_rom_base_address::R`](R) reader structure"]
impl crate::Readable for ExpansionRomBaseAddressSpec {}
#[doc = "`reset()` method sets EXPANSION_ROM_BASE_ADDRESS to value 0"]
impl crate::Resettable for ExpansionRomBaseAddressSpec {
    const RESET_VALUE: u32 = 0;
}
