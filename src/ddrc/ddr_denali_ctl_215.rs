#[doc = "Register `DDR_DENALI_CTL_215` reader"]
pub type R = crate::R<DdrDenaliCtl215Spec>;
#[doc = "Register `DDR_DENALI_CTL_215` writer"]
pub type W = crate::W<DdrDenaliCtl215Spec>;
#[doc = "Field `WR_TO_ODTH_F2` reader - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF2R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F2` writer - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_TO_ODTH_F0` reader - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF0R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F0` writer - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_TO_ODTH_F1` reader - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF1R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F1` writer - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_TO_ODTH_F2` reader - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF2R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F2` writer - Defines the delay from a read command to ODT assertion."]
pub type RdToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    pub fn wr_to_odth_f2(&self) -> WrToOdthF2R {
        WrToOdthF2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    pub fn rd_to_odth_f0(&self) -> RdToOdthF0R {
        RdToOdthF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    pub fn rd_to_odth_f1(&self) -> RdToOdthF1R {
        RdToOdthF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    pub fn rd_to_odth_f2(&self) -> RdToOdthF2R {
        RdToOdthF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f2(&mut self) -> WrToOdthF2W<DdrDenaliCtl215Spec> {
        WrToOdthF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f0(&mut self) -> RdToOdthF0W<DdrDenaliCtl215Spec> {
        RdToOdthF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f1(&mut self) -> RdToOdthF1W<DdrDenaliCtl215Spec> {
        RdToOdthF1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the delay from a read command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f2(&mut self) -> RdToOdthF2W<DdrDenaliCtl215Spec> {
        RdToOdthF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_215::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_215::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl215Spec;
impl crate::RegisterSpec for DdrDenaliCtl215Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_215::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl215Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_215::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl215Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_215 to value 0"]
impl crate::Resettable for DdrDenaliCtl215Spec {
    const RESET_VALUE: u32 = 0;
}
