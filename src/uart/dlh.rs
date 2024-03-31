#[doc = "Register `DLH` reader"]
pub type R = crate::R<DlhSpec>;
#[doc = "Register `DLH` writer"]
pub type W = crate::W<DlhSpec>;
#[doc = "Field `BAUD_RATE_DIVISOR_H` reader - Upper 8 bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART."]
pub type BaudRateDivisorHR = crate::FieldReader;
#[doc = "Field `BAUD_RATE_DIVISOR_H` writer - Upper 8 bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART."]
pub type BaudRateDivisorHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Upper 8 bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART."]
    #[inline(always)]
    pub fn baud_rate_divisor_h(&self) -> BaudRateDivisorHR {
        BaudRateDivisorHR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper 8 bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART."]
    #[inline(always)]
    #[must_use]
    pub fn baud_rate_divisor_h(&mut self) -> BaudRateDivisorHW<DlhSpec> {
        BaudRateDivisorHW::new(self, 0)
    }
}
#[doc = "Divisor Latch (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlhSpec;
impl crate::RegisterSpec for DlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlh::R`](R) reader structure"]
impl crate::Readable for DlhSpec {}
#[doc = "`write(|w| ..)` method takes [`dlh::W`](W) writer structure"]
impl crate::Writable for DlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLH to value 0"]
impl crate::Resettable for DlhSpec {
    const RESET_VALUE: u32 = 0;
}
