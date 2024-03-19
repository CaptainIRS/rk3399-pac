#[doc = "Register `DDR_DENALI_PHY_337` reader"]
pub type R = crate::R<DdrDenaliPhy337Spec>;
#[doc = "Register `DDR_DENALI_PHY_337` writer"]
pub type W = crate::W<DdrDenaliPhy337Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_2` reader - Initial DQ/DM slave delay setting during write data leveling for slice 2."]
pub type PhyWdqlvlDqdmSlvDlyStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_2` writer - Initial DQ/DM slave delay setting during write data leveling for slice 2."]
pub type PhyWdqlvlDqdmSlvDlyStart2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_2` reader - Read leveling starting value for the DQS/DQ slave delay settings for slice 2."]
pub type PhyRdlvlRddqsDqSlvDlyStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_2` writer - Read leveling starting value for the DQS/DQ slave delay settings for slice 2."]
pub type PhyRdlvlRddqsDqSlvDlyStart2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_2(&self) -> PhyWdqlvlDqdmSlvDlyStart2R {
        PhyWdqlvlDqdmSlvDlyStart2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 2."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_2(&self) -> PhyRdlvlRddqsDqSlvDlyStart2R {
        PhyRdlvlRddqsDqSlvDlyStart2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Initial DQ/DM slave delay setting during write data leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_2(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyStart2W<DdrDenaliPhy337Spec> {
        PhyWdqlvlDqdmSlvDlyStart2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read leveling starting value for the DQS/DQ slave delay settings for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_2(
        &mut self,
    ) -> PhyRdlvlRddqsDqSlvDlyStart2W<DdrDenaliPhy337Spec> {
        PhyRdlvlRddqsDqSlvDlyStart2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_337::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_337::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy337Spec;
impl crate::RegisterSpec for DdrDenaliPhy337Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_337::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy337Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_337::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy337Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_337 to value 0"]
impl crate::Resettable for DdrDenaliPhy337Spec {
    const RESET_VALUE: u32 = 0;
}
