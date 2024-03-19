#[doc = "Register `DDR_DENALI_PHY_218` reader"]
pub type R = crate::R<DdrDenaliPhy218Spec>;
#[doc = "Register `DDR_DENALI_PHY_218` writer"]
pub type W = crate::W<DdrDenaliPhy218Spec>;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_1` reader - DQ slave delay step size during write data leveling for slice 1."]
pub type PhyWdqlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_1` writer - DQ slave delay step size during write data leveling for slice 1."]
pub type PhyWdqlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDLVL_DLY_STEP_1` reader - DQS slave delay step size during read leveling for slice 1."]
pub type PhyRdlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DLY_STEP_1` writer - DQS slave delay step size during read leveling for slice 1."]
pub type PhyRdlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dly_step_1(&self) -> PhyWdqlvlDlyStep1R {
        PhyWdqlvlDlyStep1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_dly_step_1(&self) -> PhyRdlvlDlyStep1R {
        PhyRdlvlDlyStep1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dly_step_1(&mut self) -> PhyWdqlvlDlyStep1W<DdrDenaliPhy218Spec> {
        PhyWdqlvlDlyStep1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_dly_step_1(&mut self) -> PhyRdlvlDlyStep1W<DdrDenaliPhy218Spec> {
        PhyRdlvlDlyStep1W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_218::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_218::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy218Spec;
impl crate::RegisterSpec for DdrDenaliPhy218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_218::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy218Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_218::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy218Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_218 to value 0"]
impl crate::Resettable for DdrDenaliPhy218Spec {
    const RESET_VALUE: u32 = 0;
}
