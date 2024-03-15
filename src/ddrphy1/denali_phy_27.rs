#[doc = "Register `DENALI_PHY_27` reader"]
pub type R = crate::R<DenaliPhy27Spec>;
#[doc = "Register `DENALI_PHY_27` writer"]
pub type W = crate::W<DenaliPhy27Spec>;
#[doc = "Field `PHY_WDQLVL_DATADM_MASK_0` reader - Per-bit mask for write data leveling for slice 0. Set to 1 to mask any bit from the leveling process."]
pub type PhyWdqlvlDatadmMask0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DATADM_MASK_0` writer - Per-bit mask for write data leveling for slice 0. Set to 1 to mask any bit from the leveling process."]
pub type PhyWdqlvlDatadmMask0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Per-bit mask for write data leveling for slice 0. Set to 1 to mask any bit from the leveling process."]
    #[inline(always)]
    pub fn phy_wdqlvl_datadm_mask_0(&self) -> PhyWdqlvlDatadmMask0R {
        PhyWdqlvlDatadmMask0R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Per-bit mask for write data leveling for slice 0. Set to 1 to mask any bit from the leveling process."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_datadm_mask_0(&mut self) -> PhyWdqlvlDatadmMask0W<DenaliPhy27Spec> {
        PhyWdqlvlDatadmMask0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy27Spec;
impl crate::RegisterSpec for DenaliPhy27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_27::R`](R) reader structure"]
impl crate::Readable for DenaliPhy27Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_27::W`](W) writer structure"]
impl crate::Writable for DenaliPhy27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_27 to value 0"]
impl crate::Resettable for DenaliPhy27Spec {
    const RESET_VALUE: u32 = 0;
}
