#[doc = "Register `DENALI_PHY_275` reader"]
pub type R = crate::R<DenaliPhy275Spec>;
#[doc = "Register `DENALI_PHY_275` writer"]
pub type W = crate::W<DenaliPhy275Spec>;
#[doc = "Field `PHY_LP4_RDLVL_PATT11_2` reader - LPDDR4 read leveling pattern 11 data for slice 2."]
pub type PhyLp4RdlvlPatt11_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_LP4_RDLVL_PATT11_2` writer - LPDDR4 read leveling pattern 11 data for slice 2."]
pub type PhyLp4RdlvlPatt11_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 11 data for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_rdlvl_patt11_2(&self) -> PhyLp4RdlvlPatt11_2R {
        PhyLp4RdlvlPatt11_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDDR4 read leveling pattern 11 data for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_rdlvl_patt11_2(&mut self) -> PhyLp4RdlvlPatt11_2W<DenaliPhy275Spec> {
        PhyLp4RdlvlPatt11_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_275::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_275::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy275Spec;
impl crate::RegisterSpec for DenaliPhy275Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_275::R`](R) reader structure"]
impl crate::Readable for DenaliPhy275Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_275::W`](W) writer structure"]
impl crate::Writable for DenaliPhy275Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_275 to value 0"]
impl crate::Resettable for DenaliPhy275Spec {
    const RESET_VALUE: u32 = 0;
}
