#[doc = "Register `DENALI_CTL_123` reader"]
pub type R = crate::R<DenaliCtl123Spec>;
#[doc = "Register `DENALI_CTL_123` writer"]
pub type W = crate::W<DenaliCtl123Spec>;
#[doc = "Field `TVRCG_DISABLE_F0` reader - JEDEC TVRCG_DISABLE time."]
pub type TvrcgDisableF0R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_DISABLE_F0` writer - JEDEC TVRCG_DISABLE time."]
pub type TvrcgDisableF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TFC_F0` reader - JEDEC TFC, the frequency set point switching time."]
pub type TfcF0R = crate::FieldReader<u16>;
#[doc = "Field `TFC_F0` writer - JEDEC TFC, the frequency set point switching time."]
pub type TfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - JEDEC TVRCG_DISABLE time."]
    #[inline(always)]
    pub fn tvrcg_disable_f0(&self) -> TvrcgDisableF0R {
        TvrcgDisableF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - JEDEC TFC, the frequency set point switching time."]
    #[inline(always)]
    pub fn tfc_f0(&self) -> TfcF0R {
        TfcF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - JEDEC TVRCG_DISABLE time."]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_disable_f0(&mut self) -> TvrcgDisableF0W<DenaliCtl123Spec> {
        TvrcgDisableF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - JEDEC TFC, the frequency set point switching time."]
    #[inline(always)]
    #[must_use]
    pub fn tfc_f0(&mut self) -> TfcF0W<DenaliCtl123Spec> {
        TfcF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl123Spec;
impl crate::RegisterSpec for DenaliCtl123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_123::R`](R) reader structure"]
impl crate::Readable for DenaliCtl123Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_123::W`](W) writer structure"]
impl crate::Writable for DenaliCtl123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_123 to value 0"]
impl crate::Resettable for DenaliCtl123Spec {
    const RESET_VALUE: u32 = 0;
}
