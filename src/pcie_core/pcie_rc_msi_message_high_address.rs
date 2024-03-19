#[doc = "Register `PCIE_RC_MSI_MESSAGE_HIGH_ADDRESS` reader"]
pub type R = crate::R<PcieRcMsiMessageHighAddressSpec>;
#[doc = "Register `PCIE_RC_MSI_MESSAGE_HIGH_ADDRESS` writer"]
pub type W = crate::W<PcieRcMsiMessageHighAddressSpec>;
#[doc = "Field `MAH` reader - Message Address High \\[MAH\\]\n\nContains bits 63:32 of the 64-bit\n\naddress to be used in MSI Messages.\n\nA value of 0 specifies that 32-bit\n\naddresses are to be used in the\n\nmessages. This field can also be\n\nwritten from the local management\n\nbus."]
pub type MahR = crate::FieldReader<u32>;
#[doc = "Field `MAH` writer - Message Address High \\[MAH\\]\n\nContains bits 63:32 of the 64-bit\n\naddress to be used in MSI Messages.\n\nA value of 0 specifies that 32-bit\n\naddresses are to be used in the\n\nmessages. This field can also be\n\nwritten from the local management\n\nbus."]
pub type MahW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Address High \\[MAH\\]\n\nContains bits 63:32 of the 64-bit\n\naddress to be used in MSI Messages.\n\nA value of 0 specifies that 32-bit\n\naddresses are to be used in the\n\nmessages. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    pub fn mah(&self) -> MahR {
        MahR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Address High \\[MAH\\]\n\nContains bits 63:32 of the 64-bit\n\naddress to be used in MSI Messages.\n\nA value of 0 specifies that 32-bit\n\naddresses are to be used in the\n\nmessages. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    #[must_use]
    pub fn mah(&mut self) -> MahW<PcieRcMsiMessageHighAddressSpec> {
        MahW::new(self, 0)
    }
}
#[doc = "MSI Message High Address Register\n\nContains bits 63:32 of the 64-bit\n\naddress to be used in MSI Messages.\n\nA value of 0 specifies that 32-bit\n\naddresses are to be used in the\n\nmessages. This field can also be\n\nwritten from the local management\n\nbus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_message_high_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_message_high_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcMsiMessageHighAddressSpec;
impl crate::RegisterSpec for PcieRcMsiMessageHighAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_msi_message_high_address::R`](R) reader structure"]
impl crate::Readable for PcieRcMsiMessageHighAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_msi_message_high_address::W`](W) writer structure"]
impl crate::Writable for PcieRcMsiMessageHighAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_MSI_MESSAGE_HIGH_ADDRESS to value 0"]
impl crate::Resettable for PcieRcMsiMessageHighAddressSpec {
    const RESET_VALUE: u32 = 0;
}
