#[doc = "Register `AgingX0` reader"]
pub type R = crate::R<AgingX0Spec>;
#[doc = "Register `AgingX0` writer"]
pub type W = crate::W<AgingX0Spec>;
#[doc = "Field `AGINGX0` reader - Aging threshold multiplicator."]
pub type Agingx0R = crate::FieldReader;
#[doc = "Field `AGINGX0` writer - Aging threshold multiplicator."]
pub type Agingx0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Aging threshold multiplicator."]
    #[inline(always)]
    pub fn agingx0(&self) -> Agingx0R {
        Agingx0R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Aging threshold multiplicator."]
    #[inline(always)]
    #[must_use]
    pub fn agingx0(&mut self) -> Agingx0W<AgingX0Spec> {
        Agingx0W::new(self, 0)
    }
}
#[doc = "Aging threshold multiplicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aging_x0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aging_x0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgingX0Spec;
impl crate::RegisterSpec for AgingX0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aging_x0::R`](R) reader structure"]
impl crate::Readable for AgingX0Spec {}
#[doc = "`write(|w| ..)` method takes [`aging_x0::W`](W) writer structure"]
impl crate::Writable for AgingX0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AgingX0 to value 0"]
impl crate::Resettable for AgingX0Spec {
    const RESET_VALUE: u32 = 0;
}
