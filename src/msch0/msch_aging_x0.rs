#[doc = "Register `MSCH_AgingX0` reader"]
pub type R = crate::R<MschAgingX0Spec>;
#[doc = "Register `MSCH_AgingX0` writer"]
pub type W = crate::W<MschAgingX0Spec>;
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
    pub fn agingx0(&mut self) -> Agingx0W<MschAgingX0Spec> {
        Agingx0W::new(self, 0)
    }
}
#[doc = "Aging threshold multiplicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_aging_x0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_aging_x0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschAgingX0Spec;
impl crate::RegisterSpec for MschAgingX0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_aging_x0::R`](R) reader structure"]
impl crate::Readable for MschAgingX0Spec {}
#[doc = "`write(|w| ..)` method takes [`msch_aging_x0::W`](W) writer structure"]
impl crate::Writable for MschAgingX0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_AgingX0 to value 0"]
impl crate::Resettable for MschAgingX0Spec {
    const RESET_VALUE: u32 = 0;
}
