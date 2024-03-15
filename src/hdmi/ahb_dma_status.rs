#[doc = "Register `AHB_DMA_STATUS` reader"]
pub type R = crate::R<AhbDmaStatusSpec>;
#[doc = "Field `AUTOSTART_STATUS` reader - Indicates the set of start and stop addresses currently used by the AHB audio DMA. If cleared (1'b0), the start and stop addresses configured in the address range 0x3604 to 0x360B are being used. When set (1'b1), the configurations at address range 0x3620 to 0x3627 are being used. This bit is always at zero when autostart_enable is cleared (1'b0)."]
pub type AutostartStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the set of start and stop addresses currently used by the AHB audio DMA. If cleared (1'b0), the start and stop addresses configured in the address range 0x3604 to 0x360B are being used. When set (1'b1), the configurations at address range 0x3620 to 0x3627 are being used. This bit is always at zero when autostart_enable is cleared (1'b0)."]
    #[inline(always)]
    pub fn autostart_status(&self) -> AutostartStatusR {
        AutostartStatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Indicates the set of start and stop addresses currently used by the AHB audio DMA. If cleared (1'b0), the start and stop addresses configured in the address range 0x3604 to 0x360B are being used. When set (1'b1), the configurations at address range 0x3620 to 0x3627 are being used. This bit is always at zero when autostart_enable is cleared (1'b0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaStatusSpec;
impl crate::RegisterSpec for AhbDmaStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_status::R`](R) reader structure"]
impl crate::Readable for AhbDmaStatusSpec {}
#[doc = "`reset()` method sets AHB_DMA_STATUS to value 0"]
impl crate::Resettable for AhbDmaStatusSpec {
    const RESET_VALUE: u8 = 0;
}
