#[doc = "Register `GRF_EMMCPHY_STATUS` reader"]
pub type R = crate::R<GrfEmmcphyStatusSpec>;
#[doc = "Register `GRF_EMMCPHY_STATUS` writer"]
pub type W = crate::W<GrfEmmcphyStatusSpec>;
#[doc = "Field `EMMCPHY_STATUS` reader - emmc phy status register"]
pub type EmmcphyStatusR = crate::FieldReader<u32>;
#[doc = "Field `EMMCPHY_STATUS` writer - emmc phy status register"]
pub type EmmcphyStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - emmc phy status register"]
    #[inline(always)]
    pub fn emmcphy_status(&self) -> EmmcphyStatusR {
        EmmcphyStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - emmc phy status register"]
    #[inline(always)]
    #[must_use]
    pub fn emmcphy_status(&mut self) -> EmmcphyStatusW<GrfEmmcphyStatusSpec> {
        EmmcphyStatusW::new(self, 0)
    }
}
#[doc = "emmc phy status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfEmmcphyStatusSpec;
impl crate::RegisterSpec for GrfEmmcphyStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_emmcphy_status::R`](R) reader structure"]
impl crate::Readable for GrfEmmcphyStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_emmcphy_status::W`](W) writer structure"]
impl crate::Writable for GrfEmmcphyStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_EMMCPHY_STATUS to value 0"]
impl crate::Resettable for GrfEmmcphyStatusSpec {
    const RESET_VALUE: u32 = 0;
}
