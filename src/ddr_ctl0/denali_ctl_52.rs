#[doc = "Register `DENALI_CTL_52` reader"]
pub type R = crate::R<DenaliCtl52Spec>;
#[doc = "Register `DENALI_CTL_52` writer"]
pub type W = crate::W<DenaliCtl52Spec>;
#[doc = "Field `TPDEX_F0` reader - DRAM TPDEX value for frequency copy 0 in cycles."]
pub type TpdexF0R = crate::FieldReader<u16>;
#[doc = "Field `TPDEX_F0` writer - DRAM TPDEX value for frequency copy 0 in cycles."]
pub type TpdexF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TPDEX_F1` reader - DRAM TPDEX value for frequency copy 1 in cycles."]
pub type TpdexF1R = crate::FieldReader<u16>;
#[doc = "Field `TPDEX_F1` writer - DRAM TPDEX value for frequency copy 1 in cycles."]
pub type TpdexF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRAM TPDEX value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tpdex_f0(&self) -> TpdexF0R {
        TpdexF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TPDEX value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tpdex_f1(&self) -> TpdexF1R {
        TpdexF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TPDEX value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tpdex_f0(&mut self) -> TpdexF0W<DenaliCtl52Spec> {
        TpdexF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TPDEX value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tpdex_f1(&mut self) -> TpdexF1W<DenaliCtl52Spec> {
        TpdexF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl52Spec;
impl crate::RegisterSpec for DenaliCtl52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_52::R`](R) reader structure"]
impl crate::Readable for DenaliCtl52Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_52::W`](W) writer structure"]
impl crate::Writable for DenaliCtl52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_52 to value 0"]
impl crate::Resettable for DenaliCtl52Spec {
    const RESET_VALUE: u32 = 0;
}
