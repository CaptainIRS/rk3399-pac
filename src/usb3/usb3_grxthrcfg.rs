#[doc = "Register `USB3_GRXTHRCFG` reader"]
pub type R = crate::R<Usb3GrxthrcfgSpec>;
#[doc = "Register `USB3_GRXTHRCFG` writer"]
pub type W = crate::W<Usb3GrxthrcfgSpec>;
#[doc = "Field `USBMAXRXBURSTSIZE` reader - USB Maximum Receive Burst Size\n\nIn host mode, this field specifies the Maximum Bulk IN burst the\n\nusb3 controller can perform.\n\nWhen the system bus is slower than the USB, RX FIFO can\n\noverrun during a long burst.\n\nYou can program a smaller value to this field to limit the RX burst\n\nsize that the core can perform. It only applies to SS Bulk,\n\nIsochronous, and Interrupt IN endpoints in the host mode.\n\nIn device mode, this field specifies the NUMP value that is sent in\n\nERDY for an OUT endpoint.\n\nThis field is valid only when USBRxPktCntSel is one. The valid\n\nvalues for this field are from 1 to 16."]
pub type UsbmaxrxburstsizeR = crate::FieldReader;
#[doc = "Field `USBMAXRXBURSTSIZE` writer - USB Maximum Receive Burst Size\n\nIn host mode, this field specifies the Maximum Bulk IN burst the\n\nusb3 controller can perform.\n\nWhen the system bus is slower than the USB, RX FIFO can\n\noverrun during a long burst.\n\nYou can program a smaller value to this field to limit the RX burst\n\nsize that the core can perform. It only applies to SS Bulk,\n\nIsochronous, and Interrupt IN endpoints in the host mode.\n\nIn device mode, this field specifies the NUMP value that is sent in\n\nERDY for an OUT endpoint.\n\nThis field is valid only when USBRxPktCntSel is one. The valid\n\nvalues for this field are from 1 to 16."]
pub type UsbmaxrxburstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USBRXPKTCNT` reader - USB Receive Packet Count\n\nIn host mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can start the corresponding USB RX transaction (burst).\n\nIn device mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can send ERDY for a flow-controlled endpoint.\n\nThis field is valid only when the USB Receive Packet Count Enable\n\nfield is set to 1. The valid values for this field are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nReceive Burst Size field."]
pub type UsbrxpktcntR = crate::FieldReader;
#[doc = "Field `USBRXPKTCNT` writer - USB Receive Packet Count\n\nIn host mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can start the corresponding USB RX transaction (burst).\n\nIn device mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can send ERDY for a flow-controlled endpoint.\n\nThis field is valid only when the USB Receive Packet Count Enable\n\nfield is set to 1. The valid values for this field are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nReceive Burst Size field."]
pub type UsbrxpktcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "USB ReceivePacket Count Enable\n\nThis field enables/disables the USB reception multi-packet\n\nthresholding:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrxpktcntsel {
    #[doc = "0: The core can only start reception on the USB when the RX FIFO has space for at least one packet."]
    B0 = 0,
    #[doc = "1: The core can only start reception on the USB when the RX FIFO has space for at least USBRxPktCnt amount of packets. This mode is valid in both host and device mode. It is only used for SuperSpeed. In device mode, Setting this bit to 1 also enables the functionality of reporting NUMP in the ACK TP based on the RX FIFO space instead of reporting a fixed NUMP derived from DCFG.NUMP If you are using external buffer control (EBC) feature, disable this mode by setting USBRxPktCntSel to 0."]
    B1 = 1,
}
impl From<Usbrxpktcntsel> for bool {
    #[inline(always)]
    fn from(variant: Usbrxpktcntsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRXPKTCNTSEL` reader - USB ReceivePacket Count Enable\n\nThis field enables/disables the USB reception multi-packet\n\nthresholding:"]
