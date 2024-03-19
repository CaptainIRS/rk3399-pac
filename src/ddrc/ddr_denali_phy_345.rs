#[doc = "Register `DDR_DENALI_PHY_345` reader"]
pub type R = crate::R<DdrDenaliPhy345Spec>;
#[doc = "Register `DDR_DENALI_PHY_345` writer"]
pub type W = crate::W<DdrDenaliPhy345Spec>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_2` reader - Interim backup step delay used in gate training algorithm for slice 2."]
pub type PhyGtlvlBackStep2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_2` writer - Interim backup step delay used in gate training algorithm for slice 2."]
pub type PhyGtlvlBackStep2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_2` reader - Final backup step delay used in gate training algorithm for slice 2."]
pub type PhyGtlvlFinalStep2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_2` writer - Final backup step delay used in gate training algorithm for slice 2."]
pub type PhyGtlvlFinalStep2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_back_step_2(&self) -> PhyGtlvlBackStep2R {
        PhyGtlvlBackStep2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_final_step_2(&self) -> PhyGtlvlFinalStep2R {
        PhyGtlvlFinalStep2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interim backup step delay used in gate training algorithm for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_back_step_2(&mut self) -> PhyGtlvlBackStep2W<DdrDenaliPhy345Spec> {
        PhyGtlvlBackStep2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Final backup step delay used in gate training algorithm for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_final_step_2(&mut self) -> PhyGtlvlFinalStep2W<DdrDenaliPhy345Spec> {
        PhyGtlvlFinalStep2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_345::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_345::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy345Spec;
impl crate::RegisterSpec for DdrDenaliPhy345Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_345::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy345Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_345::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy345Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_345 to value 0"]
impl crate::Resettable for DdrDenaliPhy345Spec {
    const RESET_VALUE: u32 = 0;
}
