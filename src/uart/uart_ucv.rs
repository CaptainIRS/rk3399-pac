#[doc = "Register `UART_UCV` reader"]
pub type R = crate::R<UartUcvSpec>;
#[doc = "Field `VER` reader - ASCII value for each number in the version"]
pub type VerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ASCII value for each number in the version"]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(self.bits)
    }
}
#[doc = "UART Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ucv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUcvSpec;
impl crate::RegisterSpec for UartUcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ucv::R`](R) reader structure"]
impl crate::Readable for UartUcvSpec {}
#[doc = "`reset()` method sets UART_UCV to value 0x0330_372a"]
impl crate::Resettable for UartUcvSpec {
    const RESET_VALUE: u32 = 0x0330_372a;
}
