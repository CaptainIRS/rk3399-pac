#[doc = "Register `DENALI_CTL_53` reader"]
pub type R = crate::R<DenaliCtl53Spec>;
#[doc = "Register `DENALI_CTL_53` writer"]
pub type W = crate::W<DenaliCtl53Spec>;
#[doc = "Field `TPDEX_F2` reader - DRAM TPDEX value for frequency copy 2 in cycles."]
pub type TpdexF2R = crate::FieldReader<u16>;
#[doc = "Field `TPDEX_F2` writer - DRAM TPDEX value for frequency copy 2 in cycles."]
pub type TpdexF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXPDLL_F0` reader - DRAM TXPDLL value for frequency copy 0 in cycles."]
pub type TxpdllF0R = crate::FieldReader<u16>;
#[doc = "Field `TXPDLL_F0` writer - DRAM TXPDLL value for frequency copy 0 in cycles."]
pub type TxpdllF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRAM TPDEX value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tpdex_f2(&self) -> TpdexF2R {
        TpdexF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TXPDLL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn txpdll_f0(&self) -> TxpdllF0R {
        TxpdllF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TPDEX value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tpdex_f2(&mut self) -> TpdexF2W<DenaliCtl53Spec> {
        TpdexF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TXPDLL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn txpdll_f0(&mut self) -> TxpdllF0W<DenaliCtl53Spec> {
        TxpdllF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl53Spec;
impl crate::RegisterSpec for DenaliCtl53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_53::R`](R) reader structure"]
impl crate::Readable for DenaliCtl53Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_53::W`](W) writer structure"]
impl crate::Writable for DenaliCtl53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_53 to value 0"]
impl crate::Resettable for DenaliCtl53Spec {
    const RESET_VALUE: u32 = 0;
}
