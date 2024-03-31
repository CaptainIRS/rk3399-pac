#[doc = "Register `DENALI_PHY_268` reader"]
pub type R = crate::R<DenaliPhy268Spec>;
#[doc = "Register `DENALI_PHY_268` writer"]
pub type W = crate::W<DenaliPhy268Spec>;
#[doc = "Field `PHY_LPDDR_2` reader - Indicates a cycle of delay for the slice 2 to match the address slice."]
pub type PhyLpddr2R = crate::BitReader;
#[doc = "Field `PHY_LPDDR_2` writer - Indicates a cycle of delay for the slice 2 to match the address slice."]
pub type PhyLpddr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LPDDR_TYPE_2` reader - Indicates the type of DRAM for slice 2. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType2R = crate::FieldReader;
#[doc = "Field `PHY_LPDDR_TYPE_2` writer - Indicates the type of DRAM for slice 2. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_2` reader - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 2."]
pub type PhyGateSmpl1SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_2` writer - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 2."]
pub type PhyGateSmpl1SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 2 to match the address slice."]
    #[inline(always)]
    pub fn phy_lpddr_2(&self) -> PhyLpddr2R {
        PhyLpddr2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 2. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    pub fn phy_lpddr_type_2(&self) -> PhyLpddrType2R {
        PhyLpddrType2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 2."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slave_delay_2(&self) -> PhyGateSmpl1SlaveDelay2R {
        PhyGateSmpl1SlaveDelay2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 2 to match the address slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_2(&mut self) -> PhyLpddr2W<DenaliPhy268Spec> {
        PhyLpddr2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 2. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_type_2(&mut self) -> PhyLpddrType2W<DenaliPhy268Spec> {
        PhyLpddrType2W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl1_slave_delay_2(&mut self) -> PhyGateSmpl1SlaveDelay2W<DenaliPhy268Spec> {
        PhyGateSmpl1SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_268::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_268::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy268Spec;
impl crate::RegisterSpec for DenaliPhy268Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_268::R`](R) reader structure"]
impl crate::Readable for DenaliPhy268Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_268::W`](W) writer structure"]
impl crate::Writable for DenaliPhy268Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_268 to value 0"]
impl crate::Resettable for DenaliPhy268Spec {
    const RESET_VALUE: u32 = 0;
}
