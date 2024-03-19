#[doc = "Register `PCIE_PF_MSI_MESSAGE_LOW_ADDRESS` reader"]
pub type R = crate::R<PciePfMsiMessageLowAddressSpec>;
#[doc = "Register `PCIE_PF_MSI_MESSAGE_LOW_ADDRESS` writer"]
pub type W = crate::W<PciePfMsiMessageLowAddressSpec>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
The two lower bits of the address are hardwired to 0 to align the address on a double-word boundary."]
pub type R1R = crate::FieldReader;
#[doc = "Field `MAL` reader - Message Address Low \\[MAL\\]
Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub type MalR = crate::FieldReader<u32>;
#[doc = "Field `MAL` writer - Message Address Low \\[MAL\\]
Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
pub type MalW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Reserved \\[R1\\]
The two lower bits of the address are hardwired to 0 to align the address on a double-word boundary."]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Message Address Low \\[MAL\\]
Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn mal(&self) -> MalR {
        MalR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Message Address Low \\[MAL\\]
Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn mal(&mut self) -> MalW<PciePfMsiMessageLowAddressSpec> {
        MalW::new(self, 2)
    }
}
#[doc = "MSI Message Low Address Register Lower bits of the address to be used in MSI messages. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_message_low_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_message_low_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiMessageLowAddressSpec;
impl crate::RegisterSpec for PciePfMsiMessageLowAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_message_low_address::R`](R) reader structure"]
impl crate::Readable for PciePfMsiMessageLowAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_msi_message_low_address::W`](W) writer structure"]
impl crate::Writable for PciePfMsiMessageLowAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_MSI_MESSAGE_LOW_ADDRESS to value 0"]
impl crate::Resettable for PciePfMsiMessageLowAddressSpec {
    const RESET_VALUE: u32 = 0;
}
