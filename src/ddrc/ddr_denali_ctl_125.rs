#[doc = "Register `DDR_DENALI_CTL_125` reader"]
pub type R = crate::R<DdrDenaliCtl125Spec>;
#[doc = "Register `DDR_DENALI_CTL_125` writer"]
pub type W = crate::W<DdrDenaliCtl125Spec>;
#[doc = "Field `TVRCG_ENABLE_F1` reader - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF1R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_ENABLE_F1` writer - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TVRCG_DISABLE_F1` reader - JEDEC TVRCG_DISABLE time."]
pub type TvrcgDisableF1R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_DISABLE_F1` writer - JEDEC TVRCG_DISABLE time."]
pub type TvrcgDisableF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    pub fn tvrcg_enable_f1(&self) -> TvrcgEnableF1R {
        TvrcgEnableF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_DISABLE time."]
    #[inline(always)]
    pub fn tvrcg_disable_f1(&self) -> TvrcgDisableF1R {
        TvrcgDisableF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_enable_f1(&mut self) -> TvrcgEnableF1W<DdrDenaliCtl125Spec> {
        TvrcgEnableF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_DISABLE time."]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_disable_f1(&mut self) -> TvrcgDisableF1W<DdrDenaliCtl125Spec> {
        TvrcgDisableF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl125Spec;
impl crate::RegisterSpec for DdrDenaliCtl125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_125::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl125Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_125::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_125 to value 0"]
impl crate::Resettable for DdrDenaliCtl125Spec {
    const RESET_VALUE: u32 = 0;
}
