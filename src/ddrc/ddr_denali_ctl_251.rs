#[doc = "Register `DDR_DENALI_CTL_251` reader"]
pub type R = crate::R<DdrDenaliCtl251Spec>;
#[doc = "Register `DDR_DENALI_CTL_251` writer"]
pub type W = crate::W<DdrDenaliCtl251Spec>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F2` reader - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F2` writer - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_TIMEOUT_F2` reader - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_TIMEOUT_F2` writer - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_high_threshold_f2(&self) -> RdlvlHighThresholdF2R {
        RdlvlHighThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_timeout_f2(&self) -> RdlvlTimeoutF2R {
        RdlvlTimeoutF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_high_threshold_f2(&mut self) -> RdlvlHighThresholdF2W<DdrDenaliCtl251Spec> {
        RdlvlHighThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_timeout_f2(&mut self) -> RdlvlTimeoutF2W<DdrDenaliCtl251Spec> {
        RdlvlTimeoutF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_251::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_251::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl251Spec;
impl crate::RegisterSpec for DdrDenaliCtl251Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_251::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl251Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_251::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl251Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_251 to value 0"]
impl crate::Resettable for DdrDenaliCtl251Spec {
    const RESET_VALUE: u32 = 0;
}
