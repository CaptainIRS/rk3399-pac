#[doc = "Register `DDR_DENALI_PHY_272` reader"]
pub type R = crate::R<DdrDenaliPhy272Spec>;
#[doc = "Register `DDR_DENALI_PHY_272` writer"]
pub type W = crate::W<DdrDenaliPhy272Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT8_2` reader - LPDDR4 read leveling pattern 8 data for slice 2."]
pub type PhyLp4RdlvlPatt8_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT8_2` writer - LPDDR4 read leveling pattern 8 data for slice 2."]
pub type PhyLp4RdlvlPatt8_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 8 data for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt8_2(&self) -> PhyLp4RdlvlPatt8_2R {
        PhyLp4RdlvlPatt8_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 8 data for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt8_2(&mut self) -> PhyLp4RdlvlPatt8_2W<DdrDenaliPhy272Spec> {
        PhyLp4RdlvlPatt8_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_272::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_272::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy272Spec;
impl crate::RegisterSpec for DdrDenaliPhy272Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_272::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy272Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_272::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy272Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_272 to value 0"]
impl crate::Resettable for DdrDenaliPhy272Spec {
    const RESET_VALUE: u32 = 0;
}
