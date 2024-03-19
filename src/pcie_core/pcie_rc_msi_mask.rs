#[doc = "Register `PCIE_RC_MSI_MASK` reader"]
pub type R = crate::R<PcieRcMsiMaskSpec>;
#[doc = "Register `PCIE_RC_MSI_MASK` writer"]
pub type W = crate::W<PcieRcMsiMaskSpec>;
#[doc = "Field `MM` reader - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid mask bits."]
pub type MmR = crate::BitReader;
#[doc = "Field `MM` writer - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid mask bits."]
pub type MmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid mask bits."]
    #[inline(always)]
    pub fn mm(&self) -> MmR {
        MmR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid mask bits."]
    #[inline(always)]
    #[must_use]
    pub fn mm(&mut self) -> MmW<PcieRcMsiMaskSpec> {
        MmW::new(self, 0)
    }
}
#[doc = "MSI Mask Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcMsiMaskSpec;
impl crate::RegisterSpec for PcieRcMsiMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_msi_mask::R`](R) reader structure"]
impl crate::Readable for PcieRcMsiMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_msi_mask::W`](W) writer structure"]
impl crate::Writable for PcieRcMsiMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_MSI_MASK to value 0"]
impl crate::Resettable for PcieRcMsiMaskSpec {
    const RESET_VALUE: u32 = 0;
}
