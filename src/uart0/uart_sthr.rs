#[doc = "Register `UART_STHR` reader"]
pub type R = crate::R<UartSthrSpec>;
#[doc = "Field `SHADOW_THR` reader - This is a shadow register for the THR."]
pub type ShadowThrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This is a shadow register for the THR."]
    #[inline(always)]
    pub fn shadow_thr(&self) -> ShadowThrR {
        ShadowThrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Shadow Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sthr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSthrSpec;
impl crate::RegisterSpec for UartSthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sthr::R`](R) reader structure"]
impl crate::Readable for UartSthrSpec {}
#[doc = "`reset()` method sets UART_STHR to value 0"]
impl crate::Resettable for UartSthrSpec {
    const RESET_VALUE: u32 = 0;
}
