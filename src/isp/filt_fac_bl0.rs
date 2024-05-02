#[doc = "Register `FILT_FAC_BL0` reader"]
pub type R = crate::R<FiltFacBl0Spec>;
#[doc = "Register `FILT_FAC_BL0` writer"]
pub type W = crate::W<FiltFacBl0Spec>;
#[doc = "Field `filt_fac_bl0` reader - Filter factor for blur 0 level\n\n"]
pub type FiltFacBl0R = crate::FieldReader;
#[doc = "Field `filt_fac_bl0` writer - Filter factor for blur 0 level\n\n"]
pub type FiltFacBl0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Filter factor for blur 0 level\n\n"]
    #[inline(always)]
    pub fn filt_fac_bl0(&self) -> FiltFacBl0R {
        FiltFacBl0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Filter factor for blur 0 level\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn filt_fac_bl0(&mut self) -> FiltFacBl0W<FiltFacBl0Spec> {
        FiltFacBl0W::new(self, 0)
    }
}
#[doc = "Parameter for blur 0 filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_bl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_bl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltFacBl0Spec;
impl crate::RegisterSpec for FiltFacBl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_fac_bl0::R`](R) reader structure"]
impl crate::Readable for FiltFacBl0Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_fac_bl0::W`](W) writer structure"]
impl crate::Writable for FiltFacBl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_FAC_BL0 to value 0x06"]
impl crate::Resettable for FiltFacBl0Spec {
    const RESET_VALUE: u32 = 0x06;
}
