#[doc = "Register `SWREG_47` reader"]
pub type R = crate::R<Swreg47Spec>;
#[doc = "Register `SWREG_47` writer"]
pub type W = crate::W<Swreg47Spec>;
#[doc = "Field `CIR_INTRA_MB_ITVL` reader - the interval for cir intra macro block\n\n0: disable\n\nother: enable and be set"]
pub type CirIntraMbItvlR = crate::FieldReader<u16>;
#[doc = "Field `CIR_INTRA_MB_ITVL` writer - the interval for cir intra macro block\n\n0: disable\n\nother: enable and be set"]
pub type CirIntraMbItvlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CIR_FIRST_INTRA` reader - the first macro block sellected for cir\n\n0:disable\n\nother:enable and be set"]
pub type CirFirstIntraR = crate::FieldReader<u16>;
#[doc = "Field `CIR_FIRST_INTRA` writer - the first macro block sellected for cir\n\n0:disable\n\nother:enable and be set"]
pub type CirFirstIntraW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the interval for cir intra macro block\n\n0: disable\n\nother: enable and be set"]
    #[inline(always)]
    pub fn cir_intra_mb_itvl(&self) -> CirIntraMbItvlR {
        CirIntraMbItvlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the first macro block sellected for cir\n\n0:disable\n\nother:enable and be set"]
    #[inline(always)]
    pub fn cir_first_intra(&self) -> CirFirstIntraR {
        CirFirstIntraR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the interval for cir intra macro block\n\n0: disable\n\nother: enable and be set"]
    #[inline(always)]
    #[must_use]
    pub fn cir_intra_mb_itvl(&mut self) -> CirIntraMbItvlW<Swreg47Spec> {
        CirIntraMbItvlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - the first macro block sellected for cir\n\n0:disable\n\nother:enable and be set"]
    #[inline(always)]
    #[must_use]
    pub fn cir_first_intra(&mut self) -> CirFirstIntraW<Swreg47Spec> {
        CirFirstIntraW::new(self, 16)
    }
}
#[doc = "CIR intra control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg47Spec;
impl crate::RegisterSpec for Swreg47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_47::R`](R) reader structure"]
impl crate::Readable for Swreg47Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_47::W`](W) writer structure"]
impl crate::Writable for Swreg47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_47 to value 0"]
impl crate::Resettable for Swreg47Spec {
    const RESET_VALUE: u32 = 0;
}
