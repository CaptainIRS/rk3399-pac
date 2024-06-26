#[doc = "Register `DENALI_PHY_911` reader"]
pub type R = crate::R<DenaliPhy911Spec>;
#[doc = "Register `DENALI_PHY_911` writer"]
pub type W = crate::W<DenaliPhy911Spec>;
#[doc = "Field `PHY_PLL_CTRL` reader - PHY clock PLL controls."]
pub type PhyPllCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_CTRL` writer - PHY clock PLL controls."]
pub type PhyPllCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `PHY_PLL_CTRL_CA` reader - PHY clock PLL controls for CA 2x config."]
pub type PhyPllCtrlCaR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_CTRL_CA` writer - PHY clock PLL controls for CA 2x config."]
pub type PhyPllCtrlCaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - PHY clock PLL controls."]
    #[inline(always)]
    pub fn phy_pll_ctrl(&self) -> PhyPllCtrlR {
        PhyPllCtrlR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - PHY clock PLL controls for CA 2x config."]
    #[inline(always)]
    pub fn phy_pll_ctrl_ca(&self) -> PhyPllCtrlCaR {
        PhyPllCtrlCaR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - PHY clock PLL controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_ctrl(&mut self) -> PhyPllCtrlW<DenaliPhy911Spec> {
        PhyPllCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:28 - PHY clock PLL controls for CA 2x config."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_ctrl_ca(&mut self) -> PhyPllCtrlCaW<DenaliPhy911Spec> {
        PhyPllCtrlCaW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_911::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_911::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy911Spec;
impl crate::RegisterSpec for DenaliPhy911Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_911::R`](R) reader structure"]
impl crate::Readable for DenaliPhy911Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_911::W`](W) writer structure"]
impl crate::Writable for DenaliPhy911Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_911 to value 0"]
impl crate::Resettable for DenaliPhy911Spec {
    const RESET_VALUE: u32 = 0;
}
