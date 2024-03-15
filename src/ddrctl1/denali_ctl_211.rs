#[doc = "Register `DENALI_CTL_211` reader"]
pub type R = crate::R<DenaliCtl211Spec>;
#[doc = "Register `DENALI_CTL_211` writer"]
pub type W = crate::W<DenaliCtl211Spec>;
#[doc = "Field `OUT_OF_RANGE_SOURCE_ID` reader - Source ID of command that caused an out-of-range interrupt. READ- ONLY"]
pub type OutOfRangeSourceIdR = crate::FieldReader;
#[doc = "Field `ODT_RD_MAP_CS0` reader - Determines which chip(s) will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type OdtRdMapCs0R = crate::FieldReader;
#[doc = "Field `ODT_RD_MAP_CS0` writer - Determines which chip(s) will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type OdtRdMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODT_WR_MAP_CS0` reader - Determines which chip(s) will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
pub type OdtWrMapCs0R = crate::FieldReader;
#[doc = "Field `ODT_WR_MAP_CS0` writer - Determines which chip(s) will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
pub type OdtWrMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODT_RD_MAP_CS1` reader - Determines which chip(s) will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
pub type OdtRdMapCs1R = crate::FieldReader;
#[doc = "Field `ODT_RD_MAP_CS1` writer - Determines which chip(s) will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
pub type OdtRdMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Source ID of command that caused an out-of-range interrupt. READ- ONLY"]
    #[inline(always)]
    pub fn out_of_range_source_id(&self) -> OutOfRangeSourceIdR {
        OutOfRangeSourceIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Determines which chip(s) will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    pub fn odt_rd_map_cs0(&self) -> OdtRdMapCs0R {
        OdtRdMapCs0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Determines which chip(s) will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
    #[inline(always)]
    pub fn odt_wr_map_cs0(&self) -> OdtWrMapCs0R {
        OdtWrMapCs0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Determines which chip(s) will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
    #[inline(always)]
    pub fn odt_rd_map_cs1(&self) -> OdtRdMapCs1R {
        OdtRdMapCs1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Determines which chip(s) will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    #[must_use]
    pub fn odt_rd_map_cs0(&mut self) -> OdtRdMapCs0W<DenaliCtl211Spec> {
        OdtRdMapCs0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Determines which chip(s) will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
    #[inline(always)]
    #[must_use]
    pub fn odt_wr_map_cs0(&mut self) -> OdtWrMapCs0W<DenaliCtl211Spec> {
        OdtWrMapCs0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Determines which chip(s) will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
    #[inline(always)]
    #[must_use]
    pub fn odt_rd_map_cs1(&mut self) -> OdtRdMapCs1W<DenaliCtl211Spec> {
        OdtRdMapCs1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_211::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_211::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl211Spec;
impl crate::RegisterSpec for DenaliCtl211Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_211::R`](R) reader structure"]
impl crate::Readable for DenaliCtl211Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_211::W`](W) writer structure"]
impl crate::Writable for DenaliCtl211Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_211 to value 0"]
impl crate::Resettable for DenaliCtl211Spec {
    const RESET_VALUE: u32 = 0;
}
