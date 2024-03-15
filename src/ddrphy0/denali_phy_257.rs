#[doc = "Register `DENALI_PHY_257` reader"]
pub type R = crate::R<DenaliPhy257Spec>;
#[doc = "Register `DENALI_PHY_257` writer"]
pub type W = crate::W<DenaliPhy257Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_2` reader - DQ/DM bit swizzling 1 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_2` writer - DQ/DM bit swizzling 1 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_2` reader - Write data clock bypass mode slave delay setting for slice 2."]
pub type PhyClkWrBypassSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_2` writer - Write data clock bypass mode slave delay setting for slice 2."]
pub type PhyClkWrBypassSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle1_2(&self) -> PhyDqDmSwizzle1_2R {
        PhyDqDmSwizzle1_2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wr_bypass_slave_delay_2(&self) -> PhyClkWrBypassSlaveDelay2R {
        PhyClkWrBypassSlaveDelay2R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle1_2(&mut self) -> PhyDqDmSwizzle1_2W<DenaliPhy257Spec> {
        PhyDqDmSwizzle1_2W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wr_bypass_slave_delay_2(
        &mut self,
    ) -> PhyClkWrBypassSlaveDelay2W<DenaliPhy257Spec> {
        PhyClkWrBypassSlaveDelay2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_257::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_257::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy257Spec;
impl crate::RegisterSpec for DenaliPhy257Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_257::R`](R) reader structure"]
impl crate::Readable for DenaliPhy257Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_257::W`](W) writer structure"]
impl crate::Writable for DenaliPhy257Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_257 to value 0"]
impl crate::Resettable for DenaliPhy257Spec {
    const RESET_VALUE: u32 = 0;
}
