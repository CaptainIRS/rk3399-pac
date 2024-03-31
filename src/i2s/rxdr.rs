#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RxdrSpec>;
#[doc = "Field `RXDR` reader - Receive FIFO Data Register\n\nWhen the register is read, data in the receive FIFO is accessed."]
pub type RxdrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO Data Register\n\nWhen the register is read, data in the receive FIFO is accessed."]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(self.bits)
    }
}
#[doc = "Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdrSpec;
impl crate::RegisterSpec for RxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RxdrSpec {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RxdrSpec {
    const RESET_VALUE: u32 = 0;
}
