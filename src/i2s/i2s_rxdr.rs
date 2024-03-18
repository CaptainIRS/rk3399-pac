#[doc = "Register `I2S_RXDR` reader"]
pub type R = crate::R<I2sRxdrSpec>;
#[doc = "Field `RXDR` reader - Receive FIFO Data Register When the register is read, data in the receive FIFO is accessed."]
pub type RxdrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO Data Register When the register is read, data in the receive FIFO is accessed."]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(self.bits)
    }
}
#[doc = "Receive FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sRxdrSpec;
impl crate::RegisterSpec for I2sRxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_rxdr::R`](R) reader structure"]
impl crate::Readable for I2sRxdrSpec {}
#[doc = "`reset()` method sets I2S_RXDR to value 0"]
impl crate::Resettable for I2sRxdrSpec {
    const RESET_VALUE: u32 = 0;
}
