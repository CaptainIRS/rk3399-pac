#[doc = "Register `DPCC_PG_FAC_3` reader"]
pub type R = crate::R<DpccPgFac3Spec>;
#[doc = "Register `DPCC_PG_FAC_3` writer"]
pub type W = crate::W<DpccPgFac3Spec>;
#[doc = "Field `PG_FAC_3_G` reader - green"]
pub type PgFac3GR = crate::FieldReader;
#[doc = "Field `PG_FAC_3_G` writer - green"]
pub type PgFac3GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PG_FAC_3_RB` reader - red/blue"]
pub type PgFac3RbR = crate::FieldReader;
#[doc = "Field `PG_FAC_3_RB` writer - red/blue"]
pub type PgFac3RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    pub fn pg_fac_3_g(&self) -> PgFac3GR {
        PgFac3GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    pub fn pg_fac_3_rb(&self) -> PgFac3RbR {
        PgFac3RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    #[must_use]
    pub fn pg_fac_3_g(&mut self) -> PgFac3GW<DpccPgFac3Spec> {
        PgFac3GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn pg_fac_3_rb(&mut self) -> PgFac3RbW<DpccPgFac3Spec> {
        PgFac3RbW::new(self, 8)
    }
}
#[doc = "Peak gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_pg_fac_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_pg_fac_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccPgFac3Spec;
impl crate::RegisterSpec for DpccPgFac3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_pg_fac_3::R`](R) reader structure"]
impl crate::Readable for DpccPgFac3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_pg_fac_3::W`](W) writer structure"]
impl crate::Writable for DpccPgFac3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_PG_FAC_3 to value 0"]
impl crate::Resettable for DpccPgFac3Spec {
    const RESET_VALUE: u32 = 0;
}
