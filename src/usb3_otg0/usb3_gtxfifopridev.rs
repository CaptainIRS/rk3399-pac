#[doc = "Register `USB3_GTXFIFOPRIDEV` reader"]
pub type R = crate::R<Usb3GtxfifopridevSpec>;
#[doc = "Register `USB3_GTXFIFOPRIDEV` writer"]
pub type W = crate::W<Usb3GtxfifopridevSpec>;
#[doc = "Field `GTXFIFOPRIDEV` reader - Device TxFIFO priority This register specifies the relative DMA priority level among the Device TXFIFOs (one per IN endpoint). Each register bit\\[n\\]
controls the priority (1: high, 0: low) of each TXFIFO\\[n\\]. When multiple TXFIFOs compete for DMA service at a given time (that is, multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner: 1. High-priority TXFIFOs are granted access using round-robin arbitration 2. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full). For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed. When configuring periodic IN endpoints, software must set register bit\\[n\\]=1, where n is the TXFIFO assignment. This ensures that the DMA for isochronous or interrupt IN endpoints are prioritized over bulk or control IN endpoints. This register is present only when the core is configured to operate in the device mode. The register size corresponds to the number of Device IN endpoints."]
pub type GtxfifopridevR = crate::FieldReader;
#[doc = "Field `GTXFIFOPRIDEV` writer - Device TxFIFO priority This register specifies the relative DMA priority level among the Device TXFIFOs (one per IN endpoint). Each register bit\\[n\\]
controls the priority (1: high, 0: low) of each TXFIFO\\[n\\]. When multiple TXFIFOs compete for DMA service at a given time (that is, multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner: 1. High-priority TXFIFOs are granted access using round-robin arbitration 2. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full). For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed. When configuring periodic IN endpoints, software must set register bit\\[n\\]=1, where n is the TXFIFO assignment. This ensures that the DMA for isochronous or interrupt IN endpoints are prioritized over bulk or control IN endpoints. This register is present only when the core is configured to operate in the device mode. The register size corresponds to the number of Device IN endpoints."]
pub type GtxfifopridevW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Device TxFIFO priority This register specifies the relative DMA priority level among the Device TXFIFOs (one per IN endpoint). Each register bit\\[n\\]
controls the priority (1: high, 0: low) of each TXFIFO\\[n\\]. When multiple TXFIFOs compete for DMA service at a given time (that is, multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner: 1. High-priority TXFIFOs are granted access using round-robin arbitration 2. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full). For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed. When configuring periodic IN endpoints, software must set register bit\\[n\\]=1, where n is the TXFIFO assignment. This ensures that the DMA for isochronous or interrupt IN endpoints are prioritized over bulk or control IN endpoints. This register is present only when the core is configured to operate in the device mode. The register size corresponds to the number of Device IN endpoints."]
    #[inline(always)]
    pub fn gtxfifopridev(&self) -> GtxfifopridevR {
        GtxfifopridevR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device TxFIFO priority This register specifies the relative DMA priority level among the Device TXFIFOs (one per IN endpoint). Each register bit\\[n\\]
controls the priority (1: high, 0: low) of each TXFIFO\\[n\\]. When multiple TXFIFOs compete for DMA service at a given time (that is, multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner: 1. High-priority TXFIFOs are granted access using round-robin arbitration 2. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full). For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed. When configuring periodic IN endpoints, software must set register bit\\[n\\]=1, where n is the TXFIFO assignment. This ensures that the DMA for isochronous or interrupt IN endpoints are prioritized over bulk or control IN endpoints. This register is present only when the core is configured to operate in the device mode. The register size corresponds to the number of Device IN endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn gtxfifopridev(&mut self) -> GtxfifopridevW<Usb3GtxfifopridevSpec> {
        GtxfifopridevW::new(self, 0)
    }
}
#[doc = "Global Device TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifopridev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifopridev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GtxfifopridevSpec;
impl crate::RegisterSpec for Usb3GtxfifopridevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gtxfifopridev::R`](R) reader structure"]
impl crate::Readable for Usb3GtxfifopridevSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gtxfifopridev::W`](W) writer structure"]
impl crate::Writable for Usb3GtxfifopridevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GTXFIFOPRIDEV to value 0"]
impl crate::Resettable for Usb3GtxfifopridevSpec {
    const RESET_VALUE: u32 = 0;
}
