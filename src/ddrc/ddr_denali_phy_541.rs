#[doc = "Register `DDR_DENALI_PHY_541` reader"]
pub type R = crate::R<DdrDenaliPhy541Spec>;
#[doc = "Register `DDR_DENALI_PHY_541` writer"]
pub type W = crate::W<DdrDenaliPhy541Spec>;
#[doc = "Field `PHY_ADR_ADDR_SEL_0` reader - Mux select to map in LPDDR4 addressing for address slice 0."]
pub type PhyAdrAddrSel0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_ADDR_SEL_0` writer - Mux select to map in LPDDR4 addressing for address slice 0."]
pub type PhyAdrAddrSel0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_addr_sel_0(&self) -> PhyAdrAddrSel0R {
        PhyAdrAddrSel0R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_addr_sel_0(&mut self) -> PhyAdrAddrSel0W<DdrDenaliPhy541Spec> {
        PhyAdrAddrSel0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_541::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_541::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy541Spec;
impl crate::RegisterSpec for DdrDenaliPhy541Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_541::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy541Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_541::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy541Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_541 to value 0"]
impl crate::Resettable for DdrDenaliPhy541Spec {
    const RESET_VALUE: u32 = 0;
}
