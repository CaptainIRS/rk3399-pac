#[doc = "Register `USB3_GTXFIFOPRIHST` reader"]
pub type R = crate::R<Usb3GtxfifoprihstSpec>;
#[doc = "Register `USB3_GTXFIFOPRIHST` writer"]
pub type W = crate::W<Usb3GtxfifoprihstSpec>;
#[doc = "Field `GTXFIFOPRIHST` reader - Host TxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost TXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of TXFIFO\\[n\\]
within a speed group.\n\nWhen multiple TXFIFOs compete for DMA service at a given time\n\n(i.e., multiple TXQs contain TX DMA requests and their\n\ncorresponding TXFIFOs have space available), the TX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority TXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority TXFIFOs are granted access using round-robin\n\narbitration only after the high-priority TXFIFOs have no further\n\nprocessing to do (that is, either the TXQs are empty or the\n\ncorresponding TXFIFOs are full).\n\n2. The TX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
pub type GtxfifoprihstR = crate::FieldReader;
#[doc = "Field `GTXFIFOPRIHST` writer - Host TxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost TXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of TXFIFO\\[n\\]
within a speed group.\n\nWhen multiple TXFIFOs compete for DMA service at a given time\n\n(i.e., multiple TXQs contain TX DMA requests and their\n\ncorresponding TXFIFOs have space available), the TX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority TXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority TXFIFOs are granted access using round-robin\n\narbitration only after the high-priority TXFIFOs have no further\n\nprocessing to do (that is, either the TXQs are empty or the\n\ncorresponding TXFIFOs are full).\n\n2. The TX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
pub type GtxfifoprihstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Host TxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost TXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of TXFIFO\\[n\\]
within a speed group.\n\nWhen multiple TXFIFOs compete for DMA service at a given time\n\n(i.e., multiple TXQs contain TX DMA requests and their\n\ncorresponding TXFIFOs have space available), the TX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority TXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority TXFIFOs are granted access using round-robin\n\narbitration only after the high-priority TXFIFOs have no further\n\nprocessing to do (that is, either the TXQs are empty or the\n\ncorresponding TXFIFOs are full).\n\n2. The TX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
    #[inline(always)]
    pub fn gtxfifoprihst(&self) -> GtxfifoprihstR {
        GtxfifoprihstR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Host TxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost TXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of TXFIFO\\[n\\]
within a speed group.\n\nWhen multiple TXFIFOs compete for DMA service at a given time\n\n(i.e., multiple TXQs contain TX DMA requests and their\n\ncorresponding TXFIFOs have space available), the TX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority TXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority TXFIFOs are granted access using round-robin\n\narbitration only after the high-priority TXFIFOs have no further\n\nprocessing to do (that is, either the TXQs are empty or the\n\ncorresponding TXFIFOs are full).\n\n2. The TX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
    #[inline(always)]
    #[must_use]
    pub fn gtxfifoprihst(&mut self) -> GtxfifoprihstW<Usb3GtxfifoprihstSpec> {
        GtxfifoprihstW::new(self, 0)
    }
}
#[doc = "Global Host TX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifoprihst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifoprihst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GtxfifoprihstSpec;
impl crate::RegisterSpec for Usb3GtxfifoprihstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gtxfifoprihst::R`](R) reader structure"]
impl crate::Readable for Usb3GtxfifoprihstSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gtxfifoprihst::W`](W) writer structure"]
impl crate::Writable for Usb3GtxfifoprihstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GTXFIFOPRIHST to value 0"]
impl crate::Resettable for Usb3GtxfifoprihstSpec {
    const RESET_VALUE: u32 = 0;
}
