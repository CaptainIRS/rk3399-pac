#[doc = "Register `DPCC_PG_FAC_2` reader"]
pub type R = crate::R<DpccPgFac2Spec>;
#[doc = "Register `DPCC_PG_FAC_2` writer"]
pub type W = crate::W<DpccPgFac2Spec>;
#[doc = "Field `PG_FAC_2_G` reader - green"]
pub type PgFac2GR = crate::FieldReader;
#[doc = "Field `PG_FAC_2_G` writer - green"]
pub type PgFac2GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PG_FAC_2_RB` reader - red/blue"]
pub type PgFac2RbR = crate::FieldReader;
#[doc = "Field `PG_FAC_2_RB` writer - red/blue"]
pub type PgFac2RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    pub fn pg_fac_2_g(&self) -> PgFac2GR {
        PgFac2GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    pub fn pg_fac_2_rb(&self) -> PgFac2RbR {
        PgFac2RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    #[must_use]
    pub fn pg_fac_2_g(&mut self) -> PgFac2GW<DpccPgFac2Spec> {
        PgFac2GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn pg_fac_2_rb(&mut self) -> PgFac2RbW<DpccPgFac2Spec> {
        PgFac2RbW::new(self, 8)
    }
}
#[doc = "Peak gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_pg_fac_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_pg_fac_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccPgFac2Spec;
impl crate::RegisterSpec for DpccPgFac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_pg_fac_2::R`](R) reader structure"]
impl crate::Readable for DpccPgFac2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_pg_fac_2::W`](W) writer structure"]
impl crate::Writable for DpccPgFac2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_PG_FAC_2 to value 0"]
impl crate::Resettable for DpccPgFac2Spec {
    const RESET_VALUE: u32 = 0;
}
