#[doc = "Register `UART_DLL` reader"]
pub type R = crate::R<UartDllSpec>;
#[doc = "Register `UART_DLL` writer"]
pub type W = crate::W<UartDllSpec>;
#[doc = "Field `BAUD_RATE_DIVISOR_L` reader - Lower 8-bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART. This register may\n\nonly be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART\n\nis not busy (USR\\[0\\]
is zero). The output baud rate is equal to the\n\nserial clock (sclk) frequency divided by sixteen times the value of\n\nthe baud rate divisor, as follows: baud rate = (serial clock freq) /\n\n(16 * divisor).\n\nNote that with the Divisor Latch Registers (DLL and DLH) set to\n\nzero, the baud clock is disabled and no serial communications\n\noccur. Also, once the DLH is set, at least 8 clock cycles of the\n\nslowest UART clock should be allowed to pass before transmitting\n\nor receiving data."]
pub type BaudRateDivisorLR = crate::FieldReader;
#[doc = "Field `BAUD_RATE_DIVISOR_L` writer - Lower 8-bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART. This register may\n\nonly be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART\n\nis not busy (USR\\[0\\]
is zero). The output baud rate is equal to the\n\nserial clock (sclk) frequency divided by sixteen times the value of\n\nthe baud rate divisor, as follows: baud rate = (serial clock freq) /\n\n(16 * divisor).\n\nNote that with the Divisor Latch Registers (DLL and DLH) set to\n\nzero, the baud clock is disabled and no serial communications\n\noccur. Also, once the DLH is set, at least 8 clock cycles of the\n\nslowest UART clock should be allowed to pass before transmitting\n\nor receiving data."]
pub type BaudRateDivisorLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Lower 8-bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART. This register may\n\nonly be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART\n\nis not busy (USR\\[0\\]
is zero). The output baud rate is equal to the\n\nserial clock (sclk) frequency divided by sixteen times the value of\n\nthe baud rate divisor, as follows: baud rate = (serial clock freq) /\n\n(16 * divisor).\n\nNote that with the Divisor Latch Registers (DLL and DLH) set to\n\nzero, the baud clock is disabled and no serial communications\n\noccur. Also, once the DLH is set, at least 8 clock cycles of the\n\nslowest UART clock should be allowed to pass before transmitting\n\nor receiving data."]
    #[inline(always)]
    pub fn baud_rate_divisor_l(&self) -> BaudRateDivisorLR {
        BaudRateDivisorLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower 8-bits of a 16-bit, read/write, Divisor Latch register that\n\ncontains the baud rate divisor for the UART. This register may\n\nonly be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART\n\nis not busy (USR\\[0\\]
is zero). The output baud rate is equal to the\n\nserial clock (sclk) frequency divided by sixteen times the value of\n\nthe baud rate divisor, as follows: baud rate = (serial clock freq) /\n\n(16 * divisor).\n\nNote that with the Divisor Latch Registers (DLL and DLH) set to\n\nzero, the baud clock is disabled and no serial communications\n\noccur. Also, once the DLH is set, at least 8 clock cycles of the\n\nslowest UART clock should be allowed to pass before transmitting\n\nor receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn baud_rate_divisor_l(&mut self) -> BaudRateDivisorLW<UartDllSpec> {
        BaudRateDivisorLW::new(self, 0)
    }
}
#[doc = "Divisor Latch (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDllSpec;
impl crate::RegisterSpec for UartDllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_dll::R`](R) reader structure"]
impl crate::Readable for UartDllSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_dll::W`](W) writer structure"]
impl crate::Writable for UartDllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_DLL to value 0"]
impl crate::Resettable for UartDllSpec {
    const RESET_VALUE: u32 = 0;
}
