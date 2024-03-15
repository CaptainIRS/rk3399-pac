#[doc = "Register `RKI2C_TXDATA2` reader"]
pub type R = crate::R<Rki2cTxdata2Spec>;
#[doc = "Register `RKI2C_TXDATA2` writer"]
pub type W = crate::W<Rki2cTxdata2Spec>;
#[doc = "Field `TXDATA2` reader - data2 to be transmitted 32 bits data"]
pub type Txdata2R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA2` writer - data2 to be transmitted 32 bits data"]
pub type Txdata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data2 to be transmitted 32 bits data"]
    #[inline(always)]
    pub fn txdata2(&self) -> Txdata2R {
        Txdata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data2 to be transmitted 32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata2(&mut self) -> Txdata2W<Rki2cTxdata2Spec> {
        Txdata2W::new(self, 0)
    }
}
#[doc = "I2C tx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_txdata2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_txdata2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cTxdata2Spec;
impl crate::RegisterSpec for Rki2cTxdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_txdata2::R`](R) reader structure"]
impl crate::Readable for Rki2cTxdata2Spec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_txdata2::W`](W) writer structure"]
impl crate::Writable for Rki2cTxdata2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_TXDATA2 to value 0"]
impl crate::Resettable for Rki2cTxdata2Spec {
    const RESET_VALUE: u32 = 0;
}
