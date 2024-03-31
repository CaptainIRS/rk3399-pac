#[doc = "Register `DENALI_PHY_140` reader"]
pub type R = crate::R<DenaliPhy140Spec>;
#[doc = "Register `DENALI_PHY_140` writer"]
pub type W = crate::W<DenaliPhy140Spec>;
#[doc = "Field `PHY_LPDDR_1` reader - Indicates a cycle of delay for the slice 1 to match the address slice."]
pub type PhyLpddr1R = crate::BitReader;
#[doc = "Field `PHY_LPDDR_1` writer - Indicates a cycle of delay for the slice 1 to match the address slice."]
pub type PhyLpddr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LPDDR_TYPE_1` reader - Indicates the type of DRAM for slice 1. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType1R = crate::FieldReader;
#[doc = "Field `PHY_LPDDR_TYPE_1` writer - Indicates the type of DRAM for slice 1. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
pub type PhyLpddrType1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_1` reader - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 1."]
pub type PhyGateSmpl1SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_1` writer - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 1."]
pub type PhyGateSmpl1SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 1 to match the address slice."]
    #[inline(always)]
    pub fn phy_lpddr_1(&self) -> PhyLpddr1R {
        PhyLpddr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 1. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    pub fn phy_lpddr_type_1(&self) -> PhyLpddrType1R {
        PhyLpddrType1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 1."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slave_delay_1(&self) -> PhyGateSmpl1SlaveDelay1R {
        PhyGateSmpl1SlaveDelay1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates a cycle of delay for the slice 1 to match the address slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_1(&mut self) -> PhyLpddr1W<DenaliPhy140Spec> {
        PhyLpddr1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of DRAM for slice 1. Clear to 0 for DDR3 or DDR4, program to 1 for LPDDR3, or program to 2 for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_type_1(&mut self) -> PhyLpddrType1W<DenaliPhy140Spec> {
        PhyLpddrType1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Number of cycles to delay the read DQS gate signal to generate gate1 signal for on the fly read DQS training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl1_slave_delay_1(&mut self) -> PhyGateSmpl1SlaveDelay1W<DenaliPhy140Spec> {
        PhyGateSmpl1SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy140Spec;
impl crate::RegisterSpec for DenaliPhy140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_140::R`](R) reader structure"]
impl crate::Readable for DenaliPhy140Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_140::W`](W) writer structure"]
impl crate::Writable for DenaliPhy140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_140 to value 0"]
impl crate::Resettable for DenaliPhy140Spec {
    const RESET_VALUE: u32 = 0;
}
