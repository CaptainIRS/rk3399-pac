#[doc = "Register `DDR_DENALI_CTL_241` reader"]
pub type R = crate::R<DdrDenaliCtl241Spec>;
#[doc = "Register `DDR_DENALI_CTL_241` writer"]
pub type W = crate::W<DdrDenaliCtl241Spec>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F0` reader - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F0` writer - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_TIMEOUT_F0` reader - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_TIMEOUT_F0` writer - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_high_threshold_f0(&self) -> RdlvlHighThresholdF0R {
        RdlvlHighThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_timeout_f0(&self) -> RdlvlTimeoutF0R {
        RdlvlTimeoutF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_high_threshold_f0(&mut self) -> RdlvlHighThresholdF0W<DdrDenaliCtl241Spec> {
        RdlvlHighThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_timeout_f0(&mut self) -> RdlvlTimeoutF0W<DdrDenaliCtl241Spec> {
        RdlvlTimeoutF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_241::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_241::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl241Spec;
impl crate::RegisterSpec for DdrDenaliCtl241Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_241::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl241Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_241::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl241Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_241 to value 0"]
impl crate::Resettable for DdrDenaliCtl241Spec {
    const RESET_VALUE: u32 = 0;
}
