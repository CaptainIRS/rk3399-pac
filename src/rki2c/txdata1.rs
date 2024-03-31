#[doc = "Register `TXDATA1` reader"]
pub type R = crate::R<Txdata1Spec>;
#[doc = "Register `TXDATA1` writer"]
pub type W = crate::W<Txdata1Spec>;
#[doc = "Field `TXDATA1` reader - data1 to be transmitted\n\n32 bits data"]
pub type Txdata1R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA1` writer - data1 to be transmitted\n\n32 bits data"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data1 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata1(&self) -> Txdata1R {
        Txdata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data1 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> Txdata1W<Txdata1Spec> {
        Txdata1W::new(self, 0)
    }
}
#[doc = "I2C tx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata1Spec;
impl crate::RegisterSpec for Txdata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata1::R`](R) reader structure"]
impl crate::Readable for Txdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`txdata1::W`](W) writer structure"]
impl crate::Writable for Txdata1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA1 to value 0"]
impl crate::Resettable for Txdata1Spec {
    const RESET_VALUE: u32 = 0;
}
