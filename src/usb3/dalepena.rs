#[doc = "Register `DALEPENA` reader"]
pub type R = crate::R<DalepenaSpec>;
#[doc = "Register `DALEPENA` writer"]
pub type W = crate::W<DalepenaSpec>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoints\n\nThis field indicates if a USB endpoint is active in the current\n\nconfiguration and interface. It applies to USB IN endpoints 0~15\n\nand OUT endpoints 0~15, with one bit for each of the 32 possible\n\nendpoints. Even numbers are for USB OUT endpoints, and odd\n\nnumbers are for USB IN endpoints, as follows:\n\nBit\\[0\\]: USB EP0-OUT\n\nBit\\[1\\]: USB EP0-IN\n\nBit\\[2\\]: USB EP1-OUT\n\nBit\\[3\\]: USB EP1-IN\n\n...\n\nThe entity programming this register must set bits 0 and 1\n\nbecause they enable control endpoints that map to physical\n\nendpoints (resources) after USBReset.\n\nHardware clears these bits for all endpoints (other than EP0-OUT\n\nand EP0-IN) after detecting a USB reset event. After receiving\n\nSetConfiguration and SetInterface requests, the application must\n\nprogram endpoint registers accordingly and set these bits.\n\nFor more information, Pls see 'Flexible Endpoint Mapping'\n\nsection."]
pub type UsbactepR = crate::FieldReader<u32>;
#[doc = "Field `USBACTEP` writer - USB Active Endpoints\n\nThis field indicates if a USB endpoint is active in the current\n\nconfiguration and interface. It applies to USB IN endpoints 0~15\n\nand OUT endpoints 0~15, with one bit for each of the 32 possible\n\nendpoints. Even numbers are for USB OUT endpoints, and odd\n\nnumbers are for USB IN endpoints, as follows:\n\nBit\\[0\\]: USB EP0-OUT\n\nBit\\[1\\]: USB EP0-IN\n\nBit\\[2\\]: USB EP1-OUT\n\nBit\\[3\\]: USB EP1-IN\n\n...\n\nThe entity programming this register must set bits 0 and 1\n\nbecause they enable control endpoints that map to physical\n\nendpoints (resources) after USBReset.\n\nHardware clears these bits for all endpoints (other than EP0-OUT\n\nand EP0-IN) after detecting a USB reset event. After receiving\n\nSetConfiguration and SetInterface requests, the application must\n\nprogram endpoint registers accordingly and set these bits.\n\nFor more information, Pls see 'Flexible Endpoint Mapping'\n\nsection."]
pub type UsbactepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB Active Endpoints\n\nThis field indicates if a USB endpoint is active in the current\n\nconfiguration and interface. It applies to USB IN endpoints 0~15\n\nand OUT endpoints 0~15, with one bit for each of the 32 possible\n\nendpoints. Even numbers are for USB OUT endpoints, and odd\n\nnumbers are for USB IN endpoints, as follows:\n\nBit\\[0\\]: USB EP0-OUT\n\nBit\\[1\\]: USB EP0-IN\n\nBit\\[2\\]: USB EP1-OUT\n\nBit\\[3\\]: USB EP1-IN\n\n...\n\nThe entity programming this register must set bits 0 and 1\n\nbecause they enable control endpoints that map to physical\n\nendpoints (resources) after USBReset.\n\nHardware clears these bits for all endpoints (other than EP0-OUT\n\nand EP0-IN) after detecting a USB reset event. After receiving\n\nSetConfiguration and SetInterface requests, the application must\n\nprogram endpoint registers accordingly and set these bits.\n\nFor more information, Pls see 'Flexible Endpoint Mapping'\n\nsection."]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB Active Endpoints\n\nThis field indicates if a USB endpoint is active in the current\n\nconfiguration and interface. It applies to USB IN endpoints 0~15\n\nand OUT endpoints 0~15, with one bit for each of the 32 possible\n\nendpoints. Even numbers are for USB OUT endpoints, and odd\n\nnumbers are for USB IN endpoints, as follows:\n\nBit\\[0\\]: USB EP0-OUT\n\nBit\\[1\\]: USB EP0-IN\n\nBit\\[2\\]: USB EP1-OUT\n\nBit\\[3\\]: USB EP1-IN\n\n...\n\nThe entity programming this register must set bits 0 and 1\n\nbecause they enable control endpoints that map to physical\n\nendpoints (resources) after USBReset.\n\nHardware clears these bits for all endpoints (other than EP0-OUT\n\nand EP0-IN) after detecting a USB reset event. After receiving\n\nSetConfiguration and SetInterface requests, the application must\n\nprogram endpoint registers accordingly and set these bits.\n\nFor more information, Pls see 'Flexible Endpoint Mapping'\n\nsection."]
    #[inline(always)]
    #[must_use]
    pub fn usbactep(&mut self) -> UsbactepW<DalepenaSpec> {
        UsbactepW::new(self, 0)
    }
}
#[doc = "Device Active USB Endpoint Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dalepena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dalepena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DalepenaSpec;
impl crate::RegisterSpec for DalepenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dalepena::R`](R) reader structure"]
impl crate::Readable for DalepenaSpec {}
#[doc = "`write(|w| ..)` method takes [`dalepena::W`](W) writer structure"]
impl crate::Writable for DalepenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DALEPENA to value 0"]
impl crate::Resettable for DalepenaSpec {
    const RESET_VALUE: u32 = 0;
}
