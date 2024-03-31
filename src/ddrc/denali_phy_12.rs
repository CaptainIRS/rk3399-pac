#[doc = "Register `DENALI_PHY_12` reader"]
pub type R = crate::R<DenaliPhy12Spec>;
#[doc = "Register `DENALI_PHY_12` writer"]
pub type W = crate::W<DenaliPhy12Spec>;
#[doc = "Field `PHY_LPDDR_0` reader - Indicates a cycle of delay for the slice 0 to match the address slice."]
pub type PhyLpddr0R = crate::BitReader;
#[doc = "Field `PHY_LPDDR_0` writer - Indicates a cycle of delay for the slice 0 to match the address slice."]
pub type PhyLpddr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LPDDR_TYPE_0` reader - Indicates the type of DRAM for slice 0. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType0R = crate::FieldReader;
#[doc = "Field `PHY_LPDDR_TYPE_0` writer - Indicates the type of DRAM for slice 0. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_0` reader - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 0."]
pub type PhyGateSmpl1SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_0` writer - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 0."]
pub type PhyGateSmpl1SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 0 to match the address slice."]
    #[inline(always)]
    pub fn phy_lpddr_0(&self) -> PhyLpddr0R {
        PhyLpddr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 0. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    pub fn phy_lpddr_type_0(&self) -> PhyLpddrType0R {
        PhyLpddrType0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 0."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slave_delay_0(&self) -> PhyGateSmpl1SlaveDelay0R {
        PhyGateSmpl1SlaveDelay0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 0 to match the address slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_0(&mut self) -> PhyLpddr0W<DenaliPhy12Spec> {
        PhyLpddr0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 0. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_type_0(&mut self) -> PhyLpddrType0W<DenaliPhy12Spec> {
        PhyLpddrType0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl1_slave_delay_0(&mut self) -> PhyGateSmpl1SlaveDelay0W<DenaliPhy12Spec> {
        PhyGateSmpl1SlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy12Spec;
impl crate::RegisterSpec for DenaliPhy12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_12::R`](R) reader structure"]
impl crate::Readable for DenaliPhy12Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_12::W`](W) writer structure"]
impl crate::Writable for DenaliPhy12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_12 to value 0"]
impl crate::Resettable for DenaliPhy12Spec {
    const RESET_VALUE: u32 = 0;
}
