#[doc = "Register `SWREG_81` reader"]
pub type R = crate::R<Swreg81Spec>;
#[doc = "Register `SWREG_81` writer"]
pub type W = crate::W<Swreg81Spec>;
#[doc = "Field `CABAC_TABLE_ST_ADR` reader - the cabac table start address\n\nH264: cabac table"]
pub type CabacTableStAdrR = crate::FieldReader<u32>;
#[doc = "Field `CABAC_TABLE_ST_ADR` writer - the cabac table start address\n\nH264: cabac table"]
pub type CabacTableStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the cabac table start address\n\nH264: cabac table"]
    #[inline(always)]
    pub fn cabac_table_st_adr(&self) -> CabacTableStAdrR {
        CabacTableStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the cabac table start address\n\nH264: cabac table"]
    #[inline(always)]
    #[must_use]
    pub fn cabac_table_st_adr(&mut self) -> CabacTableStAdrW<Swreg81Spec> {
        CabacTableStAdrW::new(self, 0)
    }
}
#[doc = "the cabac table start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg81Spec;
impl crate::RegisterSpec for Swreg81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_81::R`](R) reader structure"]
impl crate::Readable for Swreg81Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_81::W`](W) writer structure"]
impl crate::Writable for Swreg81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_81 to value 0"]
impl crate::Resettable for Swreg81Spec {
    const RESET_VALUE: u32 = 0;
}
