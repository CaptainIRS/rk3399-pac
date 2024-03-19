#[doc = "Register `DDR_DENALI_PHY_473` reader"]
pub type R = crate::R<DdrDenaliPhy473Spec>;
#[doc = "Register `DDR_DENALI_PHY_473` writer"]
pub type W = crate::W<DdrDenaliPhy473Spec>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_3` reader - Interim backup step delay used in gate training algorithm for slice 3."]
pub type PhyGtlvlBackStep3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_3` writer - Interim backup step delay used in gate training algorithm for slice 3."]
pub type PhyGtlvlBackStep3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_3` reader - Final backup step delay used in gate training algorithm for slice 3."]
pub type PhyGtlvlFinalStep3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_3` writer - Final backup step delay used in gate training algorithm for slice 3."]
pub type PhyGtlvlFinalStep3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 3."]
    #[inline(always)]
    pub fn phy_gtlvl_back_step_3(&self) -> PhyGtlvlBackStep3R {
        PhyGtlvlBackStep3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 3."]
    #[inline(always)]
    pub fn phy_gtlvl_final_step_3(&self) -> PhyGtlvlFinalStep3R {
        PhyGtlvlFinalStep3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_back_step_3(&mut self) -> PhyGtlvlBackStep3W<DdrDenaliPhy473Spec> {
        PhyGtlvlBackStep3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_final_step_3(&mut self) -> PhyGtlvlFinalStep3W<DdrDenaliPhy473Spec> {
        PhyGtlvlFinalStep3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_473::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_473::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy473Spec;
impl crate::RegisterSpec for DdrDenaliPhy473Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_473::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy473Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_473::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy473Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_473 to value 0"]
impl crate::Resettable for DdrDenaliPhy473Spec {
    const RESET_VALUE: u32 = 0;
}
