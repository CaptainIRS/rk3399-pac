#[doc = "Register `DENALI_PHY_776` reader"]
pub type R = crate::R<DenaliPhy776Spec>;
#[doc = "Register `DENALI_PHY_776` writer"]
pub type W = crate::W<DenaliPhy776Spec>;
#[doc = "Field `PHY_ADR_DDL_MODE_2` reader - DDL mode for address slice 2."]
pub type PhyAdrDdlMode2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_DDL_MODE_2` writer - DDL mode for address slice 2."]
pub type PhyAdrDdlMode2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - DDL mode for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_ddl_mode_2(&self) -> PhyAdrDdlMode2R {
        PhyAdrDdlMode2R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - DDL mode for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ddl_mode_2(&mut self) -> PhyAdrDdlMode2W<DenaliPhy776Spec> {
        PhyAdrDdlMode2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_776::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_776::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy776Spec;
impl crate::RegisterSpec for DenaliPhy776Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_776::R`](R) reader structure"]
impl crate::Readable for DenaliPhy776Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_776::W`](W) writer structure"]
impl crate::Writable for DenaliPhy776Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_776 to value 0"]
impl crate::Resettable for DenaliPhy776Spec {
    const RESET_VALUE: u32 = 0;
}
