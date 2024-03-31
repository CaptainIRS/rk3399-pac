#[doc = "Register `TXDATA2` reader"]
pub type R = crate::R<Txdata2Spec>;
#[doc = "Register `TXDATA2` writer"]
pub type W = crate::W<Txdata2Spec>;
#[doc = "Field `TXDATA2` reader - data2 to be transmitted\n\n32 bits data"]
pub type Txdata2R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA2` writer - data2 to be transmitted\n\n32 bits data"]
pub type Txdata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data2 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata2(&self) -> Txdata2R {
        Txdata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data2 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata2(&mut self) -> Txdata2W<Txdata2Spec> {
        Txdata2W::new(self, 0)
    }
}
#[doc = "I2C tx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata2Spec;
impl crate::RegisterSpec for Txdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata2::R`](R) reader structure"]
impl crate::Readable for Txdata2Spec {}
#[doc = "`write(|w| ..)` method takes [`txdata2::W`](W) writer structure"]
impl crate::Writable for Txdata2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA2 to value 0"]
impl crate::Resettable for Txdata2Spec {
    const RESET_VALUE: u32 = 0;
}
