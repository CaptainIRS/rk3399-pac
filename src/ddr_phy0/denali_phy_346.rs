#[doc = "Register `DENALI_PHY_346` reader"]
pub type R = crate::R<DenaliPhy346Spec>;
#[doc = "Register `DENALI_PHY_346` writer"]
pub type W = crate::W<DenaliPhy346Spec>;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_2` reader - DQ slave delay step size during write data leveling for slice 2."]
pub type PhyWdqlvlDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_2` writer - DQ slave delay step size during write data leveling for slice 2."]
pub type PhyWdqlvlDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDLVL_DLY_STEP_2` reader - DQS slave delay step size during read leveling for slice 2."]
pub type PhyRdlvlDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DLY_STEP_2` writer - DQS slave delay step size during read leveling for slice 2."]
pub type PhyRdlvlDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_dly_step_2(&self) -> PhyWdqlvlDlyStep2R {
        PhyWdqlvlDlyStep2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_dly_step_2(&self) -> PhyRdlvlDlyStep2R {
        PhyRdlvlDlyStep2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQ slave delay step size during write data leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dly_step_2(&mut self) -> PhyWdqlvlDlyStep2W<DenaliPhy346Spec> {
        PhyWdqlvlDlyStep2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQS slave delay step size during read leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_dly_step_2(&mut self) -> PhyRdlvlDlyStep2W<DenaliPhy346Spec> {
        PhyRdlvlDlyStep2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_346::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_346::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy346Spec;
impl crate::RegisterSpec for DenaliPhy346Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_346::R`](R) reader structure"]
impl crate::Readable for DenaliPhy346Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_346::W`](W) writer structure"]
impl crate::Writable for DenaliPhy346Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_346 to value 0"]
impl crate::Resettable for DenaliPhy346Spec {
    const RESET_VALUE: u32 = 0;
}
