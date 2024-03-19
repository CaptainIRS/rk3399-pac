#[doc = "Register `RKI2C_TXDATA0` reader"]
pub type R = crate::R<Rki2cTxdata0Spec>;
#[doc = "Register `RKI2C_TXDATA0` writer"]
pub type W = crate::W<Rki2cTxdata0Spec>;
#[doc = "Field `TXDATA0` reader - data0 to be transmitted\n\n32 bits data"]
pub type Txdata0R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA0` writer - data0 to be transmitted\n\n32 bits data"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data0 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata0(&self) -> Txdata0R {
        Txdata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data0 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> Txdata0W<Rki2cTxdata0Spec> {
        Txdata0W::new(self, 0)
    }
}
#[doc = "I2C tx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cTxdata0Spec;
impl crate::RegisterSpec for Rki2cTxdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_txdata0::R`](R) reader structure"]
impl crate::Readable for Rki2cTxdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_txdata0::W`](W) writer structure"]
impl crate::Writable for Rki2cTxdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_TXDATA0 to value 0"]
impl crate::Resettable for Rki2cTxdata0Spec {
    const RESET_VALUE: u32 = 0;
}
