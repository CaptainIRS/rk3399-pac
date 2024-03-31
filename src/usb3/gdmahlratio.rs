#[doc = "Register `GDMAHLRATIO` reader"]
pub type R = crate::R<GdmahlratioSpec>;
#[doc = "Register `GDMAHLRATIO` writer"]
pub type W = crate::W<GdmahlratioSpec>;
#[doc = "Field `HSTTXFIFO` reader - Host TXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
pub type HsttxfifoR = crate::FieldReader;
#[doc = "Field `HSTTXFIFO` writer - Host TXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
pub type HsttxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HSTRXFIFO` reader - Host RXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
pub type HstrxfifoR = crate::FieldReader;
#[doc = "Field `HSTRXFIFO` writer - Host RXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
pub type HstrxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Host TXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
    #[inline(always)]
    pub fn hsttxfifo(&self) -> HsttxfifoR {
        HsttxfifoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Host RXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
    #[inline(always)]
    pub fn hstrxfifo(&self) -> HstrxfifoR {
        HstrxfifoR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Host TXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn hsttxfifo(&mut self) -> HsttxfifoW<GdmahlratioSpec> {
        HsttxfifoW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Host RXFIFO DMA High-Low Priority\n\nThis register specifies the relative priority of the SS FIFOs with\n\nrespect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the\n\nHS/FSLS round-robin arbiter group every DMA High-Low Priority\n\nRatio grants as indicated in the register separately for TX and RX.\n\nTo illustrate, consider that all FIFOs are requesting access\n\nsimultaneously, and the ratio is 4. SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, SS gets priority for 4 packets,\n\nHS/FSLS gets priority for 1 packet, and so on.\n\nIf FIFOs from both speed groups are not requesting access\n\nsimultaneously then,\n\n1. if SS got grants 4 out of the last 4 times, then HS/FSLS get the\n\npriority on any future request.\n\n2. if HS/FSLS got the grant last time, SS gets the priority on the\n\nnext request.\n\n3. if there is a valid request on either SS or HS/FSLS, a grant is\n\nalways awarded; there is no idle.\n\nThis register is present if the core is configured to operate in host\n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn hstrxfifo(&mut self) -> HstrxfifoW<GdmahlratioSpec> {
        HstrxfifoW::new(self, 8)
    }
}
#[doc = "Global Host FIFO DMA High-Low Priority Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdmahlratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdmahlratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmahlratioSpec;
impl crate::RegisterSpec for GdmahlratioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdmahlratio::R`](R) reader structure"]
impl crate::Readable for GdmahlratioSpec {}
#[doc = "`write(|w| ..)` method takes [`gdmahlratio::W`](W) writer structure"]
impl crate::Writable for GdmahlratioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMAHLRATIO to value 0x08"]
impl crate::Resettable for GdmahlratioSpec {
    const RESET_VALUE: u32 = 0x08;
}
