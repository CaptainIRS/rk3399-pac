#[doc = "Register `DENALI_PHY_256` reader"]
pub type R = crate::R<DenaliPhy256Spec>;
#[doc = "Register `DENALI_PHY_256` writer"]
pub type W = crate::W<DenaliPhy256Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_2` reader - DQ/DM bit swizzling 0 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_2` writer - DQ/DM bit swizzling 0 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle0_2(&self) -> PhyDqDmSwizzle0_2R {
        PhyDqDmSwizzle0_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 2. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle0_2(&mut self) -> PhyDqDmSwizzle0_2W<DenaliPhy256Spec> {
        PhyDqDmSwizzle0_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_256::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_256::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy256Spec;
impl crate::RegisterSpec for DenaliPhy256Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_256::R`](R) reader structure"]
impl crate::Readable for DenaliPhy256Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_256::W`](W) writer structure"]
impl crate::Writable for DenaliPhy256Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_256 to value 0"]
impl crate::Resettable for DenaliPhy256Spec {
    const RESET_VALUE: u32 = 0;
}
