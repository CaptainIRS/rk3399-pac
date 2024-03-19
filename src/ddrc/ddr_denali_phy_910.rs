#[doc = "Register `DDR_DENALI_PHY_910` reader"]
pub type R = crate::R<DdrDenaliPhy910Spec>;
#[doc = "Register `DDR_DENALI_PHY_910` writer"]
pub type W = crate::W<DdrDenaliPhy910Spec>;
#[doc = "Field `PHY_PLL_WAIT` reader - PHY clock PLL wait time after locking."]
pub type PhyPllWaitR = crate::FieldReader;
#[doc = "Field `PHY_PLL_WAIT` writer - PHY clock PLL wait time after locking."]
pub type PhyPllWaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - PHY clock PLL wait time after locking."]
    #[inline(always)]
    pub fn phy_pll_wait(&self) -> PhyPllWaitR {
        PhyPllWaitR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - PHY clock PLL wait time after locking."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_wait(&mut self) -> PhyPllWaitW<DdrDenaliPhy910Spec> {
        PhyPllWaitW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_910::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_910::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy910Spec;
impl crate::RegisterSpec for DdrDenaliPhy910Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_910::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy910Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_910::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy910Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_910 to value 0"]
impl crate::Resettable for DdrDenaliPhy910Spec {
    const RESET_VALUE: u32 = 0;
}
