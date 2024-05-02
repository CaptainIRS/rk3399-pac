#[doc = "Register `FILT_FAC_SH0` reader"]
pub type R = crate::R<FiltFacSh0Spec>;
#[doc = "Register `FILT_FAC_SH0` writer"]
pub type W = crate::W<FiltFacSh0Spec>;
#[doc = "Field `filt_fac_sh0` reader - Filter factor for sharp0 level\n\n"]
pub type FiltFacSh0R = crate::FieldReader;
#[doc = "Field `filt_fac_sh0` writer - Filter factor for sharp0 level\n\n"]
pub type FiltFacSh0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Filter factor for sharp0 level\n\n"]
    #[inline(always)]
    pub fn filt_fac_sh0(&self) -> FiltFacSh0R {
        FiltFacSh0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Filter factor for sharp0 level\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn filt_fac_sh0(&mut self) -> FiltFacSh0W<FiltFacSh0Spec> {
        FiltFacSh0W::new(self, 0)
    }
}
#[doc = "filter factor sharp0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_sh0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_sh0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltFacSh0Spec;
impl crate::RegisterSpec for FiltFacSh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_fac_sh0::R`](R) reader structure"]
impl crate::Readable for FiltFacSh0Spec {}
#[doc = "`write(|w| ..)` method takes [`filt_fac_sh0::W`](W) writer structure"]
impl crate::Writable for FiltFacSh0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_FAC_SH0 to value 0x0c"]
impl crate::Resettable for FiltFacSh0Spec {
    const RESET_VALUE: u32 = 0x0c;
}
