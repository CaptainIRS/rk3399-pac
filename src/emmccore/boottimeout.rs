#[doc = "Register `BOOTTIMEOUT` reader"]
pub type R = crate::R<BoottimeoutSpec>;
#[doc = "Register `BOOTTIMEOUT` writer"]
pub type W = crate::W<BoottimeoutSpec>;
#[doc = "Field `BOOTTIMEOUT` reader - Boot Data Timeout Counter Value\n\nThis value determines the interval by which DAT line time-outs\n\nare detected during boot operation for eMMC card.\n\nThe value is in number of sd clock."]
pub type BoottimeoutR = crate::FieldReader<u32>;
#[doc = "Field `BOOTTIMEOUT` writer - Boot Data Timeout Counter Value\n\nThis value determines the interval by which DAT line time-outs\n\nare detected during boot operation for eMMC card.\n\nThe value is in number of sd clock."]
pub type BoottimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value\n\nThis value determines the interval by which DAT line time-outs\n\nare detected during boot operation for eMMC card.\n\nThe value is in number of sd clock."]
    #[inline(always)]
    pub fn boottimeout(&self) -> BoottimeoutR {
        BoottimeoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value\n\nThis value determines the interval by which DAT line time-outs\n\nare detected during boot operation for eMMC card.\n\nThe value is in number of sd clock."]
    #[inline(always)]
    #[must_use]
    pub fn boottimeout(&mut self) -> BoottimeoutW<BoottimeoutSpec> {
        BoottimeoutW::new(self, 0)
    }
}
#[doc = "Boot timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boottimeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boottimeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoottimeoutSpec;
impl crate::RegisterSpec for BoottimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boottimeout::R`](R) reader structure"]
impl crate::Readable for BoottimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`boottimeout::W`](W) writer structure"]
impl crate::Writable for BoottimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTTIMEOUT to value 0"]
impl crate::Resettable for BoottimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
