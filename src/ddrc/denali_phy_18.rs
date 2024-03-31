#[doc = "Register `DENALI_PHY_18` reader"]
pub type R = crate::R<DenaliPhy18Spec>;
#[doc = "Register `DENALI_PHY_18` writer"]
pub type W = crate::W<DenaliPhy18Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT10_0` reader - LPDDR4 read leveling pattern 10 data for slice 0."]
pub type PhyLp4RdlvlPatt10_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT10_0` writer - LPDDR4 read leveling pattern 10 data for slice 0."]
pub type PhyLp4RdlvlPatt10_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 10 data for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt10_0(&self) -> PhyLp4RdlvlPatt10_0R {
        PhyLp4RdlvlPatt10_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 10 data for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt10_0(&mut self) -> PhyLp4RdlvlPatt10_0W<DenaliPhy18Spec> {
        PhyLp4RdlvlPatt10_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy18Spec;
impl crate::RegisterSpec for DenaliPhy18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_18::R`](R) reader structure"]
impl crate::Readable for DenaliPhy18Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_18::W`](W) writer structure"]
impl crate::Writable for DenaliPhy18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_18 to value 0"]
impl crate::Resettable for DenaliPhy18Spec {
    const RESET_VALUE: u32 = 0;
}
