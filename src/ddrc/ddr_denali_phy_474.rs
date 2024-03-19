#[doc = "Register `DDR_DENALI_PHY_474` reader"]
pub type R = crate::R<DdrDenaliPhy474Spec>;
#[doc = "Register `DDR_DENALI_PHY_474` writer"]
pub type W = crate::W<DdrDenaliPhy474Spec>;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_3` reader - DQ slave delay step size during write data leveling for slice 3."]
pub type PhyWdqlvlDlyStep3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_3` writer - DQ slave delay step size during write data leveling for slice 3."]
pub type PhyWdqlvlDlyStep3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDLVL_DLY_STEP_3` reader - DQS slave delay step size during read leveling for slice 3."]
pub type PhyRdlvlDlyStep3R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DLY_STEP_3` writer - DQS slave delay step size during read leveling for slice 3."]
pub type PhyRdlvlDlyStep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_dly_step_3(&self) -> PhyWdqlvlDlyStep3R {
        PhyWdqlvlDlyStep3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_dly_step_3(&self) -> PhyRdlvlDlyStep3R {
        PhyRdlvlDlyStep3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dly_step_3(&mut self) -> PhyWdqlvlDlyStep3W<DdrDenaliPhy474Spec> {
        PhyWdqlvlDlyStep3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_dly_step_3(&mut self) -> PhyRdlvlDlyStep3W<DdrDenaliPhy474Spec> {
        PhyRdlvlDlyStep3W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_474::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_474::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy474Spec;
impl crate::RegisterSpec for DdrDenaliPhy474Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_474::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy474Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_474::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy474Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_474 to value 0"]
impl crate::Resettable for DdrDenaliPhy474Spec {
    const RESET_VALUE: u32 = 0;
}
