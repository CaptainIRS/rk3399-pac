#[doc = "Register `DDR_DENALI_PHY_384` reader"]
pub type R = crate::R<DdrDenaliPhy384Spec>;
#[doc = "Register `DDR_DENALI_PHY_384` writer"]
pub type W = crate::W<DdrDenaliPhy384Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_3` reader - DQ/DM bit swizzling 0 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE0_3` writer - DQ/DM bit swizzling 0 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
pub type PhyDqDmSwizzle0_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle0_3(&self) -> PhyDqDmSwizzle0_3R {
        PhyDqDmSwizzle0_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DQ/DM bit swizzling 0 for slice 3. Bits (3:0) inform the PHY which bit in {DM,DQ\\]} map to DQ0, Bits (7:4) inform the PHY which bit in {DM,DQ} map to DQ1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle0_3(&mut self) -> PhyDqDmSwizzle0_3W<DdrDenaliPhy384Spec> {
        PhyDqDmSwizzle0_3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_384::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_384::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy384Spec;
impl crate::RegisterSpec for DdrDenaliPhy384Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_384::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy384Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_384::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy384Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_384 to value 0"]
impl crate::Resettable for DdrDenaliPhy384Spec {
    const RESET_VALUE: u32 = 0;
}
