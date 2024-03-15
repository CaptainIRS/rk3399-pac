#[doc = "Register `DENALI_CTL_102` reader"]
pub type R = crate::R<DenaliCtl102Spec>;
#[doc = "Register `DENALI_CTL_102` writer"]
pub type W = crate::W<DenaliCtl102Spec>;
#[doc = "Field `LP_AUTO_PD_IDLE` reader - Defines the idle time until the controller will place memory in active power-down."]
pub type LpAutoPdIdleR = crate::FieldReader<u16>;
#[doc = "Field `LP_AUTO_PD_IDLE` writer - Defines the idle time until the controller will place memory in active power-down."]
pub type LpAutoPdIdleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LP_AUTO_SRPD_LITE_IDLE` reader - Defines the idle time until the controller will place memory in self- refresh power-down lite mode."]
pub type LpAutoSrpdLiteIdleR = crate::FieldReader<u16>;
#[doc = "Field `LP_AUTO_SRPD_LITE_IDLE` writer - Defines the idle time until the controller will place memory in self- refresh power-down lite mode."]
pub type LpAutoSrpdLiteIdleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Defines the idle time until the controller will place memory in active power-down."]
    #[inline(always)]
    pub fn lp_auto_pd_idle(&self) -> LpAutoPdIdleR {
        LpAutoPdIdleR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Defines the idle time until the controller will place memory in self- refresh power-down lite mode."]
    #[inline(always)]
    pub fn lp_auto_srpd_lite_idle(&self) -> LpAutoSrpdLiteIdleR {
        LpAutoSrpdLiteIdleR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the idle time until the controller will place memory in active power-down."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_pd_idle(&mut self) -> LpAutoPdIdleW<DenaliCtl102Spec> {
        LpAutoPdIdleW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Defines the idle time until the controller will place memory in self- refresh power-down lite mode."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_srpd_lite_idle(&mut self) -> LpAutoSrpdLiteIdleW<DenaliCtl102Spec> {
        LpAutoSrpdLiteIdleW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl102Spec;
impl crate::RegisterSpec for DenaliCtl102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_102::R`](R) reader structure"]
impl crate::Readable for DenaliCtl102Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_102::W`](W) writer structure"]
impl crate::Writable for DenaliCtl102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_102 to value 0"]
impl crate::Resettable for DenaliCtl102Spec {
    const RESET_VALUE: u32 = 0;
}
