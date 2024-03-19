#[doc = "Register `USB3_GTXFIFOSIZ%s` reader"]
pub type R = crate::R<Usb3GtxfifosizSpec>;
#[doc = "Register `USB3_GTXFIFOSIZ%s` writer"]
pub type W = crate::W<Usb3GtxfifosizSpec>;
#[doc = "Field `TXFDEP_N` reader - TxFIFO Depth\n\nThis field contains the depth of TxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 32,768"]
pub type TxfdepNR = crate::FieldReader<u16>;
#[doc = "Field `TXFDEP_N` writer - TxFIFO Depth\n\nThis field contains the depth of TxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 32,768"]
pub type TxfdepNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXFSTADDR_N` reader - Transmit FIFOn RAM Start Address\n\nThis field contains the memory start address for TxFIFOn in 64-\n\nbit words."]
pub type TxfstaddrNR = crate::FieldReader<u16>;
#[doc = "Field `TXFSTADDR_N` writer - Transmit FIFOn RAM Start Address\n\nThis field contains the memory start address for TxFIFOn in 64-\n\nbit words."]
pub type TxfstaddrNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxFIFO Depth\n\nThis field contains the depth of TxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 32,768"]
    #[inline(always)]
    pub fn txfdep_n(&self) -> TxfdepNR {
        TxfdepNR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit FIFOn RAM Start Address\n\nThis field contains the memory start address for TxFIFOn in 64-\n\nbit words."]
    #[inline(always)]
    pub fn txfstaddr_n(&self) -> TxfstaddrNR {
        TxfstaddrNR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxFIFO Depth\n\nThis field contains the depth of TxFIFOn in 64-bit words.\n\nMinimum value: 32; Maximum value: 32,768"]
    #[inline(always)]
    #[must_use]
    pub fn txfdep_n(&mut self) -> TxfdepNW<Usb3GtxfifosizSpec> {
        TxfdepNW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit FIFOn RAM Start Address\n\nThis field contains the memory start address for TxFIFOn in 64-\n\nbit words."]
    #[inline(always)]
    #[must_use]
    pub fn txfstaddr_n(&mut self) -> TxfstaddrNW<Usb3GtxfifosizSpec> {
        TxfstaddrNW::new(self, 16)
    }
}
#[doc = "Global Transmit FIFO Size Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gtxfifosiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gtxfifosiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GtxfifosizSpec;
impl crate::RegisterSpec for Usb3GtxfifosizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gtxfifosiz::R`](R) reader structure"]
impl crate::Readable for Usb3GtxfifosizSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gtxfifosiz::W`](W) writer structure"]
impl crate::Writable for Usb3GtxfifosizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GTXFIFOSIZ%s to value 0x42"]
impl crate::Resettable for Usb3GtxfifosizSpec {
    const RESET_VALUE: u32 = 0x42;
}
