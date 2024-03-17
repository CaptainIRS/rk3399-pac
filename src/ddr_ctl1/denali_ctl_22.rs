#[doc = "Register `DENALI_CTL_22` reader"]
pub type R = crate::R<DenaliCtl22Spec>;
#[doc = "Register `DENALI_CTL_22` writer"]
pub type W = crate::W<DenaliCtl22Spec>;
#[doc = "Field `TDLL_F0` reader - DRAM TDLL value for frequency copy 0 in cycles."]
pub type TdllF0R = crate::FieldReader<u16>;
#[doc = "Field `TDLL_F0` writer - DRAM TDLL value for frequency copy 0 in cycles."]
pub type TdllF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDLL_F1` reader - DRAM TDLL value for frequency copy 1 in cycles."]
pub type TdllF1R = crate::FieldReader<u16>;
#[doc = "Field `TDLL_F1` writer - DRAM TDLL value for frequency copy 1 in cycles."]
pub type TdllF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRAM TDLL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tdll_f0(&self) -> TdllF0R {
        TdllF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TDLL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tdll_f1(&self) -> TdllF1R {
        TdllF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TDLL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdll_f0(&mut self) -> TdllF0W<DenaliCtl22Spec> {
        TdllF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TDLL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdll_f1(&mut self) -> TdllF1W<DenaliCtl22Spec> {
        TdllF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl22Spec;
impl crate::RegisterSpec for DenaliCtl22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_22::R`](R) reader structure"]
impl crate::Readable for DenaliCtl22Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_22::W`](W) writer structure"]
impl crate::Writable for DenaliCtl22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_22 to value 0"]
impl crate::Resettable for DenaliCtl22Spec {
    const RESET_VALUE: u32 = 0;
}
