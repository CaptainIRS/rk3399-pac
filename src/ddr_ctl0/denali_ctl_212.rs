#[doc = "Register `DENALI_CTL_212` reader"]
pub type R = crate::R<DenaliCtl212Spec>;
#[doc = "Register `DENALI_CTL_212` writer"]
pub type W = crate::W<DenaliCtl212Spec>;
#[doc = "Field `ODT_WR_MAP_CS1` reader - Determines which chip(s) will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
pub type OdtWrMapCs1R = crate::FieldReader;
#[doc = "Field `ODT_WR_MAP_CS1` writer - Determines which chip(s) will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
pub type OdtWrMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TODTL_2CMD_F0` reader - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF0R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F0` writer - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TODTL_2CMD_F1` reader - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF1R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F1` writer - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TODTL_2CMD_F2` reader - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF2R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F2` writer - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
pub type Todtl2cmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Determines which chip(s) will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
    #[inline(always)]
    pub fn odt_wr_map_cs1(&self) -> OdtWrMapCs1R {
        OdtWrMapCs1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    pub fn todtl_2cmd_f0(&self) -> Todtl2cmdF0R {
        Todtl2cmdF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    pub fn todtl_2cmd_f1(&self) -> Todtl2cmdF1R {
        Todtl2cmdF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    pub fn todtl_2cmd_f2(&self) -> Todtl2cmdF2R {
        Todtl2cmdF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines which chip(s) will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
    #[inline(always)]
    #[must_use]
    pub fn odt_wr_map_cs1(&mut self) -> OdtWrMapCs1W<DenaliCtl212Spec> {
        OdtWrMapCs1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f0(&mut self) -> Todtl2cmdF0W<DenaliCtl212Spec> {
        Todtl2cmdF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f1(&mut self) -> Todtl2cmdF1W<DenaliCtl212Spec> {
        Todtl2cmdF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next non- write, non-read command."]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f2(&mut self) -> Todtl2cmdF2W<DenaliCtl212Spec> {
        Todtl2cmdF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_212::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_212::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl212Spec;
impl crate::RegisterSpec for DenaliCtl212Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_212::R`](R) reader structure"]
impl crate::Readable for DenaliCtl212Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_212::W`](W) writer structure"]
impl crate::Writable for DenaliCtl212Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_212 to value 0"]
impl crate::Resettable for DenaliCtl212Spec {
    const RESET_VALUE: u32 = 0;
}
