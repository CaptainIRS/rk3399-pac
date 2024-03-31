#[doc = "Register `DENALI_CTL_127` reader"]
pub type R = crate::R<DenaliCtl127Spec>;
#[doc = "Register `DENALI_CTL_127` writer"]
pub type W = crate::W<DenaliCtl127Spec>;
#[doc = "Field `TVREF_LONG_F1` reader - JEDEC TVREF, design will always use the long value."]
pub type TvrefLongF1R = crate::FieldReader<u16>;
#[doc = "Field `TVREF_LONG_F1` writer - JEDEC TVREF, design will always use the long value."]
pub type TvrefLongF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TVRCG_ENABLE_F2` reader - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF2R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_ENABLE_F2` writer - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - JEDEC TVREF, design will always use the long value."]
    #[inline(always)]
    pub fn tvref_long_f1(&self) -> TvrefLongF1R {
        TvrefLongF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    pub fn tvrcg_enable_f2(&self) -> TvrcgEnableF2R {
        TvrcgEnableF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - JEDEC TVREF, design will always use the long value."]
    #[inline(always)]
    #[must_use]
    pub fn tvref_long_f1(&mut self) -> TvrefLongF1W<DenaliCtl127Spec> {
        TvrefLongF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_enable_f2(&mut self) -> TvrcgEnableF2W<DenaliCtl127Spec> {
        TvrcgEnableF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_127::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_127::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl127Spec;
impl crate::RegisterSpec for DenaliCtl127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_127::R`](R) reader structure"]
impl crate::Readable for DenaliCtl127Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_127::W`](W) writer structure"]
impl crate::Writable for DenaliCtl127Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_127 to value 0"]
impl crate::Resettable for DenaliCtl127Spec {
    const RESET_VALUE: u32 = 0;
}
