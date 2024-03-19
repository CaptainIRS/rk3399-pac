#[doc = "Register `DDR_DENALI_PHY_920` reader"]
pub type R = crate::R<DdrDenaliPhy920Spec>;
#[doc = "Register `DDR_DENALI_PHY_920` writer"]
pub type W = crate::W<DdrDenaliPhy920Spec>;
#[doc = "Field `PHY_PLL_CTRL_OVERRIDE` reader - Individual PHY clock PLL control overrides."]
pub type PhyPllCtrlOverrideR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_CTRL_OVERRIDE` writer - Individual PHY clock PLL control overrides."]
pub type PhyPllCtrlOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_PLL_OBS_0` reader - PHY clock PLL_0 observe values. READ-ONLY"]
pub type PhyPllObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Individual PHY clock PLL control overrides."]
    #[inline(always)]
    pub fn phy_pll_ctrl_override(&self) -> PhyPllCtrlOverrideR {
        PhyPllCtrlOverrideR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PHY clock PLL_0 observe values. READ-ONLY"]
    #[inline(always)]
    pub fn phy_pll_obs_0(&self) -> PhyPllObs0R {
        PhyPllObs0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Individual PHY clock PLL control overrides."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_ctrl_override(&mut self) -> PhyPllCtrlOverrideW<DdrDenaliPhy920Spec> {
        PhyPllCtrlOverrideW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_920::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_920::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy920Spec;
impl crate::RegisterSpec for DdrDenaliPhy920Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_920::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy920Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_920::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy920Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_920 to value 0"]
impl crate::Resettable for DdrDenaliPhy920Spec {
    const RESET_VALUE: u32 = 0;
}
