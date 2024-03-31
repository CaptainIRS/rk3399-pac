#[doc = "Register `TXDATA4` reader"]
pub type R = crate::R<Txdata4Spec>;
#[doc = "Register `TXDATA4` writer"]
pub type W = crate::W<Txdata4Spec>;
#[doc = "Field `TXDATA4` reader - data4 to be transmitted\n\n32 bits data"]
pub type Txdata4R = crate::FieldReader<u32>;
#[doc = "Field `TXDATA4` writer - data4 to be transmitted\n\n32 bits data"]
pub type Txdata4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data4 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    pub fn txdata4(&self) -> Txdata4R {
        Txdata4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data4 to be transmitted\n\n32 bits data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata4(&mut self) -> Txdata4W<Txdata4Spec> {
        Txdata4W::new(self, 0)
    }
}
#[doc = "I2C tx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata4Spec;
impl crate::RegisterSpec for Txdata4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata4::R`](R) reader structure"]
impl crate::Readable for Txdata4Spec {}
#[doc = "`write(|w| ..)` method takes [`txdata4::W`](W) writer structure"]
impl crate::Writable for Txdata4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA4 to value 0"]
impl crate::Resettable for Txdata4Spec {
    const RESET_VALUE: u32 = 0;
}
