#[doc = "Register `DDR_DENALI_CTL_34` reader"]
pub type R = crate::R<DdrDenaliCtl34Spec>;
#[doc = "Register `DDR_DENALI_CTL_34` writer"]
pub type W = crate::W<DdrDenaliCtl34Spec>;
#[doc = "Field `TCKESR_F0` reader - Minimum CKE low pulse width during a self-refresh for frequency copy 0."]
pub type TckesrF0R = crate::FieldReader;
#[doc = "Field `TCKESR_F0` writer - Minimum CKE low pulse width during a self-refresh for frequency copy 0."]
pub type TckesrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRTP_F1` reader - DRAM TRTP value for frequency copy 1 in cycles."]
pub type TrtpF1R = crate::FieldReader;
#[doc = "Field `TRTP_F1` writer - DRAM TRTP value for frequency copy 1 in cycles."]
pub type TrtpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_F1` reader - DRAM TMRD value for frequency copy 1 in cycles."]
pub type TmrdF1R = crate::FieldReader;
#[doc = "Field `TMRD_F1` writer - DRAM TMRD value for frequency copy 1 in cycles."]
pub type TmrdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F1` reader - DRAM TMOD value for frequency copy 1 in cycles."]
pub type TmodF1R = crate::FieldReader;
#[doc = "Field `TMOD_F1` writer - DRAM TMOD value for frequency copy 1 in cycles."]
pub type TmodF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 0."]
    #[inline(always)]
    pub fn tckesr_f0(&self) -> TckesrF0R {
        TckesrF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRTP value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trtp_f1(&self) -> TrtpF1R {
        TrtpF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TMRD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tmrd_f1(&self) -> TmrdF1R {
        TmrdF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TMOD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tmod_f1(&self) -> TmodF1R {
        TmodF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f0(&mut self) -> TckesrF0W<DdrDenaliCtl34Spec> {
        TckesrF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRTP value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f1(&mut self) -> TrtpF1W<DdrDenaliCtl34Spec> {
        TrtpF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TMRD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f1(&mut self) -> TmrdF1W<DdrDenaliCtl34Spec> {
        TmrdF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TMOD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f1(&mut self) -> TmodF1W<DdrDenaliCtl34Spec> {
        TmodF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl34Spec;
impl crate::RegisterSpec for DdrDenaliCtl34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_34::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl34Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_34::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_34 to value 0"]
impl crate::Resettable for DdrDenaliCtl34Spec {
    const RESET_VALUE: u32 = 0;
}
