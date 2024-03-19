#[doc = "Register `USB3_GRXFIFOPRIHST` reader"]
pub type R = crate::R<Usb3GrxfifoprihstSpec>;
#[doc = "Register `USB3_GRXFIFOPRIHST` writer"]
pub type W = crate::W<Usb3GrxfifoprihstSpec>;
#[doc = "Field `GRXFIFOPRIHST` reader - Host RxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost RXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of RXFIFO\\[n\\]
within a speed group.\n\nWhen multiple RXFIFOs compete for DMA service at a given time\n\n(i.e., multiple RXQs contain RX DMA requests and their\n\ncorresponding RXFIFOs have data available), the RX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority RXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority RXFIFOs are granted access using round-robin\n\narbitration only after high-priority RXFIFOs have no further\n\nprocessing to do (that is, either the RXQs are empty or the\n\ncorresponding RXFIFOs do not have the required data).\n\n2. The RX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
pub type GrxfifoprihstR = crate::FieldReader;
#[doc = "Field `GRXFIFOPRIHST` writer - Host RxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost RXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of RXFIFO\\[n\\]
within a speed group.\n\nWhen multiple RXFIFOs compete for DMA service at a given time\n\n(i.e., multiple RXQs contain RX DMA requests and their\n\ncorresponding RXFIFOs have data available), the RX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority RXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority RXFIFOs are granted access using round-robin\n\narbitration only after high-priority RXFIFOs have no further\n\nprocessing to do (that is, either the RXQs are empty or the\n\ncorresponding RXFIFOs do not have the required data).\n\n2. The RX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
pub type GrxfifoprihstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Host RxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost RXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of RXFIFO\\[n\\]
within a speed group.\n\nWhen multiple RXFIFOs compete for DMA service at a given time\n\n(i.e., multiple RXQs contain RX DMA requests and their\n\ncorresponding RXFIFOs have data available), the RX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority RXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority RXFIFOs are granted access using round-robin\n\narbitration only after high-priority RXFIFOs have no further\n\nprocessing to do (that is, either the RXQs are empty or the\n\ncorresponding RXFIFOs do not have the required data).\n\n2. The RX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
    #[inline(always)]
    pub fn grxfifoprihst(&self) -> GrxfifoprihstR {
        GrxfifoprihstR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Host RxFIFO priority\n\nThis register specifies the relative DMA priority level among the\n\nHost RXFIFOs (one per USB bus instance) within the associated\n\nspeed group (SS or HS/FSLS). Each register bit\\[n\\]
controls the\n\npriority (1: high, 0: low) of RXFIFO\\[n\\]
within a speed group.\n\nWhen multiple RXFIFOs compete for DMA service at a given time\n\n(i.e., multiple RXQs contain RX DMA requests and their\n\ncorresponding RXFIFOs have data available), the RX DMA arbiter\n\ngrants access on a packet-basis in the following manner:\n\n1. Among the FIFOs in the same speed group (SS or HS/FSLS):\n\na. High-priority RXFIFOs are granted access using round-robin\n\narbitration\n\nb. Low-priority RXFIFOs are granted access using round-robin\n\narbitration only after high-priority RXFIFOs have no further\n\nprocessing to do (that is, either the RXQs are empty or the\n\ncorresponding RXFIFOs do not have the required data).\n\n2. The RX DMA arbiter prioritizes the SS speed group or HS/FSLS\n\nspeed group according to the ratio programmed in the\n\nGDMAHLRATIO register.\n\nFor scatter-gather packets, the arbiter grants successive DMA\n\nrequests to the same FIFO until the entire packet is completed.\n\nThis register is present only when the core is configured to\n\noperate in the host mode (includes DRD and OTG modes). The\n\nregister size corresponds to the number of configured USB bus\n\ninstances; for example, in the default configuration, there are 3\n\nUSB bus instances (1 SS, 1 HS, and 1 FSLS)."]
    #[inline(always)]
    #[must_use]
    pub fn grxfifoprihst(&mut self) -> GrxfifoprihstW<Usb3GrxfifoprihstSpec> {
        GrxfifoprihstW::new(self, 0)
    }
}
#[doc = "Global Host RX FIFO DMA Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_grxfifoprihst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_grxfifoprihst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GrxfifoprihstSpec;
impl crate::RegisterSpec for Usb3GrxfifoprihstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_grxfifoprihst::R`](R) reader structure"]
impl crate::Readable for Usb3GrxfifoprihstSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_grxfifoprihst::W`](W) writer structure"]
impl crate::Writable for Usb3GrxfifoprihstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GRXFIFOPRIHST to value 0"]
impl crate::Resettable for Usb3GrxfifoprihstSpec {
    const RESET_VALUE: u32 = 0;
}
