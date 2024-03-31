#[doc = "Register `DMASA` writer"]
pub type W = crate::W<DmasaSpec>;
#[doc = "Field `DMA_SOFTWARE_ACK` writer - This register is use to perform a DMA software acknowledge if a\n\ntransfer needs to be terminated due to an error condition."]
pub type DmaSoftwareAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - This register is use to perform a DMA software acknowledge if a\n\ntransfer needs to be terminated due to an error condition."]
    #[inline(always)]
    #[must_use]
    pub fn dma_software_ack(&mut self) -> DmaSoftwareAckW<DmasaSpec> {
        DmaSoftwareAckW::new(self, 0)
    }
}
#[doc = "DMA Software Acknowledge\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASA to value 0"]
impl crate::Resettable for DmasaSpec {
    const RESET_VALUE: u32 = 0;
}
