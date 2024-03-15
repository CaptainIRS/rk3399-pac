#[doc = "Register `USB3_GPMSTS` reader"]
pub type R = crate::R<Usb3GpmstsSpec>;
#[doc = "Register `USB3_GPMSTS` writer"]
pub type W = crate::W<Usb3GpmstsSpec>;
#[doc = "Field `U2WAKEUP` reader - U2Wakeup This field indicates the following USB 2.0 port wakeup conditions: Bit \\[0\\]: Overcurrent Detected Bit \\[1\\]: Resume Detected Bit \\[2\\]: Connect Detected Bit \\[3\\]: Disconnect Detected Bit \\[4\\]: Last Connection State Bit \\[5\\]: ID Change Detected Bit \\[6\\]: SRP Request Detected Bit \\[7\\]: ULPI Interrupt Detected Bit \\[8\\]: USB Reset Detected Bit \\[9\\]: Resume Detected Changed"]
pub type U2wakeupR = crate::FieldReader<u16>;
#[doc = "Field `U3WAKEUP` reader - U3Wakeup This field gives the following USB 3.0 port wakeup conditions: Bit \\[12\\]: Overcurrent Detected Bit \\[13\\]: Resume Detected Bit \\[14\\]: Connect Detected Bit \\[15\\]: Disconnect Detected Bit \\[16\\]: Last Connection State"]
pub type U3wakeupR = crate::FieldReader;
#[doc = "Field `PORTSEL` writer - Global Power Management Status Register This field selects the port number."]
pub type PortselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - U2Wakeup This field indicates the following USB 2.0 port wakeup conditions: Bit \\[0\\]: Overcurrent Detected Bit \\[1\\]: Resume Detected Bit \\[2\\]: Connect Detected Bit \\[3\\]: Disconnect Detected Bit \\[4\\]: Last Connection State Bit \\[5\\]: ID Change Detected Bit \\[6\\]: SRP Request Detected Bit \\[7\\]: ULPI Interrupt Detected Bit \\[8\\]: USB Reset Detected Bit \\[9\\]: Resume Detected Changed"]
    #[inline(always)]
    pub fn u2wakeup(&self) -> U2wakeupR {
        U2wakeupR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:16 - U3Wakeup This field gives the following USB 3.0 port wakeup conditions: Bit \\[12\\]: Overcurrent Detected Bit \\[13\\]: Resume Detected Bit \\[14\\]: Connect Detected Bit \\[15\\]: Disconnect Detected Bit \\[16\\]: Last Connection State"]
    #[inline(always)]
    pub fn u3wakeup(&self) -> U3wakeupR {
        U3wakeupR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Global Power Management Status Register This field selects the port number."]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PortselW<Usb3GpmstsSpec> {
        PortselW::new(self, 28)
    }
}
#[doc = "Global Power Management Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gpmsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gpmsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GpmstsSpec;
impl crate::RegisterSpec for Usb3GpmstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gpmsts::R`](R) reader structure"]
impl crate::Readable for Usb3GpmstsSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gpmsts::W`](W) writer structure"]
impl crate::Writable for Usb3GpmstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GPMSTS to value 0"]
impl crate::Resettable for Usb3GpmstsSpec {
    const RESET_VALUE: u32 = 0;
}
