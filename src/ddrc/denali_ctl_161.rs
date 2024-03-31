#[doc = "Register `DENALI_CTL_161` reader"]
pub type R = crate::R<DenaliCtl161Spec>;
#[doc = "Register `DENALI_CTL_161` writer"]
pub type W = crate::W<DenaliCtl161Spec>;
#[doc = "Field `RL3_SUPPORT_EN` reader - Indicates if RL3 is supported by a connected LPDDR3 memory. Data read from MR0 bit 7."]
pub type Rl3SupportEnR = crate::FieldReader;
#[doc = "Field `FSP_PHY_UPDATE_MRW` reader - Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
pub type FspPhyUpdateMrwR = crate::BitReader;
#[doc = "Field `FSP_PHY_UPDATE_MRW` writer - Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
pub type FspPhyUpdateMrwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Indicates if RL3 is supported by a connected LPDDR3 memory. Data read from MR0 bit 7."]
    #[inline(always)]
    pub fn rl3_support_en(&self) -> Rl3SupportEnR {
        Rl3SupportEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 24 - Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
    #[inline(always)]
    pub fn fsp_phy_update_mrw(&self) -> FspPhyUpdateMrwR {
        FspPhyUpdateMrwR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_phy_update_mrw(&mut self) -> FspPhyUpdateMrwW<DenaliCtl161Spec> {
        FspPhyUpdateMrwW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_161::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_161::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl161Spec;
impl crate::RegisterSpec for DenaliCtl161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_161::R`](R) reader structure"]
impl crate::Readable for DenaliCtl161Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_161::W`](W) writer structure"]
impl crate::Writable for DenaliCtl161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_161 to value 0"]
impl crate::Resettable for DenaliCtl161Spec {
    const RESET_VALUE: u32 = 0;
}
