#[doc = "Register `FILT_FAC_BL1` reader"]
pub type R = crate::R<FiltFacBl1Spec>;
#[doc = "Register `FILT_FAC_BL1` writer"]
pub type W = crate::W<FiltFacBl1Spec>;
#[doc = "Field `filt_fac_bl1` reader - Filter factor for blur 1 level (max blur)"]
pub type FiltFacBl1R = crate::FieldReader;
#[doc = "Field `filt_fac_bl1` writer - Filter factor for blur 1 level (max blur)"]
pub type FiltFacBl1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Filter factor for blur 1 level (max blur)"]
    #[inline(always)]
    pub fn filt_fac_bl1(&self) -> FiltFacBl1R {
        FiltFacBl1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Filter factor for blur 1 level (max blur)"]
    #[inline(always)]
    #[must_use]
    pub fn filt_fac_bl1(&mut self) -> FiltFacBl1W<FiltFacBl1Spec> {
        FiltFacBl1W::new(self, 0)
    }
}
#[doc = "Parameter for blur 1 filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_bl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_bl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltFacBl1Spec;
impl crate::RegisterSpec for FiltFacBl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_fac_bl1::R`](R) reader structure"]
impl crate::Readable for FiltFacBl1Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_fac_bl1::W`](W) writer structure"]
impl crate::Writable for FiltFacBl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_FAC_BL1 to value 0x02"]
impl crate::Resettable for FiltFacBl1Spec {
    const RESET_VALUE: u32 = 0x02;
}
