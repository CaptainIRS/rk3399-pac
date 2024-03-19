#[doc = "Register `DDR_DENALI_PHY_01` reader"]
pub type R = crate::R<DdrDenaliPhy01Spec>;
#[doc = "Register `DDR_DENALI_PHY_01` writer"]
pub type W = crate::W<DdrDenaliPhy01Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_0` reader - DQ/DM bit swizzling 1 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_0` writer - DQ/DM bit swizzling 1 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_0` reader - Write data clock bypass mode slave delay setting for slice 0."]
pub type PhyClkWrBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_0` writer - Write data clock bypass mode slave delay setting for slice 0."]
pub type PhyClkWrBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle1_0(&self) -> PhyDqDmSwizzle1_0R {
        PhyDqDmSwizzle1_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wr_bypass_slave_delay_0(&self) -> PhyClkWrBypassSlaveDelay0R {
        PhyClkWrBypassSlaveDelay0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle1_0(&mut self) -> PhyDqDmSwizzle1_0W<DdrDenaliPhy01Spec> {
        PhyDqDmSwizzle1_0W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wr_bypass_slave_delay_0(
        &mut self,
    ) -> PhyClkWrBypassSlaveDelay0W<DdrDenaliPhy01Spec> {
        PhyClkWrBypassSlaveDelay0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy01Spec;
impl crate::RegisterSpec for DdrDenaliPhy01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_01::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy01Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_01::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_01 to value 0"]
impl crate::Resettable for DdrDenaliPhy01Spec {
    const RESET_VALUE: u32 = 0;
}
