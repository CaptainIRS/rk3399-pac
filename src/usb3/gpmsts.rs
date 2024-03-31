#[doc = "Register `GPMSTS` reader"]
pub type R = crate::R<GpmstsSpec>;
#[doc = "Register `GPMSTS` writer"]
pub type W = crate::W<GpmstsSpec>;
#[doc = "Field `U2WAKEUP` reader - U2Wakeup\n\nThis field indicates the following USB 2.0 port wakeup conditions:\n\nBit \\[0\\]: Overcurrent Detected\n\nBit \\[1\\]: Resume Detected\n\nBit \\[2\\]: Connect Detected\n\nBit \\[3\\]: Disconnect Detected\n\nBit \\[4\\]: Last Connection State\n\nBit \\[5\\]: ID Change Detected\n\nBit \\[6\\]: SRP Request Detected\n\nBit \\[7\\]: ULPI Interrupt Detected\n\nBit \\[8\\]: USB Reset Detected\n\nBit \\[9\\]: Resume Detected Changed"]
pub type U2wakeupR = crate::FieldReader<u16>;
#[doc = "Field `U3WAKEUP` reader - U3Wakeup\n\nThis field gives the following USB 3.0 port wakeup conditions:\n\nBit \\[12\\]: Overcurrent Detected\n\nBit \\[13\\]: Resume Detected\n\nBit \\[14\\]: Connect Detected\n\nBit \\[15\\]: Disconnect Detected\n\nBit \\[16\\]: Last Connection State"]
pub type U3wakeupR = crate::FieldReader;
#[doc = "Field `PORTSEL` writer - Global Power Management Status Register\n\nThis field selects the port number."]
pub type PortselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - U2Wakeup\n\nThis field indicates the following USB 2.0 port wakeup conditions:\n\nBit \\[0\\]: Overcurrent Detected\n\nBit \\[1\\]: Resume Detected\n\nBit \\[2\\]: Connect Detected\n\nBit \\[3\\]: Disconnect Detected\n\nBit \\[4\\]: Last Connection State\n\nBit \\[5\\]: ID Change Detected\n\nBit \\[6\\]: SRP Request Detected\n\nBit \\[7\\]: ULPI Interrupt Detected\n\nBit \\[8\\]: USB Reset Detected\n\nBit \\[9\\]: Resume Detected Changed"]
    #[inline(always)]
    pub fn u2wakeup(&self) -> U2wakeupR {
        U2wakeupR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:16 - U3Wakeup\n\nThis field gives the following USB 3.0 port wakeup conditions:\n\nBit \\[12\\]: Overcurrent Detected\n\nBit \\[13\\]: Resume Detected\n\nBit \\[14\\]: Connect Detected\n\nBit \\[15\\]: Disconnect Detected\n\nBit \\[16\\]: Last Connection State"]
    #[inline(always)]
    pub fn u3wakeup(&self) -> U3wakeupR {
        U3wakeupR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Global Power Management Status Register\n\nThis field selects the port number."]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PortselW<GpmstsSpec> {
        PortselW::new(self, 28)
    }
}
#[doc = "Global Power Management Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpmsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpmsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpmstsSpec;
impl crate::RegisterSpec for GpmstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpmsts::R`](R) reader structure"]
impl crate::Readable for GpmstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gpmsts::W`](W) writer structure"]
impl crate::Writable for GpmstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPMSTS to value 0"]
impl crate::Resettable for GpmstsSpec {
    const RESET_VALUE: u32 = 0;
}
