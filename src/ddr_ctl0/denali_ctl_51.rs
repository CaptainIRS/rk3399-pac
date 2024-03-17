#[doc = "Register `DENALI_CTL_51` reader"]
pub type R = crate::R<DenaliCtl51Spec>;
#[doc = "Register `DENALI_CTL_51` writer"]
pub type W = crate::W<DenaliCtl51Spec>;
#[doc = "Field `TREF_INTERVAL` reader - Defines the cycles between refreshes to different chip selects."]
pub type TrefIntervalR = crate::FieldReader<u16>;
#[doc = "Field `TREF_INTERVAL` writer - Defines the cycles between refreshes to different chip selects."]
pub type TrefIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    pub fn tref_interval(&self) -> TrefIntervalR {
        TrefIntervalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    #[must_use]
    pub fn tref_interval(&mut self) -> TrefIntervalW<DenaliCtl51Spec> {
        TrefIntervalW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl51Spec;
impl crate::RegisterSpec for DenaliCtl51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_51::R`](R) reader structure"]
impl crate::Readable for DenaliCtl51Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_51::W`](W) writer structure"]
impl crate::Writable for DenaliCtl51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_51 to value 0"]
impl crate::Resettable for DenaliCtl51Spec {
    const RESET_VALUE: u32 = 0;
}
