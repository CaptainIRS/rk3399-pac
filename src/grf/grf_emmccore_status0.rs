#[doc = "Register `GRF_EMMCCORE_STATUS0` reader"]
pub type R = crate::R<GrfEmmccoreStatus0Spec>;
#[doc = "Register `GRF_EMMCCORE_STATUS0` writer"]
pub type W = crate::W<GrfEmmccoreStatus0Spec>;
#[doc = "Field `EMMCCORE_STATUS0` reader - emmc controller status register 0"]
pub type EmmccoreStatus0R = crate::FieldReader<u32>;
#[doc = "Field `EMMCCORE_STATUS0` writer - emmc controller status register 0"]
pub type EmmccoreStatus0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - emmc controller status register 0"]
    #[inline(always)]
    pub fn emmccore_status0(&self) -> EmmccoreStatus0R {
        EmmccoreStatus0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - emmc controller status register 0"]
    #[inline(always)]
    #[must_use]
    pub fn emmccore_status0(&mut self) -> EmmccoreStatus0W<GrfEmmccoreStatus0Spec> {
        EmmccoreStatus0W::new(self, 0)
    }
}
#[doc = "emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfEmmccoreStatus0Spec;
impl crate::RegisterSpec for GrfEmmccoreStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_emmccore_status0::R`](R) reader structure"]
impl crate::Readable for GrfEmmccoreStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_emmccore_status0::W`](W) writer structure"]
impl crate::Writable for GrfEmmccoreStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_EMMCCORE_STATUS0 to value 0"]
impl crate::Resettable for GrfEmmccoreStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
