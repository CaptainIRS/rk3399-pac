#[doc = "Register `DDR_DENALI_PHY_922` reader"]
pub type R = crate::R<DdrDenaliPhy922Spec>;
#[doc = "Register `DDR_DENALI_PHY_922` writer"]
pub type W = crate::W<DdrDenaliPhy922Spec>;
#[doc = "Field `PHY_PLL_OBS_3` reader - PHY TOP level clock PLL_3 observe values."]
pub type PhyPllObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_TESTOUT_SEL` reader - PHY PLL testout select."]
pub type PhyPllTestoutSelR = crate::FieldReader;
#[doc = "Field `PHY_PLL_TESTOUT_SEL` writer - PHY PLL testout select."]
pub type PhyPllTestoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_TCKSRE_WAIT` reader - Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
pub type PhyTcksreWaitR = crate::FieldReader;
#[doc = "Field `PHY_TCKSRE_WAIT` writer - Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
pub type PhyTcksreWaitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - PHY TOP level clock PLL_3 observe values."]
    #[inline(always)]
    pub fn phy_pll_obs_3(&self) -> PhyPllObs3R {
        PhyPllObs3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - PHY PLL testout select."]
    #[inline(always)]
    pub fn phy_pll_testout_sel(&self) -> PhyPllTestoutSelR {
        PhyPllTestoutSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
    #[inline(always)]
    pub fn phy_tcksre_wait(&self) -> PhyTcksreWaitR {
        PhyTcksreWaitR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - PHY PLL testout select."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_testout_sel(&mut self) -> PhyPllTestoutSelW<DdrDenaliPhy922Spec> {
        PhyPllTestoutSelW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Specifies the number of cycles the PHY should wait before turning off the PLL for a deep sleep or DFS event."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tcksre_wait(&mut self) -> PhyTcksreWaitW<DdrDenaliPhy922Spec> {
        PhyTcksreWaitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_922::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_922::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy922Spec;
impl crate::RegisterSpec for DdrDenaliPhy922Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_922::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy922Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_922::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy922Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_922 to value 0"]
impl crate::Resettable for DdrDenaliPhy922Spec {
    const RESET_VALUE: u32 = 0;
}
