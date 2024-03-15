#[doc = "Register `GRF_EMMCPHY_CON6` reader"]
pub type R = crate::R<GrfEmmcphyCon6Spec>;
#[doc = "Register `GRF_EMMCPHY_CON6` writer"]
pub type W = crate::W<GrfEmmcphyCon6Spec>;
#[doc = "Field `EMMCPHY_CON6` reader - emmc phy control register 5"]
pub type EmmcphyCon6R = crate::FieldReader<u16>;
#[doc = "Field `EMMCPHY_CON6` writer - emmc phy control register 5"]
pub type EmmcphyCon6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - emmc phy control register 5"]
    #[inline(always)]
    pub fn emmcphy_con6(&self) -> EmmcphyCon6R {
        EmmcphyCon6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - emmc phy control register 5"]
    #[inline(always)]
    #[must_use]
    pub fn emmcphy_con6(&mut self) -> EmmcphyCon6W<GrfEmmcphyCon6Spec> {
        EmmcphyCon6W::new(self, 0)
    }
}
#[doc = "emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfEmmcphyCon6Spec;
impl crate::RegisterSpec for GrfEmmcphyCon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_emmcphy_con6::R`](R) reader structure"]
impl crate::Readable for GrfEmmcphyCon6Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_emmcphy_con6::W`](W) writer structure"]
impl crate::Writable for GrfEmmcphyCon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_EMMCPHY_CON6 to value 0"]
impl crate::Resettable for GrfEmmcphyCon6Spec {
    const RESET_VALUE: u32 = 0;
}
