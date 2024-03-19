#[doc = "Register `PCIE_PF_MSI_MESSAGE_HIGH_ADDRESS` reader"]
pub type R = crate::R<PciePfMsiMessageHighAddressSpec>;
#[doc = "Register `PCIE_PF_MSI_MESSAGE_HIGH_ADDRESS` writer"]
pub type W = crate::W<PciePfMsiMessageHighAddressSpec>;
#[doc = "Field `MAH` reader - Message Address High \\[MAH\\]
Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub type MahR = crate::FieldReader<u32>;
#[doc = "Field `MAH` writer - Message Address High \\[MAH\\]
Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
pub type MahW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Address High \\[MAH\\]
Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn mah(&self) -> MahR {
        MahR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Address High \\[MAH\\]
Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn mah(&mut self) -> MahW<PciePfMsiMessageHighAddressSpec> {
        MahW::new(self, 0)
    }
}
#[doc = "MSI Message High Address Register Contains bits 63:32 of the 64-bit address to be used in MSI Messages. A value of 0 specifies that 32-bit addresses are to be used in the messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_message_high_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_message_high_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiMessageHighAddressSpec;
impl crate::RegisterSpec for PciePfMsiMessageHighAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_message_high_address::R`](R) reader structure"]
impl crate::Readable for PciePfMsiMessageHighAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_msi_message_high_address::W`](W) writer structure"]
impl crate::Writable for PciePfMsiMessageHighAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_MSI_MESSAGE_HIGH_ADDRESS to value 0"]
impl crate::Resettable for PciePfMsiMessageHighAddressSpec {
    const RESET_VALUE: u32 = 0;
}
