#[doc = "Register `DENALI_PHY_401` reader"]
pub type R = crate::R<DenaliPhy401Spec>;
#[doc = "Register `DENALI_PHY_401` writer"]
pub type W = crate::W<DenaliPhy401Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT9_3` reader - LPDDR4 read leveling pattern 9 data for slice 3."]
pub type PhyLp4RdlvlPatt9_3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT9_3` writer - LPDDR4 read leveling pattern 9 data for slice 3."]
pub type PhyLp4RdlvlPatt9_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 9 data for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt9_3(&self) -> PhyLp4RdlvlPatt9_3R {
        PhyLp4RdlvlPatt9_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 9 data for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt9_3(&mut self) -> PhyLp4RdlvlPatt9_3W<DenaliPhy401Spec> {
        PhyLp4RdlvlPatt9_3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_401::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_401::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy401Spec;
impl crate::RegisterSpec for DenaliPhy401Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_401::R`](R) reader structure"]
impl crate::Readable for DenaliPhy401Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_401::W`](W) writer structure"]
impl crate::Writable for DenaliPhy401Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_401 to value 0"]
impl crate::Resettable for DenaliPhy401Spec {
    const RESET_VALUE: u32 = 0;
}
