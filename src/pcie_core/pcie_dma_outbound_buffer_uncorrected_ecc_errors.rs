#[doc = "Register `PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS` reader"]
pub type R = crate::R<PcieDmaOutboundBufferUncorrectedEccErrorsSpec>;
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
#[doc = "PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_outbound_buffer_uncorrected_ecc_errors::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaOutboundBufferUncorrectedEccErrorsSpec;
impl crate::RegisterSpec for PcieDmaOutboundBufferUncorrectedEccErrorsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_outbound_buffer_uncorrected_ecc_errors::R`](R) reader structure"]
impl crate::Readable for PcieDmaOutboundBufferUncorrectedEccErrorsSpec {}
#[doc = "`reset()` method sets PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS to value 0"]
impl crate::Resettable for PcieDmaOutboundBufferUncorrectedEccErrorsSpec {
    const RESET_VALUE: u32 = 0;
}
