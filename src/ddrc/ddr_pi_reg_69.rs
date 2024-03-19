#[doc = "Register `DDR_PI_REG_69` reader"]
pub type R = crate::R<DdrPiReg69Spec>;
#[doc = "Register `DDR_PI_REG_69` writer"]
pub type W = crate::W<DdrPiReg69Spec>;
#[doc = "Field `PI_ODT_WR_MAP_CS0` reader - Determines the chip(s) that have termination when a write occurs\n\non chip select 0. Set bit X to enable termination on csX when cs0 is\n\nperforming a write."]
pub type PiOdtWrMapCs0R = crate::FieldReader;
#[doc = "Field `PI_ODT_WR_MAP_CS0` writer - Determines the chip(s) that have termination when a write occurs\n\non chip select 0. Set bit X to enable termination on csX when cs0 is\n\nperforming a write."]
pub type PiOdtWrMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ODT_RD_MAP_CS1` reader - Determines the chip(s) that have termination when a read occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a read."]
pub type PiOdtRdMapCs1R = crate::FieldReader;
#[doc = "Field `PI_ODT_RD_MAP_CS1` writer - Determines the chip(s) that have termination when a read occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a read."]
pub type PiOdtRdMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ODT_WR_MAP_CS1` reader - Determines the chip(s) that have termination when a write occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a write."]
pub type PiOdtWrMapCs1R = crate::FieldReader;
#[doc = "Field `PI_ODT_WR_MAP_CS1` writer - Determines the chip(s) that have termination when a write occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a write."]
pub type PiOdtWrMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Determines the chip(s) that have termination when a write occurs\n\non chip select 0. Set bit X to enable termination on csX when cs0 is\n\nperforming a write."]
    #[inline(always)]
    pub fn pi_odt_wr_map_cs0(&self) -> PiOdtWrMapCs0R {
        PiOdtWrMapCs0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Determines the chip(s) that have termination when a read occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a read."]
    #[inline(always)]
    pub fn pi_odt_rd_map_cs1(&self) -> PiOdtRdMapCs1R {
        PiOdtRdMapCs1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Determines the chip(s) that have termination when a write occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a write."]
    #[inline(always)]
    pub fn pi_odt_wr_map_cs1(&self) -> PiOdtWrMapCs1R {
        PiOdtWrMapCs1R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the chip(s) that have termination when a write occurs\n\non chip select 0. Set bit X to enable termination on csX when cs0 is\n\nperforming a write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_wr_map_cs0(&mut self) -> PiOdtWrMapCs0W<DdrPiReg69Spec> {
        PiOdtWrMapCs0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Determines the chip(s) that have termination when a read occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a read."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_rd_map_cs1(&mut self) -> PiOdtRdMapCs1W<DdrPiReg69Spec> {
        PiOdtRdMapCs1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Determines the chip(s) that have termination when a write occurs\n\non chip select 1. Set bit X to enable termination on csX when cs1 is\n\nperforming a write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_wr_map_cs1(&mut self) -> PiOdtWrMapCs1W<DdrPiReg69Spec> {
        PiOdtWrMapCs1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_69::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_69::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg69Spec;
impl crate::RegisterSpec for DdrPiReg69Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_69::R`](R) reader structure"]
impl crate::Readable for DdrPiReg69Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_69::W`](W) writer structure"]
impl crate::Writable for DdrPiReg69Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_69 to value 0"]
impl crate::Resettable for DdrPiReg69Spec {
    const RESET_VALUE: u32 = 0;
}
