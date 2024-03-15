#[doc = "Register `DENALI_PHY_433` reader"]
pub type R = crate::R<DenaliPhy433Spec>;
#[doc = "Register `DENALI_PHY_433` writer"]
pub type W = crate::W<DenaliPhy433Spec>;
#[doc = "Field `PHY_DDL_MODE_3` reader - DDL mode for slice 3."]
pub type PhyDdlMode3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DDL_MODE_3` writer - DDL mode for slice 3."]
pub type PhyDdlMode3W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - DDL mode for slice 3."]
    #[inline(always)]
    pub fn phy_ddl_mode_3(&self) -> PhyDdlMode3R {
        PhyDdlMode3R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - DDL mode for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_mode_3(&mut self) -> PhyDdlMode3W<DenaliPhy433Spec> {
        PhyDdlMode3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_433::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_433::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy433Spec;
impl crate::RegisterSpec for DenaliPhy433Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_433::R`](R) reader structure"]
impl crate::Readable for DenaliPhy433Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_433::W`](W) writer structure"]
impl crate::Writable for DenaliPhy433Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_433 to value 0"]
impl crate::Resettable for DenaliPhy433Spec {
    const RESET_VALUE: u32 = 0;
}
