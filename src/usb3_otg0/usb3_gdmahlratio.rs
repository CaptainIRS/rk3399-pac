#[doc = "Register `USB3_GDMAHLRATIO` reader"]
pub type R = crate::R<Usb3GdmahlratioSpec>;
#[doc = "Register `USB3_GDMAHLRATIO` writer"]
pub type W = crate::W<Usb3GdmahlratioSpec>;
#[doc = "Field `HSTTXFIFO` reader - Host TXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
pub type HsttxfifoR = crate::FieldReader;
#[doc = "Field `HSTTXFIFO` writer - Host TXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
pub type HsttxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HSTRXFIFO` reader - Host RXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
pub type HstrxfifoR = crate::FieldReader;
#[doc = "Field `HSTRXFIFO` writer - Host RXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
pub type HstrxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Host TXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
    #[inline(always)]
    pub fn hsttxfifo(&self) -> HsttxfifoR {
        HsttxfifoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Host RXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
    #[inline(always)]
    pub fn hstrxfifo(&self) -> HstrxfifoR {
        HstrxfifoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Host TXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn hsttxfifo(&mut self) -> HsttxfifoW<Usb3GdmahlratioSpec> {
        HsttxfifoW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Host RXFIFO DMA High-Low Priority This register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX. To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on. If FIFOs from both speed groups are not requesting access simultaneously then, 1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request. 2. if HS/FSLS got the grant last time, SS gets the priority on the next request. 3. if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle. This register is present if the core is configured to operate in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn hstrxfifo(&mut self) -> HstrxfifoW<Usb3GdmahlratioSpec> {
        HstrxfifoW::new(self, 8)
    }
}
#[doc = "Global Host FIFO DMA High-Low Priority Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdmahlratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdmahlratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdmahlratioSpec;
impl crate::RegisterSpec for Usb3GdmahlratioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdmahlratio::R`](R) reader structure"]
impl crate::Readable for Usb3GdmahlratioSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gdmahlratio::W`](W) writer structure"]
impl crate::Writable for Usb3GdmahlratioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GDMAHLRATIO to value 0x08"]
impl crate::Resettable for Usb3GdmahlratioSpec {
    const RESET_VALUE: u32 = 0x08;
}
