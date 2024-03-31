#[doc = "Register `EMMCCORE_STATUS3` reader"]
pub type R = crate::R<EmmccoreStatus3Spec>;
#[doc = "Register `EMMCCORE_STATUS3` writer"]
pub type W = crate::W<EmmccoreStatus3Spec>;
#[doc = "Field `EMMCCORE_STATUS3` reader - emmc controller status register 3"]
pub type EmmccoreStatus3R = crate::FieldReader<u32>;
#[doc = "Field `EMMCCORE_STATUS3` writer - emmc controller status register 3"]
pub type EmmccoreStatus3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - emmc controller status register 3"]
    #[inline(always)]
    pub fn emmccore_status3(&self) -> EmmccoreStatus3R {
        EmmccoreStatus3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - emmc controller status register 3"]
    #[inline(always)]
    #[must_use]
    pub fn emmccore_status3(&mut self) -> EmmccoreStatus3W<EmmccoreStatus3Spec> {
        EmmccoreStatus3W::new(self, 0)
    }
}
#[doc = "emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreStatus3Spec;
impl crate::RegisterSpec for EmmccoreStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_status3::R`](R) reader structure"]
impl crate::Readable for EmmccoreStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_status3::W`](W) writer structure"]
impl crate::Writable for EmmccoreStatus3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_STATUS3 to value 0"]
impl crate::Resettable for EmmccoreStatus3Spec {
    const RESET_VALUE: u32 = 0;
}
