#[doc = "Register `PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS` reader"]
pub type R = crate::R<PcieDmaInboundBufferUncorrectedEccErrorsSpec>;
#[doc = "Field `total` reader - ECC Error Reg \\[total\\]
ECC Error Detection Register"]
pub type TotalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ECC Error Reg \\[total\\]
ECC Error Detection Register"]
    #[inline(always)]
    pub fn total(&self) -> TotalR {
        TotalR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_uncorrected_ecc_errors::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaInboundBufferUncorrectedEccErrorsSpec;
impl crate::RegisterSpec for PcieDmaInboundBufferUncorrectedEccErrorsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_inbound_buffer_uncorrected_ecc_errors::R`](R) reader structure"]
impl crate::Readable for PcieDmaInboundBufferUncorrectedEccErrorsSpec {}
#[doc = "`reset()` method sets PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS to value 0"]
impl crate::Resettable for PcieDmaInboundBufferUncorrectedEccErrorsSpec {
    const RESET_VALUE: u32 = 0;
}
