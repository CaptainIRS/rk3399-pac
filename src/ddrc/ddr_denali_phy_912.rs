#[doc = "Register `DDR_DENALI_PHY_912` reader"]
pub type R = crate::R<DdrDenaliPhy912Spec>;
#[doc = "Register `DDR_DENALI_PHY_912` writer"]
pub type W = crate::W<DdrDenaliPhy912Spec>;
#[doc = "Field `PHY_PLL_BYPASS` reader - PHY clock PLL bypass select."]
pub type PhyPllBypassR = crate::FieldReader;
#[doc = "Field `PHY_PLL_BYPASS` writer - PHY clock PLL bypass select."]
pub type PhyPllBypassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PHY clock PLL bypass select."]
    #[inline(always)]
    pub fn phy_pll_bypass(&self) -> PhyPllBypassR {
        PhyPllBypassR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PHY clock PLL bypass select."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_bypass(&mut self) -> PhyPllBypassW<DdrDenaliPhy912Spec> {
        PhyPllBypassW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_912::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_912::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy912Spec;
impl crate::RegisterSpec for DdrDenaliPhy912Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_912::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy912Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_912::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy912Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_912 to value 0"]
impl crate::Resettable for DdrDenaliPhy912Spec {
    const RESET_VALUE: u32 = 0;
}
