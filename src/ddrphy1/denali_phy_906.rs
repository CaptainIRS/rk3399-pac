#[doc = "Register `DENALI_PHY_906` reader"]
pub type R = crate::R<DenaliPhy906Spec>;
#[doc = "Register `DENALI_PHY_906` writer"]
pub type W = crate::W<DenaliPhy906Spec>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS` reader - Observation register for all address/control group slice slave delay encoded values. READ- ONLY"]
pub type PhyGrpSlvDlyEncObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SHIFT_OBS` reader - Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
pub type PhyGrpShiftObsR = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SLAVE_LOOP_CNT_UPDATE` reader - Sets the frequency by which the slave delay encoded value holding registers are updated for the address/control master."]
pub type PhyAdrctlSlaveLoopCntUpdateR = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SLAVE_LOOP_CNT_UPDATE` writer - Sets the frequency by which the slave delay encoded value holding registers are updated for the address/control master."]
pub type PhyAdrctlSlaveLoopCntUpdateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Observation register for all address/control group slice slave delay encoded values. READ- ONLY"]
    #[inline(always)]
    pub fn phy_grp_slv_dly_enc_obs(&self) -> PhyGrpSlvDlyEncObsR {
        PhyGrpSlvDlyEncObsR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - Observation register for the address/control group automatic half cycle and cycle shift values. READ-ONLY"]
    #[inline(always)]
    pub fn phy_grp_shift_obs(&self) -> PhyGrpShiftObsR {
        PhyGrpShiftObsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Sets the frequency by which the slave delay encoded value holding registers are updated for the address/control master."]
    #[inline(always)]
    pub fn phy_adrctl_slave_loop_cnt_update(&self) -> PhyAdrctlSlaveLoopCntUpdateR {
        PhyAdrctlSlaveLoopCntUpdateR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Sets the frequency by which the slave delay encoded value holding registers are updated for the address/control master."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_slave_loop_cnt_update(
        &mut self,
    ) -> PhyAdrctlSlaveLoopCntUpdateW<DenaliPhy906Spec> {
        PhyAdrctlSlaveLoopCntUpdateW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_906::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_906::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy906Spec;
impl crate::RegisterSpec for DenaliPhy906Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_906::R`](R) reader structure"]
impl crate::Readable for DenaliPhy906Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_906::W`](W) writer structure"]
impl crate::Writable for DenaliPhy906Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_906 to value 0"]
impl crate::Resettable for DenaliPhy906Spec {
    const RESET_VALUE: u32 = 0;
}
