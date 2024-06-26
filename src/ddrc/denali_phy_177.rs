#[doc = "Register `DENALI_PHY_177` reader"]
pub type R = crate::R<DenaliPhy177Spec>;
#[doc = "Register `DENALI_PHY_177` writer"]
pub type W = crate::W<DenaliPhy177Spec>;
#[doc = "Field `PHY_DDL_MODE_1` reader - DDL mode for slice 1."]
pub type PhyDdlMode1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DDL_MODE_1` writer - DDL mode for slice 1."]
pub type PhyDdlMode1W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - DDL mode for slice 1."]
    #[inline(always)]
    pub fn phy_ddl_mode_1(&self) -> PhyDdlMode1R {
        PhyDdlMode1R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - DDL mode for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_mode_1(&mut self) -> PhyDdlMode1W<DenaliPhy177Spec> {
        PhyDdlMode1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_177::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_177::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy177Spec;
impl crate::RegisterSpec for DenaliPhy177Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_177::R`](R) reader structure"]
impl crate::Readable for DenaliPhy177Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_177::W`](W) writer structure"]
impl crate::Writable for DenaliPhy177Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_177 to value 0"]
impl crate::Resettable for DenaliPhy177Spec {
    const RESET_VALUE: u32 = 0;
}
