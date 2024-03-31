#[doc = "Register `EMMCCORE_STATUS2` reader"]
pub type R = crate::R<EmmccoreStatus2Spec>;
#[doc = "Register `EMMCCORE_STATUS2` writer"]
pub type W = crate::W<EmmccoreStatus2Spec>;
#[doc = "Field `EMMCCORE_STATUS2` reader - emmc controller status register 2"]
pub type EmmccoreStatus2R = crate::FieldReader<u32>;
#[doc = "Field `EMMCCORE_STATUS2` writer - emmc controller status register 2"]
pub type EmmccoreStatus2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - emmc controller status register 2"]
    #[inline(always)]
    pub fn emmccore_status2(&self) -> EmmccoreStatus2R {
        EmmccoreStatus2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - emmc controller status register 2"]
    #[inline(always)]
    #[must_use]
    pub fn emmccore_status2(&mut self) -> EmmccoreStatus2W<EmmccoreStatus2Spec> {
        EmmccoreStatus2W::new(self, 0)
    }
}
#[doc = "emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreStatus2Spec;
impl crate::RegisterSpec for EmmccoreStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_status2::R`](R) reader structure"]
impl crate::Readable for EmmccoreStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_status2::W`](W) writer structure"]
impl crate::Writable for EmmccoreStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_STATUS2 to value 0"]
impl crate::Resettable for EmmccoreStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
