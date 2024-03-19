#[doc = "Register `DDR_DENALI_PHY_00` reader"]
pub type R = crate::R<DdrDenaliPhy00Spec>;
#[doc = "Register `DDR_DENALI_PHY_00` writer"]
pub type W = crate::W<DdrDenaliPhy00Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_0` reader - DQ/DM bit swizzling 0 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_0` writer - DQ/DM bit swizzling 0 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle0_0(&self) -> PhyDqDmSwizzle0_0R {
        PhyDqDmSwizzle0_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 0. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle0_0(&mut self) -> PhyDqDmSwizzle0_0W<DdrDenaliPhy00Spec> {
        PhyDqDmSwizzle0_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy00Spec;
impl crate::RegisterSpec for DdrDenaliPhy00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_00::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy00Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_00::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_00 to value 0"]
impl crate::Resettable for DdrDenaliPhy00Spec {
    const RESET_VALUE: u32 = 0;
}
