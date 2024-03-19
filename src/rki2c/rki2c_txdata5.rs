#[doc = "Register `RKI2C_TXDATA5` reader"]
pub type R = crate::R<Rki2cTxdata5Spec>;
#[doc = "Register `RKI2C_TXDATA5` writer"]
pub type W = crate::W<Rki2cTxdata5Spec>;
#[doc = "Field `TXDATA5` reader - data5 to be transmitted\n\n32 bits data"]
pub type Txdata5R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA5` writer - data5 to be transmitted\n\n32 bits data"]
pub type Txdata5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data5 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata5(&self) -> Txdata5R {
        Txdata5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data5 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata5(&mut self) -> Txdata5W<Rki2cTxdata5Spec> {
        Txdata5W::new(self, 0)
    }
}
#[doc = "I2C tx data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cTxdata5Spec;
impl crate::RegisterSpec for Rki2cTxdata5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_txdata5::R`](R) reader structure"]
impl crate::Readable for Rki2cTxdata5Spec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_txdata5::W`](W) writer structure"]
impl crate::Writable for Rki2cTxdata5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_TXDATA5 to value 0"]
impl crate::Resettable for Rki2cTxdata5Spec {
    const RESET_VALUE: u32 = 0;
}
