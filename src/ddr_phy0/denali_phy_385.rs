#[doc = "Register `DENALI_PHY_385` reader"]
pub type R = crate::R<DenaliPhy385Spec>;
#[doc = "Register `DENALI_PHY_385` writer"]
pub type W = crate::W<DenaliPhy385Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_3` reader - DQ/DM bit swizzling 1 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_3R = crate::FieldReader;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_3` writer - DQ/DM bit swizzling 1 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_3` reader - Write data clock bypass mode slave delay setting for slice 3."]
pub type PhyClkWrBypassSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_3` writer - Write data clock bypass mode slave delay setting for slice 3."]
pub type PhyClkWrBypassSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle1_3(&self) -> PhyDqDmSwizzle1_3R {
        PhyDqDmSwizzle1_3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wr_bypass_slave_delay_3(&self) -> PhyClkWrBypassSlaveDelay3R {
        PhyClkWrBypassSlaveDelay3R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQ/DM bit swizzling 1 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle1_3(&mut self) -> PhyDqDmSwizzle1_3W<DenaliPhy385Spec> {
        PhyDqDmSwizzle1_3W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Write data clock bypass mode slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wr_bypass_slave_delay_3(
        &mut self,
    ) -> PhyClkWrBypassSlaveDelay3W<DenaliPhy385Spec> {
        PhyClkWrBypassSlaveDelay3W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_385::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_385::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy385Spec;
impl crate::RegisterSpec for DenaliPhy385Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_385::R`](R) reader structure"]
impl crate::Readable for DenaliPhy385Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_385::W`](W) writer structure"]
impl crate::Writable for DenaliPhy385Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_385 to value 0"]
impl crate::Resettable for DenaliPhy385Spec {
    const RESET_VALUE: u32 = 0;
}
