#[doc = "Register `DENALI_PHY_465` reader"]
pub type R = crate::R<DenaliPhy465Spec>;
#[doc = "Register `DENALI_PHY_465` writer"]
pub type W = crate::W<DenaliPhy465Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_3` reader - Initial DQ/DM slave delay setting during write data leveling for slice 3."]
pub type PhyWdqlvlDqdmSlvDlyStart3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_3` writer - Initial DQ/DM slave delay setting during write data leveling for slice 3."]
pub type PhyWdqlvlDqdmSlvDlyStart3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_3` reader - Read leveling starting value for the DQS/DQ slave delay settings for slice 3."]
pub type PhyRdlvlRddqsDqSlvDlyStart3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_3` writer - Read leveling starting value for the DQS/DQ slave delay settings for slice 3."]
pub type PhyRdlvlRddqsDqSlvDlyStart3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_3(&self) -> PhyWdqlvlDqdmSlvDlyStart3R {
        PhyWdqlvlDqdmSlvDlyStart3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 3."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_3(&self) -> PhyRdlvlRddqsDqSlvDlyStart3R {
        PhyRdlvlRddqsDqSlvDlyStart3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_3(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyStart3W<DenaliPhy465Spec> {
        PhyWdqlvlDqdmSlvDlyStart3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_3(
        &mut self,
    ) -> PhyRdlvlRddqsDqSlvDlyStart3W<DenaliPhy465Spec> {
        PhyRdlvlRddqsDqSlvDlyStart3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_465::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_465::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy465Spec;
impl crate::RegisterSpec for DenaliPhy465Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_465::R`](R) reader structure"]
impl crate::Readable for DenaliPhy465Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_465::W`](W) writer structure"]
impl crate::Writable for DenaliPhy465Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_465 to value 0"]
impl crate::Resettable for DenaliPhy465Spec {
    const RESET_VALUE: u32 = 0;
}
