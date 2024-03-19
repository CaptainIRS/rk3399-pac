#[doc = "Register `PCIE_DMA_CAPABILITY_AND_VERSION` reader"]
pub type R = crate::R<PcieDmaCapabilityAndVersionSpec>;
#[doc = "Field `MIN_VER` reader - min_ver\n\nMinor Version No"]
pub type MinVerR = crate::FieldReader;
#[doc = "Field `MAJ_VER` reader - maj_ver\n\nMajor Version No"]
pub type MajVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - min_ver\n\nMinor Version No"]
    #[inline(always)]
    pub fn min_ver(&self) -> MinVerR {
        MinVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - maj_ver\n\nMajor Version No"]
    #[inline(always)]
    pub fn maj_ver(&self) -> MajVerR {
        MajVerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "PCIe DMA Capability and Version Register\n\nReserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_capability_and_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaCapabilityAndVersionSpec;
impl crate::RegisterSpec for PcieDmaCapabilityAndVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_capability_and_version::R`](R) reader structure"]
impl crate::Readable for PcieDmaCapabilityAndVersionSpec {}
#[doc = "`reset()` method sets PCIE_DMA_CAPABILITY_AND_VERSION to value 0x01"]
impl crate::Resettable for PcieDmaCapabilityAndVersionSpec {
    const RESET_VALUE: u32 = 0x01;
}
