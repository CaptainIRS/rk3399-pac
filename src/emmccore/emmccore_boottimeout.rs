#[doc = "Register `EMMCCORE_BOOTTIMEOUT` reader"]
pub type R = crate::R<EmmccoreBoottimeoutSpec>;
#[doc = "Register `EMMCCORE_BOOTTIMEOUT` writer"]
pub type W = crate::W<EmmccoreBoottimeoutSpec>;
#[doc = "Field `BOOTTIMEOUT` reader - Boot Data Timeout Counter Value This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
pub type BoottimeoutR = crate::FieldReader<u32>;
#[doc = "Field `BOOTTIMEOUT` writer - Boot Data Timeout Counter Value This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
pub type BoottimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
    #[inline(always)]
    pub fn boottimeout(&self) -> BoottimeoutR {
        BoottimeoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
    #[inline(always)]
    #[must_use]
    pub fn boottimeout(&mut self) -> BoottimeoutW<EmmccoreBoottimeoutSpec> {
        BoottimeoutW::new(self, 0)
    }
}
#[doc = "Boot timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_boottimeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_boottimeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreBoottimeoutSpec;
impl crate::RegisterSpec for EmmccoreBoottimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_boottimeout::R`](R) reader structure"]
impl crate::Readable for EmmccoreBoottimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_boottimeout::W`](W) writer structure"]
impl crate::Writable for EmmccoreBoottimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_BOOTTIMEOUT to value 0"]
impl crate::Resettable for EmmccoreBoottimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
