#[doc = "Register `DENALI_PHY_921` reader"]
pub type R = crate::R<DenaliPhy921Spec>;
#[doc = "Field `PHY_PLL_OBS_1` reader - PHY clock PLL_1 observe values."]
pub type PhyPllObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_OBS_2` reader - PHY clock PLL_2 observe values."]
pub type PhyPllObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PHY clock PLL_1 observe values."]
    #[inline(always)]
    pub fn phy_pll_obs_1(&self) -> PhyPllObs1R {
        PhyPllObs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PHY clock PLL_2 observe values."]
    #[inline(always)]
    pub fn phy_pll_obs_2(&self) -> PhyPllObs2R {
        PhyPllObs2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_921::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy921Spec;
impl crate::RegisterSpec for DenaliPhy921Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_921::R`](R) reader structure"]
impl crate::Readable for DenaliPhy921Spec {}
#[doc = "`reset()` method sets DENALI_PHY_921 to value 0"]
impl crate::Resettable for DenaliPhy921Spec {
    const RESET_VALUE: u32 = 0;
}
