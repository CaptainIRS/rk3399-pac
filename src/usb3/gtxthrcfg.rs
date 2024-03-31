#[doc = "Register `GTXTHRCFG` reader"]
pub type R = crate::R<GtxthrcfgSpec>;
#[doc = "Register `GTXTHRCFG` writer"]
pub type W = crate::W<GtxthrcfgSpec>;
#[doc = "Field `USBMAXTXBURSTSIZE` reader - USB Maximum TX Burst Size\n\nWhen USBTxPktCntSel is 1, this field specifies the Maximum Bulk\n\nOUT burst the core can execute. When the system bus is slower\n\nthan the USB, TX FIFO can underrun during a long burst.\n\nYou can program a smaller value to this field to limit the TX burst\n\nsize that the core can execute.\n\nIt only applies to SS Bulk, Isochronous, and Interrupt OUT\n\nendpoints in the host mode. Valid values are from 1 to 16."]
pub type UsbmaxtxburstsizeR = crate::FieldReader;
#[doc = "Field `USBMAXTXBURSTSIZE` writer - USB Maximum TX Burst Size\n\nWhen USBTxPktCntSel is 1, this field specifies the Maximum Bulk\n\nOUT burst the core can execute. When the system bus is slower\n\nthan the USB, TX FIFO can underrun during a long burst.\n\nYou can program a smaller value to this field to limit the TX burst\n\nsize that the core can execute.\n\nIt only applies to SS Bulk, Isochronous, and Interrupt OUT\n\nendpoints in the host mode. Valid values are from 1 to 16."]
pub type UsbmaxtxburstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USBTXPKTCNT` reader - USB Transmit Packet Count\n\nThis field specifies the number of packets that must be in the\n\nTXFIFO before the core can start transmission for the\n\ncorresponding USB transaction (burst). This field is only valid\n\nwhen the USB Transmit Packet Count Enable field is set to one.\n\nValid values are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nTX Burst Size field."]
pub type UsbtxpktcntR = crate::FieldReader;
#[doc = "Field `USBTXPKTCNT` writer - USB Transmit Packet Count\n\nThis field specifies the number of packets that must be in the\n\nTXFIFO before the core can start transmission for the\n\ncorresponding USB transaction (burst). This field is only valid\n\nwhen the USB Transmit Packet Count Enable field is set to one.\n\nValid values are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nTX Burst Size field."]
pub type UsbtxpktcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "USB Transmit Packet Count Enable\n\nThis field enables/disables the USB transmission multi-packet\n\nthresholding:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbtxpktcntsel {
    #[doc = "0: USB transmission multi-packet thresholding is disabled; the core can only start transmission on the USB after the entire packet has been fetched into the corresponding TXFIFO."]
    B0 = 0,
    #[doc = "1: USB transmission multi-packet thresholding is enabled. The core can only start transmission on the USB after USB Transmit Packet Count amount of packets for the USB transaction (burst) are already in the corresponding TXFIFO. This mode is only valid in the host mode. It is only used for SuperSpeed."]
    B1 = 1,
}
impl From<Usbtxpktcntsel> for bool {
    #[inline(always)]
    fn from(variant: Usbtxpktcntsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBTXPKTCNTSEL` reader - USB Transmit Packet Count Enable\n\nThis field enables/disables the USB transmission multi-packet\n\nthresholding:"]
pub type UsbtxpktcntselR = crate::BitReader<Usbtxpktcntsel>;
impl UsbtxpktcntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbtxpktcntsel {
        match self.bits {
            false => Usbtxpktcntsel::B0,
            true => Usbtxpktcntsel::B1,
        }
    }
    #[doc = "USB transmission multi-packet thresholding is disabled; the core can only start transmission on the USB after the entire packet has been fetched into the corresponding TXFIFO."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbtxpktcntsel::B0
    }
    #[doc = "USB transmission multi-packet thresholding is enabled. The core can only start transmission on the USB after USB Transmit Packet Count amount of packets for the USB transaction (burst) are already in the corresponding TXFIFO. This mode is only valid in the host mode. It is only used for SuperSpeed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbtxpktcntsel::B1
    }
}
impl R {
    #[doc = "Bits 16:23 - USB Maximum TX Burst Size\n\nWhen USBTxPktCntSel is 1, this field specifies the Maximum Bulk\n\nOUT burst the core can execute. When the system bus is slower\n\nthan the USB, TX FIFO can underrun during a long burst.\n\nYou can program a smaller value to this field to limit the TX burst\n\nsize that the core can execute.\n\nIt only applies to SS Bulk, Isochronous, and Interrupt OUT\n\nendpoints in the host mode. Valid values are from 1 to 16."]
    #[inline(always)]
    pub fn usbmaxtxburstsize(&self) -> UsbmaxtxburstsizeR {
        UsbmaxtxburstsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - USB Transmit Packet Count\n\nThis field specifies the number of packets that must be in the\n\nTXFIFO before the core can start transmission for the\n\ncorresponding USB transaction (burst). This field is only valid\n\nwhen the USB Transmit Packet Count Enable field is set to one.\n\nValid values are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nTX Burst Size field."]
    #[inline(always)]
    pub fn usbtxpktcnt(&self) -> UsbtxpktcntR {
        UsbtxpktcntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - USB Transmit Packet Count Enable\n\nThis field enables/disables the USB transmission multi-packet\n\nthresholding:"]
    #[inline(always)]
    pub fn usbtxpktcntsel(&self) -> UsbtxpktcntselR {
        UsbtxpktcntselR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - USB Maximum TX Burst Size\n\nWhen USBTxPktCntSel is 1, this field specifies the Maximum Bulk\n\nOUT burst the core can execute. When the system bus is slower\n\nthan the USB, TX FIFO can underrun during a long burst.\n\nYou can program a smaller value to this field to limit the TX burst\n\nsize that the core can execute.\n\nIt only applies to SS Bulk, Isochronous, and Interrupt OUT\n\nendpoints in the host mode. Valid values are from 1 to 16."]
    #[inline(always)]
    #[must_use]
    pub fn usbmaxtxburstsize(&mut self) -> UsbmaxtxburstsizeW<GtxthrcfgSpec> {
        UsbmaxtxburstsizeW::new(self, 16)
    }
    #[doc = "Bits 24:27 - USB Transmit Packet Count\n\nThis field specifies the number of packets that must be in the\n\nTXFIFO before the core can start transmission for the\n\ncorresponding USB transaction (burst). This field is only valid\n\nwhen the USB Transmit Packet Count Enable field is set to one.\n\nValid values are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nTX Burst Size field."]
    #[inline(always)]
    #[must_use]
    pub fn usbtxpktcnt(&mut self) -> UsbtxpktcntW<GtxthrcfgSpec> {
        UsbtxpktcntW::new(self, 24)
    }
}
#[doc = "Global Tx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtxthrcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtxthrcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtxthrcfgSpec;
impl crate::RegisterSpec for GtxthrcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtxthrcfg::R`](R) reader structure"]
impl crate::Readable for GtxthrcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gtxthrcfg::W`](W) writer structure"]
impl crate::Writable for GtxthrcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTXTHRCFG to value 0"]
impl crate::Resettable for GtxthrcfgSpec {
    const RESET_VALUE: u32 = 0;
}
