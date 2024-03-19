#[doc = "Register `RKI2C_TXDATA7` reader"]
pub type R = crate::R<Rki2cTxdata7Spec>;
#[doc = "Register `RKI2C_TXDATA7` writer"]
pub type W = crate::W<Rki2cTxdata7Spec>;
#[doc = "Field `TXDATA7` reader - data7 to be transmitted\n\n32 bits data"]
pub type Txdata7R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA7` writer - data7 to be transmitted\n\n32 bits data"]
pub type Txdata7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data7 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata7(&self) -> Txdata7R {
        Txdata7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data7 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata7(&mut self) -> Txdata7W<Rki2cTxdata7Spec> {
        Txdata7W::new(self, 0)
    }
}
#[doc = "I2C tx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cTxdata7Spec;
impl crate::RegisterSpec for Rki2cTxdata7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_txdata7::R`](R) reader structure"]
impl crate::Readable for Rki2cTxdata7Spec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_txdata7::W`](W) writer structure"]
impl crate::Writable for Rki2cTxdata7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_TXDATA7 to value 0"]
impl crate::Resettable for Rki2cTxdata7Spec {
    const RESET_VALUE: u32 = 0;
}
