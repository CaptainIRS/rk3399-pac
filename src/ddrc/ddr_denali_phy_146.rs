#[doc = "Register `DDR_DENALI_PHY_146` reader"]
pub type R = crate::R<DdrDenaliPhy146Spec>;
#[doc = "Register `DDR_DENALI_PHY_146` writer"]
pub type W = crate::W<DdrDenaliPhy146Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT10_1` reader - LPDDR4 read leveling pattern 10 data for slice 1."]
pub type PhyLp4RdlvlPatt10_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT10_1` writer - LPDDR4 read leveling pattern 10 data for slice 1."]
pub type PhyLp4RdlvlPatt10_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 10 data for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt10_1(&self) -> PhyLp4RdlvlPatt10_1R {
        PhyLp4RdlvlPatt10_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 10 data for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt10_1(&mut self) -> PhyLp4RdlvlPatt10_1W<DdrDenaliPhy146Spec> {
        PhyLp4RdlvlPatt10_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_146::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_146::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy146Spec;
impl crate::RegisterSpec for DdrDenaliPhy146Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_146::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy146Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_146::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy146Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_146 to value 0"]
impl crate::Resettable for DdrDenaliPhy146Spec {
    const RESET_VALUE: u32 = 0;
}