pub type UsbrxpktcntselR = crate::BitReader<Usbrxpktcntsel>;
impl UsbrxpktcntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrxpktcntsel {
        match self.bits {
            false => Usbrxpktcntsel::B0,
            true => Usbrxpktcntsel::B1,
        }
    }
    #[doc = "The core can only start reception on the USB when the RX FIFO has space for at least one packet."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbrxpktcntsel::B0
    }
    #[doc = "The core can only start reception on the USB when the RX FIFO has space for at least USBRxPktCnt amount of packets. This mode is valid in both host and device mode. It is only used for SuperSpeed. In device mode, Setting this bit to 1 also enables the functionality of reporting NUMP in the ACK TP based on the RX FIFO space instead of reporting a fixed NUMP derived from DCFG.NUMP If you are using external buffer control (EBC) feature, disable this mode by setting USBRxPktCntSel to 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbrxpktcntsel::B1
    }
}
#[doc = "Field `USBRXPKTCNTSEL` writer - USB ReceivePacket Count Enable\n\nThis field enables/disables the USB reception multi-packet\n\nthresholding:"]
pub type UsbrxpktcntselW<'a, REG> = crate::BitWriter<'a, REG, Usbrxpktcntsel>;
impl<'a, REG> UsbrxpktcntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core can only start reception on the USB when the RX FIFO has space for at least one packet."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrxpktcntsel::B0)
    }
    #[doc = "The core can only start reception on the USB when the RX FIFO has space for at least USBRxPktCnt amount of packets. This mode is valid in both host and device mode. It is only used for SuperSpeed. In device mode, Setting this bit to 1 also enables the functionality of reporting NUMP in the ACK TP based on the RX FIFO space instead of reporting a fixed NUMP derived from DCFG.NUMP If you are using external buffer control (EBC) feature, disable this mode by setting USBRxPktCntSel to 0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrxpktcntsel::B1)
    }
}
impl R {
    #[doc = "Bits 19:23 - USB Maximum Receive Burst Size\n\nIn host mode, this field specifies the Maximum Bulk IN burst the\n\nusb3 controller can perform.\n\nWhen the system bus is slower than the USB, RX FIFO can\n\noverrun during a long burst.\n\nYou can program a smaller value to this field to limit the RX burst\n\nsize that the core can perform. It only applies to SS Bulk,\n\nIsochronous, and Interrupt IN endpoints in the host mode.\n\nIn device mode, this field specifies the NUMP value that is sent in\n\nERDY for an OUT endpoint.\n\nThis field is valid only when USBRxPktCntSel is one. The valid\n\nvalues for this field are from 1 to 16."]
    #[inline(always)]
    pub fn usbmaxrxburstsize(&self) -> UsbmaxrxburstsizeR {
        UsbmaxrxburstsizeR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - USB Receive Packet Count\n\nIn host mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can start the corresponding USB RX transaction (burst).\n\nIn device mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can send ERDY for a flow-controlled endpoint.\n\nThis field is valid only when the USB Receive Packet Count Enable\n\nfield is set to 1. The valid values for this field are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nReceive Burst Size field."]
    #[inline(always)]
    pub fn usbrxpktcnt(&self) -> UsbrxpktcntR {
        UsbrxpktcntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - USB ReceivePacket Count Enable\n\nThis field enables/disables the USB reception multi-packet\n\nthresholding:"]
    #[inline(always)]
    pub fn usbrxpktcntsel(&self) -> UsbrxpktcntselR {
        UsbrxpktcntselR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 19:23 - USB Maximum Receive Burst Size\n\nIn host mode, this field specifies the Maximum Bulk IN burst the\n\nusb3 controller can perform.\n\nWhen the system bus is slower than the USB, RX FIFO can\n\noverrun during a long burst.\n\nYou can program a smaller value to this field to limit the RX burst\n\nsize that the core can perform. It only applies to SS Bulk,\n\nIsochronous, and Interrupt IN endpoints in the host mode.\n\nIn device mode, this field specifies the NUMP value that is sent in\n\nERDY for an OUT endpoint.\n\nThis field is valid only when USBRxPktCntSel is one. The valid\n\nvalues for this field are from 1 to 16."]
    #[inline(always)]
    #[must_use]
    pub fn usbmaxrxburstsize(&mut self) -> UsbmaxrxburstsizeW<Usb3GrxthrcfgSpec> {
        UsbmaxrxburstsizeW::new(self, 19)
    }
    #[doc = "Bits 24:27 - USB Receive Packet Count\n\nIn host mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can start the corresponding USB RX transaction (burst).\n\nIn device mode, this field specifies the space (in terms of the\n\nnumber of packets) that must be available in the RX FIFO before\n\nthe core can send ERDY for a flow-controlled endpoint.\n\nThis field is valid only when the USB Receive Packet Count Enable\n\nfield is set to 1. The valid values for this field are from 1 to 15.\n\nNote: This field must be less than or equal to the USB Maximum\n\nReceive Burst Size field."]
    #[inline(always)]
    #[must_use]
    pub fn usbrxpktcnt(&mut self) -> UsbrxpktcntW<Usb3GrxthrcfgSpec> {
        UsbrxpktcntW::new(self, 24)
    }
    #[doc = "Bit 29 - USB ReceivePacket Count Enable\n\nThis field enables/disables the USB reception multi-packet\n\nthresholding:"]
    #[inline(always)]
    #[must_use]
    pub fn usbrxpktcntsel(&mut self) -> UsbrxpktcntselW<Usb3GrxthrcfgSpec> {
        UsbrxpktcntselW::new(self, 29)
    }
}
#[doc = "Global Rx Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_grxthrcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_grxthrcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GrxthrcfgSpec;
impl crate::RegisterSpec for Usb3GrxthrcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_grxthrcfg::R`](R) reader structure"]
impl crate::Readable for Usb3GrxthrcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_grxthrcfg::W`](W) writer structure"]
impl crate::Writable for Usb3GrxthrcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GRXTHRCFG to value 0"]
impl crate::Resettable for Usb3GrxthrcfgSpec {
    const RESET_VALUE: u32 = 0;
}
