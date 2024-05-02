#[doc = "Register `FILT_FAC_SH1` reader"]
pub type R = crate::R<FiltFacSh1Spec>;
#[doc = "Register `FILT_FAC_SH1` writer"]
pub type W = crate::W<FiltFacSh1Spec>;
#[doc = "Field `filt_fac_sh1` reader - Filter factor for sharp1 level\n\n"]
pub type FiltFacSh1R = crate::FieldReader;
#[doc = "Field `filt_fac_sh1` writer - Filter factor for sharp1 level\n\n"]
pub type FiltFacSh1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Filter factor for sharp1 level\n\n"]
    #[inline(always)]
    pub fn filt_fac_sh1(&self) -> FiltFacSh1R {
        FiltFacSh1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Filter factor for sharp1 level\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn filt_fac_sh1(&mut self) -> FiltFacSh1W<FiltFacSh1Spec> {
        FiltFacSh1W::new(self, 0)
    }
}
#[doc = "filter factor sharp1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_sh1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_sh1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltFacSh1Spec;
impl crate::RegisterSpec for FiltFacSh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_fac_sh1::R`](R) reader structure"]
impl crate::Readable for FiltFacSh1Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_fac_sh1::W`](W) writer structure"]
impl crate::Writable for FiltFacSh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_FAC_SH1 to value 0x10"]
impl crate::Resettable for FiltFacSh1Spec {
    const RESET_VALUE: u32 = 0x10;
}
