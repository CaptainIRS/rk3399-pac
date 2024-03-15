#[doc = "Register `DENALI_CTL_36` reader"]
pub type R = crate::R<DenaliCtl36Spec>;
#[doc = "Register `DENALI_CTL_36` writer"]
pub type W = crate::W<DenaliCtl36Spec>;
#[doc = "Field `TCKESR_F1` reader - Minimum CKE low pulse width during a self-refresh for frequency copy 1."]
pub type TckesrF1R = crate::FieldReader;
#[doc = "Field `TCKESR_F1` writer - Minimum CKE low pulse width during a self-refresh for frequency copy 1."]
pub type TckesrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRTP_F2` reader - DRAM TRTP value for frequency copy 2 in cycles."]
pub type TrtpF2R = crate::FieldReader;
#[doc = "Field `TRTP_F2` writer - DRAM TRTP value for frequency copy 2 in cycles."]
pub type TrtpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_F2` reader - DRAM TMRD value for frequency copy 2 in cycles."]
pub type TmrdF2R = crate::FieldReader;
#[doc = "Field `TMRD_F2` writer - DRAM TMRD value for frequency copy 2 in cycles."]
pub type TmrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F2` reader - DRAM TMOD value for frequency copy 2 in cycles."]
pub type TmodF2R = crate::FieldReader;
#[doc = "Field `TMOD_F2` writer - DRAM TMOD value for frequency copy 2 in cycles."]
pub type TmodF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 1."]
    #[inline(always)]
    pub fn tckesr_f1(&self) -> TckesrF1R {
        TckesrF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRTP value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trtp_f2(&self) -> TrtpF2R {
        TrtpF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TMRD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tmrd_f2(&self) -> TmrdF2R {
        TmrdF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TMOD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tmod_f2(&self) -> TmodF2R {
        TmodF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CKE low pulse width during a self-refresh for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f1(&mut self) -> TckesrF1W<DenaliCtl36Spec> {
        TckesrF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRTP value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f2(&mut self) -> TrtpF2W<DenaliCtl36Spec> {
        TrtpF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TMRD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f2(&mut self) -> TmrdF2W<DenaliCtl36Spec> {
        TmrdF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TMOD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f2(&mut self) -> TmodF2W<DenaliCtl36Spec> {
        TmodF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl36Spec;
impl crate::RegisterSpec for DenaliCtl36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_36::R`](R) reader structure"]
impl crate::Readable for DenaliCtl36Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_36::W`](W) writer structure"]
impl crate::Writable for DenaliCtl36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_36 to value 0"]
impl crate::Resettable for DenaliCtl36Spec {
    const RESET_VALUE: u32 = 0;
}
